struct MyState {
    name: String,
    count: usize,
}

fn main() -> Result<(), wasmtime::Error> {

    let engine = wasmtime::Engine::default();
    let module = wasmtime::Module::from_file(&engine, "/workspaces/dust/src/hello.wat")?;
    let mut store = wasmtime::Store::new(
        &engine,
        MyState {
            name: "supplied state".to_string(),
            count: 0,
        },
    );

    let hello_func =
        wasmtime::Func::wrap(&mut store, |mut caller: wasmtime::Caller<'_, MyState>| {
            println!("> {}", caller.data().name);
            caller.data_mut().count += 1;
            println!("> {}", caller.data().count);
        });

    let imports = [hello_func.into()];
    let instance = wasmtime::Instance::new(&mut store, &module, &imports).unwrap();

    let run = instance.get_typed_func::<(), ()>(&mut store, "run").unwrap();
    run.call(&mut store, ()).unwrap();
    run.call(&mut store, ()).unwrap();

    Ok(())
}
