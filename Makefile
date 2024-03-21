build:
	cargo build
	# cargo build --package module-wasi --target wasm32-unknown-unknown --release
	cargo build --package wasi-bin --target wasm32-wasi --release
