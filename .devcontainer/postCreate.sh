echo '----- start postCreate.sh -----'

whoami

rustup target add wasm32-unknown-unknown
rustup target add wasm32-wasi
cargo install wasm-tools

echo '----- end postCreate.sh -----'
