use std::sync::{Arc, Mutex};

use wasmtime::{component::*, Config, Error};
use wasmtime::{Result, Engine, Store};
use wasmtime_wasi::{IoView, WasiCtx, WasiView};

use async_executor::executor::Executor;
use async_executor::tokio_executor::TokioExecutor;

use wasmexample::infoxchange::status_holder::{self, Status};

bindgen!({
    world: "infoxchange",
    path: "./wit",
    async: true,
});

#[derive(Clone)]
struct CompContext {
    store: Arc<Mutex<Store<XchangeHost>>>,
    bindings: Arc<Infoxchange>,
}

impl CompContext {
    pub async fn new(wasm: &str) -> Result<Self, Error> {
        // setup our engine and linker to support an async context
        let mut config = Config::new();
        config.async_support(true);
        let engine = Engine::new(&config).unwrap();    
        let mut linker = Linker::<XchangeHost>::new(&engine);

        // add our host to the linker, so we can provide a world for our component
        wasmtime_wasi::add_to_linker_async(&mut linker)?;
        Infoxchange::add_to_linker(&mut linker, |s: &mut XchangeHost| s)?;
    
        // setup our store that holds the context our host uses with our component
        let mut store = Store::new(
            &engine,
            XchangeHost { 
                wasi_ctx: wasmtime_wasi::WasiCtxBuilder::new()
                    .inherit_stdout()
                    .build(),
                table: ResourceTable::new(),
                status: Status::Unknown
            },
        );

        // instantiate our component
        let component = Component::from_file(&engine, wasm).unwrap();
        let instance = linker.instantiate_async(&mut store, &component).await?;
        let bindings = Infoxchange::new(&mut store, &instance)?;

        Ok(Self {
            store: Arc::new(Mutex::new(store)),
            bindings: Arc::new(bindings),
        })
    }
}

struct XchangeHost {
    wasi_ctx: WasiCtx,
    table: ResourceTable,
    status: Status,
}

impl IoView for XchangeHost{
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }
}
impl WasiView for XchangeHost {
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.wasi_ctx
    }
}

impl status_holder::Host for XchangeHost {
    async fn set_status(&mut self,status: Status) -> Result<(),String> {
        println!("set_status() called with status: {:?}", status);
        self.status = status;
        Ok(())
    }

    async fn get_status(&mut self,) -> Result<Status,String> {
        Ok(self.status)
    }
}

async fn comp_worker<E: Executor>(executor: E, comp_ctx: CompContext) -> Result<()> {
    let comp = comp_ctx.bindings.wasmexample_infoxchange_worker();

    // call the components's do_work(), note this gives the component a thread to execute on 
    loop {
        // These braces will ensure our mutex gets dropped before we enter the .await
        // This is something that could aslo be accomplished by std::mem::drop, but
        //    doing a drop just before the .await seems to confuse rust's static analysis...
        //    so using the braces to accomplish the same thing.
        { 
            println!("\ncomp_worker() calling do_work()");
            let mut store = comp_ctx.store.lock().unwrap();
            
            if !comp.call_do_work(&mut *store).await? {
            //if !wasmtime_wasi::runtime::with_ambient_tokio_runtime(||comp.call_do_work(&mut *store))? {
                break; // if do_work returns false, the break out of this loop and exit the thread
            }
        }
        executor.sleep(std::time::Duration::from_secs(1)).await;
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let executor = TokioExecutor;
    let comp_ctx = CompContext::new("./infoxchange.wasm").await?;
    let worker_ctx = comp_ctx.clone();
    let comp_worker = executor.spawn( comp_worker(executor,worker_ctx) );
    
    // just for this test case, let's sleep for 3s to give our comp_worker a chance to run for a bit before
    // setting the id
    executor.sleep(std::time::Duration::from_secs(3)).await;

    {
        let id = 1;
        let mut store = comp_ctx.store.lock().unwrap();
        println!("\nmain() calling set_id: {}", id);
        match comp_ctx.bindings.wasmexample_infoxchange_id_holder().call_set_id(&mut *store, id) {
            Ok(_) => println!("main() successfully set_id: {}", id),
            Err(err) => println!("main() set_id: {} returned an error: {}", id, err),
        };
    }

    comp_worker.await.unwrap()?;
    Ok(())
}