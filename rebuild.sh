#!/bin/bash

rm -r pkg
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-dir pkg --target web ./target/wasm32-unknown-unknown/release/shooter.wasm
