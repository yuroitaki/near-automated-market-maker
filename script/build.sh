#!/bin/bash
set -e

cd "`dirname $0`"/../contract
cargo build --all --target wasm32-unknown-unknown --release
cd ..
cp contract/target/wasm32-unknown-unknown/release/*.wasm ./res/
