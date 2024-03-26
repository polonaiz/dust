build:
	cargo build
	cargo build --package wasi-bin --target wasm32-unknown-unknown --release
	cargo build --package wasi-bin --target wasm32-wasi --release

build-wasi-module:
	cargo build --package wasi-module --target wasm32-wasi --release
	ls -al ./target/wasm32-wasi/release/

invoke-wasi-module:
	wasmtime --invoke say_hello ./target/wasm32-wasi/release/wasi_module.wasm 
