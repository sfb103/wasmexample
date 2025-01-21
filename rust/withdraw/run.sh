#!/bin/bash
wasmtime ../../target/wasm32-unknown-unknown/release/withdraw.wasm --invoke withdraw $1 $2