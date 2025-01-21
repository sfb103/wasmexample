# Calculator Wasm Example

## To build and run...

### Build Wasm
The calculator Wasm is written in Rust.  The source can be found in `src/main.rs`.

Using the `wasmdevcontainer` (see instructions in the toplevel [README.md](../../README.md)), the `calculator.wasm` can be built with simply:
```
cargo build
```
The build config under `.cargo/config.toml` ensures this builds to the `wasm32-wasip1` target.  This will place the `calculator.wasm` file in the correct folder under `target/wasm32-wasip1/debug`

### Run with Wasmtime runtime

Execute in a shell with wasmtime
```
wasmtime target/wasm32-wasip1/debug/calculator.wasm 35 78
```

### Sample output:

```
myuser:~/wasmexample/rust/calculator[$] wasmtime target/wasm32-wasi/debug/calculator.wasm 33 78
33 + 78 = 113
myuser:~/wasmexample/rust/calculator[$] 
```
