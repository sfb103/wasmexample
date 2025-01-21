# Withdraw Wasm Example

## To build and run...

### Build Wasm
This withdraw implemenation is written in Rust.  The source can be found in `src/main.rs`.

Using the `wasmdevcontainer` (see instructions in the toplevel [README.md](../../README.md)), the `withdraw.wasm` can be built with simply:
```
./build.sh
```
This simply runs the command: `cargo build --release`.

The build config under `.cargo/config.toml` ensures this builds to the `wasm32-unknown-unknown` target.  This will place the `withdraw.wasm` file in the correct folder under `target/wasm32-unknown-unknown/release`

### Run with Wasmtime runtime

Execute in a shell with wasmtime
```
./run.sh 732 43
```
This simply runs the comand: `wasmtime ../../target/wasm32-unknown-unknown/release/withdraw.wasm --invoke withdraw $1 $2`

### Sample output

```
myuser:~/wasmexample/rust/withdraw[$] ./run.sh 732 43
myuser:~/wasmexample/rust/withdraw[$] 
```
This wasm module currently does not provide any output when run.  It is serving the purpose of producing a wasm that can then be converted to wat for comparison to other wat sourced from like implemenations in different languages, such as C and Go.

### Convert to wat

Execute in a shell
```
./wasm2wat.sh
```
This simply runs the command: `wasm2wat ../../target/wasm32-unknown-unknown/release/withdraw.wasm > withdraw.wat` and produces `withdraw.wat`.