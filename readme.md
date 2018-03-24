# WASM Pong

A simple pong clone implemented in Rust and WebAssembly.

## Installation

First setup the wasm32-unknown-unknown compiler target and wasm-gc
(for making the compiled output smaller), then run `make`.

```sh
rustup target add wasm32-unknown-unknown
cargo install wasm-gc
make
```

## Running

Run the game by serving `index.html` and `pong.wasm`, e.g. with [serve](https://github.com/zeit/serve).

```sh
npm install -g serve
serve
```

The game is now live at: [http://localhost:5000](http://localhost:5000)
