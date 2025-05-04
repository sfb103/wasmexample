use std::sync::atomic::{AtomicI32, Ordering};
use exports::wasmexample::infoxchange::*;
use wasmexample::infoxchange::status_holder::{self, Status};
use wit_bindgen::generate;

//generate!("infoxchange" in "../wit");
generate!({
    world: "infoxchange",
    path: "../wit",
    async: true,
});

static ID: AtomicI32 = AtomicI32::new(-1);

struct XchangeComponent;

impl id_holder::Guest for XchangeComponent {
    async fn set_id(id: i32) -> Result<(), String> {
        println!( "set_id() setting id: {}", id );
        ID.store(id,Ordering::Release);
        Ok(())
    }
    async fn get_id() -> Result<i32, String> {
        Ok(ID.load(Ordering::Acquire))
    }
}

impl worker::Guest for XchangeComponent {
    async fn do_work() -> bool {
        println!("do_work() hello from your rust wasm component!");
        let id = ID.load(Ordering::Acquire);
        if id < 0 {
            println!( "do_work() id: {} <= 0, setting status to Offline", id );
            status_holder::set_status( Status::Offline ).await.unwrap();
            return true; // Keep going.            
        } else {
            println!( "do_work() id: {} > 0, setting status to Online", id );
            status_holder::set_status( Status::Online ).await.unwrap();
            return false; // Done.
        }
    }
}

export!(XchangeComponent);