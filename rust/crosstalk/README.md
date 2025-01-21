# Crosstalk Wasm Example

## To build and run...

### Build Wasm
The crosstalk Wasm is written in Rust.  The source can be found in `wasm/src/lib.rs`.  Note the static `TALK` str, currently defined as `Hello World, from WebAssembly!!!`.<br>

Using the `wasmdevcontainer` (see instructions in the toplevel [README.md](../../README.md)), the `crosstalk.wasm` can be built with simply:
```
cd wasm
cargo build
cd ..
```
The build config under `.cargo/config.toml` ensures this builds to the wasm32-unknown-unknown (i.e. core wasm) target.  This will place the `crosstalk.wasm` file in the correct folder under `public/wasm32-unknown-unknown/debug`

### Run in JavaScript
The JavaScript hosting code that compiles and runs `crosstalk.wasm` can be served with the following commands:
```
npm i
npm start
```
This first installs the necessary npm dependencies.  Then it will run a webpack and serve index.js on the webpack-dev-server.

After this, simply open a browswer and enter the URL: `localhost:8080`.<br>

This will fetch the wasm module from the webpack server, instaniate it using streaming compilation, and call it, returning the stringified result in a notification popup (i.e. alert). The text displayed in the popup should match the `TALK` str contained in the `wasm/src/lib.rs`.

### Run in Rust with Wasmtime
The Rust hosting code that compiles and runs `crosstalk.wasm` can be run with the following command:
```
cargo run
```
This will use the Wasmtime crate to read the wasm file, compile it, and the make it's functions available to be called. The Rust hosting code then calls these methods and prints the stringified result to stdout. The text displayed in the console should match the `TALK` str contained in `wasm/src/lib.rs`.

### Sample Output
```
myuser:~/wasmexample/rust/crosstalk[$] cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.07s
     Running `/home/myuser/wasmexample/target/debug/crosstalk-app`
Hello World!!
myuser:~/wasmexample/rust/crosstalk[$]
```