#!/bin/bash
set -e

pushd $(dirname ${BASH_SOURCE[0]})
mkdir -p res

RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/simple_faucet_contract.wasm ./res/

ls -lsa res/simple_faucet_contract.wasm
