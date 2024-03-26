struct MyState {
    wasi: wasi_common::WasiCtx,
}

fn main() -> Result<(), wasmtime::Error> {
    println!("create wasmtime engine with default configuration");
    let engine = wasmtime::Engine::default();

    println!("create store");
    let mut store = wasmtime::Store::new(
        &engine,
        MyState {
            wasi: wasi_common::sync::WasiCtxBuilder::new()
                .inherit_stdio()
                .inherit_args()?
                .build(),
        },
    );

    println!("create module");
    println!("curr_dir: {:?}", std::env::current_dir());
    let module = wasmtime::Module::from_file(
        &engine,
        "/workspaces/dust/target/wasm32-wasi/release/wasi_module.wasm",
    )?;

    println!("create linker");
    let mut linker = wasmtime::Linker::new(&engine);
    println!("add wasi component");
    wasi_common::sync::add_to_linker(&mut linker, |state: &mut MyState| &mut state.wasi)?;

    println!("invoke func");
    let result = linker
        .module(&mut store, "", &module)?
        .get(&mut store, "", "say_hello").unwrap().into_func().unwrap()
        .call(&mut store, &[], &mut [])?;
    println!("done");
    Ok(())
}
