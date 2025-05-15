use std::sync::Arc;
use tokio::sync::Mutex;

use wasmtime::{
    component::{Component, Instance, Linker},
    AsContextMut, Config, Engine, Error, Result, Store, StoreContextMut,
};
use wasmtime_wasi::WasiView;

pub struct CompContext<THost, TBindings>
where
    THost: WasiView + Send + 'static,
    TBindings: Send + Sync + 'static,
{
    pub store: Arc<Mutex<Store<THost>>>,
    pub bindings: Arc<TBindings>,
}

impl<THost, TBindings> CompContext<THost, TBindings>
where
    THost: WasiView + Send + 'static, // WasiView constraint required for wasmtime_wasi::add_to_linker_async
    TBindings: Send + Sync + 'static,
{
    pub async fn new<FCreateHostData, FLinkerAdd, FCreateBindings>(
        // path to the wasm component
        wasm_path: &str,        
        // closure to create the host-specific data
        create_host_data: FCreateHostData,
        // closure to add host to the linker
        add_host_to_linker: FLinkerAdd,
        // closure to create the bindings instance
        create_bindings_instance: FCreateBindings,
    ) -> Result<Self, Error>
    where
        FCreateHostData: FnOnce() -> THost,
        FLinkerAdd: FnOnce(&mut Linker<THost>) -> Result<()>,
        FCreateBindings: FnOnce(StoreContextMut<'_, THost>, &Instance) -> Result<TBindings>,
    {
        // setup our engine and linker to support an async context
        let mut config = Config::new();
        config.async_support(true);
        let engine = Engine::new(&config)
            .map_err(|e| Error::msg(format!("Failed to create engine: {}", e)))?;
        let mut linker = Linker::<THost>::new(&engine);

        // add WASI to the linker (generic part)
        wasmtime_wasi::add_to_linker_async(&mut linker)?;

        // add world-specific host functions (provided by caller)
        add_host_to_linker(&mut linker)?;

        // setup our store that holds the context our host uses with our component
        let host_data = create_host_data();
        let mut store = Store::new(&engine, host_data);

        // instantiate our component and its bindings
        let component = Component::from_file(&engine, wasm_path)
            .map_err(|e| Error::msg(format!("Failed to load component from {}: {}", wasm_path, e)))?;
        let instance = linker.instantiate_async(store.as_context_mut(), &component).await?;
        let bindings = create_bindings_instance(store.as_context_mut(), &instance)?;
        
        Ok(Self {
            store: Arc::new(Mutex::new(store)),
            bindings: Arc::new(bindings),
        })
    }
}

// We want to support Clone for CompContext, so an instance of this struct can be
// shared between tasks/threads. When using #[derive(Clone)], the compiler will
// automatically add a Clone constraint to our generic types (THost, TBindings).
// In our case, Clone for these types is unnecessary. So to support Clone without
// placing that constraint on our generics, we'll implement Clone for CompContext
// manually with a custom impl of fn clone().
impl<THost, TBindings> Clone for CompContext<THost, TBindings>
where
    THost: WasiView + Send + 'static,
    TBindings: Send + Sync + 'static,
{
    fn clone(&self) -> Self {
        Self {
            store: Arc::clone(&self.store),
            bindings: Arc::clone(&self.bindings),
        }
    }
}

#[cfg(test)]
mod test;