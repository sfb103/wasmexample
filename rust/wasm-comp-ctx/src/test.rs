mod test_export_only {
    use wasmtime::{component::{bindgen, ResourceTable}, AsContextMut};
    use wasmtime_wasi::{WasiCtx, WasiView, IoView, WasiCtxBuilder};
    use crate::CompContext;

    bindgen!({
        world: "adder-export-only",
        path: "./test/test_export_only.wit", // path to the wit for the component
        async: true,
    });

    struct TestHost {
        wasi_ctx: WasiCtx,
        table: ResourceTable,
    }
    impl IoView for TestHost{
        fn table(&mut self) -> &mut ResourceTable {
            &mut self.table
        }
    }
    impl WasiView for TestHost {
        fn ctx(&mut self) -> &mut WasiCtx {
            &mut self.wasi_ctx
        }
    }

    #[tokio::test]
    async fn test_component_instantiation() {
        let comp_ctx = CompContext::new(
            "./test/test_export_only.wasm",
            || TestHost {
                // add permissions to the wasi sandbox as needed
                wasi_ctx: WasiCtxBuilder::new()
                    .inherit_stdout()
                    .build(),
                table: ResourceTable::new(),
            },
            |_| Ok(()),
            |mut store, instance| AdderExportOnly::new(store.as_context_mut(), instance),
        ).await.unwrap();

        // test our comp_ctx is clone-able, required for passing the ctx into tasks
        let comp_ctx_clone = comp_ctx.clone();
        let mut store = comp_ctx_clone.store.lock().await;

        // use the component to add 2 plus 2
        let two_plus_two = comp_ctx.bindings.docs_adder_export_only_add()
            .call_add(store.as_context_mut(), 2, 2).await.unwrap();
        assert_eq!( two_plus_two, 4 );

        println!("Called add, 2 + 2 = {}", two_plus_two);
        println!("Done.")
    }
}

mod test_import_export {
    use wasmtime::{component::{bindgen, ResourceTable}, AsContextMut};
    use wasmtime_wasi::{WasiCtx, WasiView, IoView, WasiCtxBuilder};
    use crate::CompContext;
    use docs::adder_import_export::test;

    bindgen!({
        world: "adder-import-export",
        path: "./test/test_import_export.wit", // path to the wit for the component
        async: true,
    });

    struct TestHost {
        wasi_ctx: WasiCtx,
        table: ResourceTable,
    }
    impl IoView for TestHost{
        fn table(&mut self) -> &mut ResourceTable {
            &mut self.table
        }
    }
    impl WasiView for TestHost {
        fn ctx(&mut self) -> &mut WasiCtx {
            &mut self.wasi_ctx
        }
    }
    //implementation of the Host trait specific to AdderImportExport world
    //this is imported by component, so must be implemented by TestHost
    impl test::Host for TestHost {
        async fn test(&mut self,) -> bool {
            true
        }
    }

    #[tokio::test]
    async fn test_component_instantiation() {
        let comp_ctx = CompContext::new(
            "./test/test_import_export.wasm",
            || TestHost {
                // add permissions to the wasi sandbox as needed
                wasi_ctx: WasiCtxBuilder::new()
                    .inherit_stdout()
                    .build(),
                table: ResourceTable::new(),
            },
            |linker| AdderImportExport::add_to_linker(linker, |host: &mut TestHost| host),
            |mut store, instance| AdderImportExport::new(store.as_context_mut(), instance),
        ).await.unwrap();

        // test our comp_ctx is clone-able, required for passing the ctx into tasks
        let comp_ctx_clone = comp_ctx.clone();
        let mut store = comp_ctx_clone.store.lock().await;

        // use the component to add 2 plus 2
        let two_plus_two = comp_ctx.bindings.docs_adder_import_export_add()
            .call_add(store.as_context_mut(), 2, 2).await.unwrap();
        assert_eq!( two_plus_two, 4 );

        println!("Called add, 2 + 2 = {}", two_plus_two);
        println!("Done.")
    }
}

mod test_import_only {
    use wasmtime::{component::{bindgen, ResourceTable}, AsContextMut};
    use wasmtime_wasi::{WasiCtx, WasiView, IoView, WasiCtxBuilder};
    use crate::CompContext;
    use docs::adder_import_only::test;

    bindgen!({
        world: "adder-import-only",
        path: "./test/test_import_only.wit", // path to the wit for the component
        async: true,
    });

    struct TestHost {
        wasi_ctx: WasiCtx,
        table: ResourceTable,
    }
    impl IoView for TestHost{
        fn table(&mut self) -> &mut ResourceTable {
            &mut self.table
        }
    }
    impl WasiView for TestHost {
        fn ctx(&mut self) -> &mut WasiCtx {
            &mut self.wasi_ctx
        }
    }
    //implementation of the Host trait specific to AdderImportExport world
    //this is imported by component, so must be implemented by TestHost
    impl test::Host for TestHost {
        async fn test(&mut self,) -> bool {
            true
        }
    }

    #[tokio::test]
    async fn test_component_instantiation() {
        let comp_ctx = CompContext::new(
            "./test/test_import_only.wasm",
            || TestHost {
                // add permissions to the wasi sandbox as needed
                wasi_ctx: WasiCtxBuilder::new()
                    .inherit_stdout()
                    .build(),
                table: ResourceTable::new(),
            },
            |linker| AdderImportOnly::add_to_linker(linker, |host: &mut TestHost| host),
            |mut store, instance| AdderImportOnly::new(store.as_context_mut(), instance),
        ).await.unwrap();

        // test our comp_ctx is clone-able, required for passing the ctx into tasks
        let comp_ctx_clone = comp_ctx.clone();
        let mut _store = comp_ctx_clone.store.lock().await;

        println!("Done.")
    }
}