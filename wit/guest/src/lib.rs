wit_bindgen::generate!({
    world: "kernel",
    path: "../wit",
});

lazy_static::lazy_static! {
    static ref TOKIO_RUNTIME: tokio::runtime::Runtime = tokio::runtime::Builder::new_current_thread().build().unwrap();
}

struct Kernel {}

impl Guest for Kernel {
    fn bootstrap(_config: String) -> String {
        lazy_static::initialize(&TOKIO_RUNTIME);

        TOKIO_RUNTIME.spawn(async move { println!("async in bootstrap") });
        "from_boot".to_string()
    }

    fn poll(_input: String) -> String {
        TOKIO_RUNTIME.spawn(async move { println!("async in poll") });

        "from_tick".to_string()
    }

    fn cleanup(_input: String) -> String {
        TOKIO_RUNTIME.spawn(async move { println!("async in cleanup") });

        "from_shutdown".to_string()
    }
}

export!(Kernel);
