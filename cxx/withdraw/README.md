# Withdraw Wasm Example

## To build and run...

### Build Wasm
This withdraw implemenation is written in C.  The source can be found in `main.c`.

Using the `wasmdevcontainer` (see instructions in the toplevel [README.md](../../README.md)), the `withdraw.wasm` can be built with simply:
```
./build.sh
```
This simply runs the command: `clang --target=wasm32-wasip1 --sysroot=/wasi-sysroot --output=withdraw.wasm -Wl,--demangle,--export=withdraw main.c`.


### Run with Wasmtime runtime

Execute in a shell with wasmtime
```
./run.sh 732 43
```
This simply runs the comand: `wasmtime withdraw.wasm --invoke withdraw $1 $2`

### Sample output

```
myuser:~/wasmexample/cxx/withdraw[$] ./run.sh 732 43
myuser:~/wasmexample/cxx/withdraw[$] 
```
This wasm module currently does not provide any output when run.  It is serving the purpose of producing a wasm that can then be converted to wat for comparison to other wat sourced from like implemenations in different languages, such as Go and Rust.

### Convert to wat

Execute in a shell
```
./wasm2wat.sh
```
This simply runs the command: `wasm2wat withdraw.wasm > withdraw.wat` and produces `withdraw.wat`.
