#!/bin/bash
set -e
export WASM_NAME=nft.wasm
RUSTFLAGS='-C link-args=-s' rustup target add wasm32-unknown-unknown
cargo build --target wasm32-unknown-unknown --release
sudo mkdir out && sudo chown -R root:root out && sudo chmod 777 -R out 
cp target/wasm32-unknown-unknown/release/$WASM_NAME out/$WASM_NAME
sudo npm i wasm-opt -g
wasm-opt -Oz out/$WASM_NAME -o out/$WASM_NAME # execute default optimization, passes, super-focusing on code