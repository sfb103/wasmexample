# Infoxchange Wasm Example

## To build and run...

### Build the Wasm Component
<<<<<<< Updated upstream
The infoxchange the Wasm Coponent is written in Rust.  The source can be found in `wasm/src/lib.rs`.<br>
=======
The infoxchange Wasm Coponent is written in Rust.  The source can be found in `wasm/src/lib.rs`.<br>
>>>>>>> Stashed changes

Using the `wasmdevcontainer` (see instructions in the toplevel [README.md](../../README.md)), the `infoxchange.wasm` can be built with simply:
```
cd wasm
cargo build
cd ..
```
The build config under `.cargo/config.toml` ensures this builds to the wasm32-wasip2 (i.e. wasi preview2) target.  This will place the `infoxchange.wasm` file under `wasm/target/wasm32-wasip2/debug`

### Run with a Rust host with Wasmtime
The Rust hosting code that compiles and runs `infoxchange.wasm` can be run with the following command:
```
cargo run
```
This will use the Wasmtime crate to read the wasm file, compile it, and link the component's exports and imports with the host code. The Rust host starts a component "worker" which calls the components "do-work" func in a repeated fashion until a false return signals to exit. This essentially provides a working thread context for the component. The host then waits a few seconds, just for testing purposes, and finally calls set-id on the component. After that it joins with the component's worker thread and waits for it to exit.

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
