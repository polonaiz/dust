wasmtime::component::bindgen!({
    world: "lib",
    path: "../wit",
});

fn main() -> Result<(), wasmtime::Error> {
    let component_bytes =
        std::fs::read("../../target/wasm32-unknown-unknown/debug/wit_guest_component.wasm").unwrap();

    let engine = wasmtime::Engine::default();
    let mut store = wasmtime::Store::new(&engine, ());
    let linker = wasmtime::component::Linker::new(&engine);

    let component = wasmtime::component::Component::new(&engine, &component_bytes)?;
    let instance = linker.instantiate(&mut store, &component)?;

    let mut results = [wasmtime::component::Val::String(Default::default())];
    instance
        .get_func(&mut store, "to-upper")
        .unwrap()
        .call(&mut store,&[wasmtime::component::Val::String("hello".into())],&mut results)
        .unwrap();

    println!("results: {results:?}");
    Ok(())
}
