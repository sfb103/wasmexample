#!/bin/bash
clang --target=wasm32-wasi --sysroot=/wasi-sysroot --output=withdraw.wasm -Wl,--demangle,--export=withdraw main.c