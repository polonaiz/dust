wit_bindgen::generate!({
    world: "lib",
    path: "../wit",
});

struct Lib;

impl Guest for Lib {
    fn run(_input: String) -> String {

        // test execute async func on tokio current thread runtime
        let runtime = tokio::runtime::Builder::new_current_thread()
            .build()
            .unwrap();
        runtime.block_on( async { println!("print from async tokio from wasi") });


        "from wasi".to_string()
    }
}
export!(Lib);
