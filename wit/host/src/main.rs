use wasmtime::component::ResourceTable;
use wasmtime_wasi::preview2::WasiCtxBuilder;

wasmtime::component::bindgen!({
    world: "kernel",
    path: "../wit",
});

struct WasmtimeContext {
    // table: wasmtime::runtime::component::ResourceTable,
    table: wasmtime::component::ResourceTable,
    wasi: wasmtime_wasi::preview2::WasiCtx,
}

impl WasmtimeContext {
    fn new(
        table: wasmtime::component::ResourceTable,
        wasi: wasmtime_wasi::preview2::WasiCtx,
    ) -> Self {
        Self { table, wasi }
    }
}

impl wasmtime_wasi::preview2::WasiView for WasmtimeContext {
    fn table(&mut self) -> &mut wasmtime::component::ResourceTable {
        &mut self.table
    }

    fn ctx(&mut self) -> &mut wasmtime_wasi::preview2::WasiCtx {
        &mut self.wasi
    }
}


fn main() -> Result<(), wasmtime::Error> {
    let component_bytes =
        std::fs::read("/workspaces/dust/target/wasm32-wasi/debug/wit_guest_component.wasm")
            .unwrap();
    println!("guest wasm size: {:?}", component_bytes.len());

    let engine = wasmtime::Engine::default();
    let mut linker = wasmtime::component::Linker::<WasmtimeContext>::new(&engine);
    wasmtime_wasi::preview2::command::sync::add_to_linker(&mut linker)?;

    let mut store = wasmtime::Store::new(
        &engine,
        WasmtimeContext::new(
            ResourceTable::new(),
            WasiCtxBuilder::new()
                .inherit_stdout()
                .inherit_stderr()
                .build(),
        ),
    );

    let component = wasmtime::component::Component::new(&engine, &component_bytes)?;
    let instance = linker.instantiate(&mut store, &component)?;

    {
        let func = instance.get_func(&mut store, "bootstrap").unwrap();
        let params = [wasmtime::component::Val::String("hello".into())];
        let mut results = [wasmtime::component::Val::String(Default::default())];
        func.call(&mut store, &params, &mut results)?;
        func.post_return(&mut store)?;
        println!("bootstrap: {:?}", results);
    }
    {
        let func = instance.get_func(&mut store, "poll").unwrap();
        let params = [wasmtime::component::Val::String("hello".into())];
        let mut results = [wasmtime::component::Val::String(Default::default())];
        func.call(&mut store, &params, &mut results).unwrap();
        func.post_return(&mut store)?;
        println!("tick: {:?}", results);
    }
    {
        let func = instance.get_func(&mut store, "cleanup").unwrap();
        let params = [wasmtime::component::Val::String("hello".into())];
        let mut results = [wasmtime::component::Val::String(Default::default())];
        func.call(&mut store, &params, &mut results).unwrap();
        func.post_return(&mut store)?;
        println!("results: {results:?}");
    }
    Ok(())
}
