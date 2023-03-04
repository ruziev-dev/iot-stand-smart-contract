#!/bin/bash

if [ -z "$KEEP_NAMES" ]; then
  export RUSTFLAGS='-C link-arg=-s'
else
  export RUSTFLAGS=''
fi
cargo build --all --target wasm32-unknown-unknown --release

near deploy iot_test.testnet --wasmFile ./target/wasm32-unknown-unknown/release/iot_stand_contract.wasm --initFunction new --initArgs '{"owner_name": "ruziev.testnet"}'