use std::sync::{Arc, Mutex};
use std::thread;

use wasmexample::infoxchange::status_holder::{self, Status};
use wasmtime::{component::*, Error};
use wasmtime::{Result, Engine, Store};
use wasmtime_wasi::{WasiCtx, WasiView};

bindgen!("infoxchange" in "./wit");

#[derive(Clone)]
struct CompContext {
    store: Arc<Mutex<Store<XchangeHost>>>,
    bindings: Arc<Infoxchange>,
}

impl CompContext {
    pub fn new(wasm: &str) -> Result<Self, Error> {
        // create our component
        let engine = Engine::default();
        let component = Component::from_file(&engine, wasm)?;
    
        let mut linker = Linker::<XchangeHost>::new(&engine);
        wasmtime_wasi::add_to_linker_sync(&mut linker)?;
        Infoxchange::add_to_linker(&mut linker, |s: &mut XchangeHost| s)?;
    
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
        let bindings = Infoxchange::instantiate(&mut store, &component, &linker)?;
        
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

impl WasiView for XchangeHost {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.wasi_ctx
    }
}

impl status_holder::Host for XchangeHost {
    fn set_status(&mut self,status: Status) -> Result<(),String> {
        println!("set_status() called with status: {:?}", status);
        self.status = status;
        Ok(())
    }

    fn get_status(&mut self,) -> Result<Status,String> {
        Ok(self.status)
    }
}

fn comp_worker(comp_ctx: CompContext) -> Result<()> {
    let comp = comp_ctx.bindings.wasmexample_infoxchange_worker();

    // call the components's do_work(), note this gives the component a thread to execute on 
    loop {
        println!("\ncomp_worker() calling do_work()");
        let mut store = comp_ctx.store.lock().unwrap();
        if !comp.call_do_work(&mut *store)? {
            break; // if do_work returns false, the break out of this loop and exit the thread
        }
        std::mem::drop(store);
        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    Ok(())
}

fn main() -> Result<()> {
    let comp_ctx = CompContext::new("./wasm/target/wasm32-wasip2/debug/infoxchange.wasm")?;
    let worker_ctx = comp_ctx.clone();
    let comp_worker = thread::spawn( move||{ comp_worker(worker_ctx).unwrap(); } );
    
    // just for this test case, let's sleep for 3s to give our comp_worker a chance to run for a bit before
    // setting the id
    std::thread::sleep(std::time::Duration::from_secs(3));

    {
        let id = 1;
        let mut store = comp_ctx.store.lock().unwrap();
        println!("\nmain() calling set_id: {}", id);
        match comp_ctx.bindings.wasmexample_infoxchange_id_holder().call_set_id(&mut *store, id) {
            Ok(_) => println!("main() successfully set_id: {}", id),
            Err(err) => println!("main() set_id: {} returned an error: {}", id, err),
        };
    }

    comp_worker.join().unwrap();
    Ok(())
}