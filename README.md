# Rust and WebAssembly workshop

Let's build a 2d shooter using canvas, wasm-bindgen and wasm-pack

## Environment setup

```bash
cargo install wasm-bindgen-cli --version 0.2.83
# optionally, https static server
cargo install https
```

## Building

```bash
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-dir pkg --target web ./target/wasm32-unknown-unknown/release/shooter.wasm
```

## Running

```bash
http
```

alternatively

```bash
python -m SimpleHTTPServer
```

then navigate to `host:port`
