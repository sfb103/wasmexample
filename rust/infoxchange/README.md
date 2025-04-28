# Infoxchange Wasm Example

## To build and run...

### Build the Wasm Component
The infoxchange Wasm Component has both Rust and C++ implementations.  The source can be found in `wasm-rust/src/lib.rs` and `wasm-cpp/infoxchange.cpp` respectively.<br>

Using the `wasmdevcontainer` (see instructions in the toplevel [README.md](../../README.md)), the `infoxchange.wasm` can be built by simply...<br>

__*...from Rust:*__
```
cd wasm-rust
cargo build
cd ..
```
The build config under `.cargo/config.toml` ensures this builds to the wasm32-wasip2 (i.e. wasi v0.2) target.  This will place the `infoxchange.wasm` file under `wasm-rust/target/wasm32-wasip2/debug`

__*...or from C++:*__
```
cd wasm-cpp
./build.sh
cd ..
```
The `build.sh` script contains a couple simple cmake commands that produce `infoxchange.wasm`. You can also run these lines directly instead of the `build.sh` script.

### Run with a Rust host with Wasmtime
The Rust hosting code that compiles and runs `infoxchange.wasm` can be run with the following command:
```
cargo run
```
As cloned, this will automatically run the wasm component built under `wasm-rust`. To run the component run under `wasm-cpp`, the `infoxchange.wasm -> wasm-rust/target/wasm32-wasip2/debug/infoxchange.wasm` softlink must be modified to `infoxchange.wasm -> wasm-cpp/infoxchange.wasm`. This will use the Wasmtime crate to read the wasm file, compile it, and link the component's exports and imports with the host code. The Rust host starts a component "worker" which calls the components "do-work" func in a repeated fashion until a false return signals to exit. This essentially provides a working thread context for the component. The host then waits a few seconds, just for testing purposes, and finally calls set-id on the component. After that it joins with the component's worker thread and waits for it to exit.

Inside the wasm component's do-work function, it checks to see if an id has been set yet or not. If it has not been set yet, it call set-status with "offline". Once it is set, it calls set-status with "online" and signals it has no more work to do by returning from do-work with false.

### Sample Output
```
bentlesf@docker-desktop:~/projects/wasmexample[$] cd rust/infoxchange/wasm/
bentlesf@docker-desktop:~/projects/wasmexample/rust/infoxchange/wasm[$] cargo build
    Updating crates.io index
  Downloaded ryu v1.0.18
  Downloaded wit-bindgen-rust-macro v0.38.0
  Downloaded wit-bindgen v0.38.0
  Downloaded wit-bindgen-rt v0.38.0
  Downloaded unicode-ident v1.0.14
  Downloaded 5 crates (140.2 KB) in 0.24s
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.12s
bentlesf@docker-desktop:~/projects/wasmexample/rust/infoxchange/wasm[$] cd ..
bentlesf@docker-desktop:~/projects/wasmexample/rust/infoxchange[$] cargo run
  Downloaded cap-net-ext v3.4.2
  Downloaded cap-time-ext v3.4.2
  Downloaded cap-rand v3.4.2
  ...
  Downloaded wasmtime-winch v29.0.1
  Downloaded cranelift-codegen-shared v0.116.1
  Downloaded cranelift-bitset v0.116.1
  Downloaded cranelift-codegen v0.116.1
  Downloaded 76 crates (5.8 MB) in 0.51s (largest was `cranelift-codegen` at 1.1 MB)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.59s
     Running `/home/bentlesf/projects/wasmexample/target/debug/infoxchange-app`

comp_worker() calling do_work()
do_work() hello from your rust wasm component!
do_work() id: -1 <= 0, setting status to Offline
set_status() called with status: Status::Offline

comp_worker() calling do_work()
do_work() hello from your rust wasm component!
do_work() id: -1 <= 0, setting status to Offline
set_status() called with status: Status::Offline

comp_worker() calling do_work()
do_work() hello from your rust wasm component!
do_work() id: -1 <= 0, setting status to Offline
set_status() called with status: Status::Offline

main() calling set_id: 1
set_id() setting id: 1
main() successfully set_id: 1

comp_worker() calling do_work()
do_work() hello from your rust wasm component!
do_work() id: 1 > 0, setting status to Online
set_status() called with status: Status::Online
bentlesf@docker-desktop:~/projects/wasmexample/rust/infoxchange[$]
```
