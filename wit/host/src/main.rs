use wasmtime::component::ResourceTable;
use wasmtime_wasi::preview2::WasiCtxBuilder;

wasmtime::component::bindgen!({
    world: "lib",
    path: "../wit",
});

struct Context {
    // table: wasmtime::runtime::component::ResourceTable,
    table: wasmtime::component::ResourceTable,
    wasi: wasmtime_wasi::preview2::WasiCtx,
}

impl Context {
    fn new(
        table: wasmtime::component::ResourceTable,
        wasi: wasmtime_wasi::preview2::WasiCtx,
    ) -> Self {
        Self { table, wasi }
    }
}

impl wasmtime_wasi::preview2::WasiView for Context {
    fn table(&mut self) -> &mut wasmtime::component::ResourceTable {
        &mut self.table
    }

    fn ctx(&mut self) -> &mut wasmtime_wasi::preview2::WasiCtx {
        &mut self.wasi
    }
}

fn main() -> Result<(), wasmtime::Error> {
    let component_bytes =
        std::fs::read("/workspaces/dust/target/wasm32-wasi/debug/wit_guest_component.wasm").unwrap();

    let engine = wasmtime::Engine::default();
    let mut linker = wasmtime::component::Linker::<Context>::new(&engine);
    wasmtime_wasi::preview2::command::sync::add_to_linker(&mut linker)?;

    let mut store = wasmtime::Store::new(
        &engine,
        Context::new(
            ResourceTable::new(),
            WasiCtxBuilder::new()
                .inherit_stdout()
                .inherit_stderr()
                .build(),
        ),
    );

    let component = wasmtime::component::Component::new(&engine, &component_bytes)?;
    let instance = linker.instantiate(&mut store, &component)?;

    let mut results = [wasmtime::component::Val::String(Default::default())];
    instance
        .get_func(&mut store, "to-upper")
        .unwrap()
        .call(
            &mut store,
            &[wasmtime::component::Val::String("hello".into())],
            &mut results,
        )
        .unwrap();

    println!("results: {results:?}");
    Ok(())
}
