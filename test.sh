#!/bin/sh

# unit testing
cargo test

# sandbox testing
./build.sh
cd sandbox-ts
npm install
npm run test -- -- "../target/wasm32-unknown-unknown/release/cert_issuer.wasm"

