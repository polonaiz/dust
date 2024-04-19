build:
	cargo build
	cargo build --package wasi-bin --target wasm32-unknown-unknown --release
	cargo build --package wasi-bin --target wasm32-wasi --release

build-wasi-module:
	cargo build --package wasi-module --target wasm32-wasi --release
	ls -al ./target/wasm32-wasi/release/

invoke-wasi-module:
	wasmtime --invoke say_hello ./target/wasm32-wasi/release/wasi_module.wasm 

build-wit: build-wit-guest build-wit-host

build-wit-guest:
	cargo build -p wit-guest --target wasm32-wasi
	wasm-tools component new \
		./target/wasm32-wasi/debug/wit_guest.wasm \
		-o ./target/wasm32-wasi/debug/wit_guest_component.wasm \
		--adapt ./wasi_snapshot_preview1.reactor.wasm
	gzip --keep --force ./target/wasm32-wasi/debug/wit_guest_component.wasm
	ls -alh ./target/wasm32-wasi/debug/wit_guest_component.wasm*

wepl-wit-guest: build-wit-guest
	wepl ./target/wasm32-wasi/debug/wit_guest_component.wasm

build-wit-host:
	cargo build -p wit-host

run-wit-host: build-wit
	RUST_BACKTRACE=full ./target/debug/wit-host

