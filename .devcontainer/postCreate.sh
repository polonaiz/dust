echo '----- start postCreate.sh -----'

whoami

rustup target add wasm32-unknown-unknown
rustup target add wasm32-wasi

echo '----- end postCreate.sh -----'
