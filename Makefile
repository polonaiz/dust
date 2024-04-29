env:
	rustup target add wasm32-unknown-unknown
	rustup target add wasm32-wasi
	cargo install wasm-tools

build: build-guest build-host

build-guest:
	cargo build -p guest --target wasm32-wasi
	wasm-tools component new \
		./target/wasm32-wasi/debug/guest.wasm \
		-o ./target/wasm32-wasi/debug/guest_component.wasm \
		--adapt ./wasi_snapshot_preview1.reactor.wasm
	gzip --keep --force ./target/wasm32-wasi/debug/guest_component.wasm
	ls -alh ./target/wasm32-wasi/debug/guest_component.wasm*

wepl-install:
	git clone https://github.com/rylev/wepl.git /workspaces/wepl
	cd /workspaces/wepl && cargo install --path .
	rm -rf /workspaces/wepl

wepl-guest: build-guest
	wepl ./target/wasm32-wasi/debug/guest_component.wasm

build-host:
	cargo build -p host

run-host: build
	RUST_BACKTRACE=full ./target/debug/host

