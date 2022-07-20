#!/bin/bash
env RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/near_proxy.wasm res/
ls -la res/near_proxy.wasm
