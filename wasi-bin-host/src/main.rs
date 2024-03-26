struct MyState {
    message: String,
    wasi: wasi_common::WasiCtx,
}

fn main() -> Result<(), wasmtime::Error> {
    //
    println!("create wasmtime engine with default configuration");
    let engine = wasmtime::Engine::default();

    //
    println!("create store");
    let mut store = wasmtime::Store::new(
        &engine,
        MyState {
            message: format!("hello"),
            wasi: wasi_common::sync::WasiCtxBuilder::new()
                .inherit_stdio()
                .inherit_args()?
                .build(),
        },
    );

    //
    println!("create module");
    println!("curr_dir: {:?}", std::env::current_dir());
    let module =
        wasmtime::Module::from_file(&engine, "./target/wasm32-wasi/release/wasi-bin.wasm")?;

    //
    println!("create linker");
    let mut linker = wasmtime::Linker::new(&engine);
    wasi_common::sync::add_to_linker(&mut linker, |state: &mut MyState| &mut state.wasi)?;

    //
    println!("call default function in module");
    linker.module(&mut store, "", &module)?;
    linker
        .get_default(&mut store, "")?
        .typed::<(), ()>(&mut store)?
        .call(&mut store, ())?;

    //
    println!("done");
    Ok(())
}
