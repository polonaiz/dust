use wasmtime::component::ResourceTable;
use wasmtime_wasi::preview2::WasiCtxBuilder;

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

    let mut config = wasmtime::Config::new();
    config.consume_fuel(true);
    let engine = wasmtime::Engine::new(&config).unwrap();
    let mut linker = wasmtime::component::Linker::<WasmtimeContext>::new(&engine);
    wasmtime_wasi::preview2::command::sync::add_to_linker(&mut linker)?;

    let mut store = wasmtime::Store::new(
        &engine,
        WasmtimeContext::new(
            ResourceTable::new(),
            WasiCtxBuilder::new()
                .inherit_stdout()
                .inherit_stderr()
                .inherit_network()
                .preopened_dir(
                    wasmtime_wasi::sync::Dir::from_std_file(std::fs::File::open("/tmp").unwrap()),
                    wasmtime_wasi::preview2::DirPerms::MUTATE
                        | wasmtime_wasi::preview2::DirPerms::READ,
                    wasmtime_wasi::preview2::FilePerms::WRITE
                        | wasmtime_wasi::preview2::FilePerms::READ,
                    "data",
                )
                .build(),
        ),
    );
    store.set_fuel(10_000_000).unwrap();

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

    loop {
        std::thread::sleep(std::time::Duration::from_millis(5000));

        {
            let func = instance.get_func(&mut store, "poll").unwrap();
            let params = [wasmtime::component::Val::String("hello".into())];
            let mut results = [wasmtime::component::Val::String(Default::default())];
            func.call(&mut store, &params, &mut results).unwrap();
            func.post_return(&mut store)?;
            println!("tick: {:?}", results);
        }
        println!("fuel: {:?}", store.get_fuel().unwrap());
    }

    // {
    //     let func = instance.get_func(&mut store, "cleanup").unwrap();
    //     let params = [wasmtime::component::Val::String("hello".into())];
    //     let mut results = [wasmtime::component::Val::String(Default::default())];
    //     func.call(&mut store, &params, &mut results).unwrap();
    //     func.post_return(&mut store)?;
    //     println!("results: {results:?}");
    // }
    // Ok(())
}
