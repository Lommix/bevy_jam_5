#!/bin/bash
cargo build --target wasm32-unknown-unknown --release
wasm-bindgen --no-typescript --target web --out-dir ./wasm --out-name "brew_tycoon" ./target/wasm32-unknown-unknown/release/brew_tycoon.wasm
wasm-opt -Oz --output ./wasm/brew_tycoon_bg.wasm ./wasm/brew_tycoon_bg.wasm

cd wasm
zip -r ../brew_tycoon.zip .
