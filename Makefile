all:
	cargo build --target=wasm32-unknown-unknown --release
	wasm-gc ./target/wasm32-unknown-unknown/release/pong.wasm ./pong.wasm
