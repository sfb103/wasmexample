use wasmtime::{component::{bindgen, ResourceTable}, AsContextMut, Result};
use wasmtime_wasi::{IoView, WasiCtx, WasiView, WasiCtxBuilder};

use async_executor::executor::Executor;
use async_executor::tokio_executor::TokioExecutor;
use wasm_comp_ctx::CompContext;

use wasmexample::infoxchange::status_holder::{self, Status};

bindgen!({
    world: "infoxchange",
    path: "./wit",
    async: true,
});

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

async fn comp_worker<E: Executor>(executor: E, comp_ctx: CompContext<XchangeHost,Infoxchange>) -> Result<()> {
    let comp = comp_ctx.bindings.wasmexample_infoxchange_worker();

    // call the components's do_work(), note this gives the component a task to execute on 
    loop { 
        println!("\ncomp_worker() calling do_work()");
        let mut store = comp_ctx.store.lock().await;
        if !comp.call_do_work(store.as_context_mut()).await? {
            break; // if do_work returns false, then break out of this loop and exit the task
        }
        drop(store); // release the lock before sleeping
        executor.sleep(std::time::Duration::from_secs(1)).await;
    }
    Ok(())
}

#[tokio::main(flavor="current_thread")]
async fn main() -> Result<()> {
    let executor = TokioExecutor;
    let comp_ctx = CompContext::new(
        "./infoxchange.wasm",
        || XchangeHost {
            // add permissions to the wasi sandbox as needed
            wasi_ctx: WasiCtxBuilder::new()
                .inherit_stdout()
                .build(),
            table: ResourceTable::new(),
            status: Status::Unknown,
        },
        |linker| { Infoxchange::add_to_linker(linker, |host_state: &mut XchangeHost| host_state) },
        |mut store, instance| { Infoxchange::new(store.as_context_mut(), instance) },
    ).await?;
    let comp_worker = executor.spawn( comp_worker(executor, comp_ctx.clone()) );
    
    // just for this test case, let's sleep for 3s to give our comp_worker a chance to run for a bit before
    // setting the id
    executor.sleep(std::time::Duration::from_secs(3)).await;

    let id = 1;
    let mut store = comp_ctx.store.lock().await;
    println!("\nmain() calling set_id: {}", id);
    match comp_ctx.bindings.wasmexample_infoxchange_id_holder().call_set_id(store.as_context_mut(), id).await {
        Ok(_) => println!("main() successfully set_id: {}", id),
        Err(err) => println!("main() set_id: {} returned an error: {}", id, err),
    };
    drop(store); // release the lock before waiting on comp_worker

    comp_worker.await.unwrap()?;
    Ok(())
}