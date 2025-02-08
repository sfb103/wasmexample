use exports::wasmexample::infoxchange::*;
use wasmexample::infoxchange::status_holder::{self, Status};
use wit_bindgen::generate;
use std::sync::Mutex;

generate!("infoxchange" in "../wit");

static ID: Mutex<i32> = Mutex::new(-1);

struct XchangeComponent;

impl id_holder::Guest for XchangeComponent {
    fn set_id(id: i32) -> Result<(), String> {
        println!( "set_id() setting id: {}", id );
        *ID.lock().unwrap() = id;
        Ok(())
    }
    fn get_id() -> Result<i32, String> {
        Ok(*ID.lock().unwrap())
    }
}

impl worker::Guest for XchangeComponent {
    fn do_work() -> bool {
        let id = *ID.lock().unwrap();
        if 0 < id {
            println!( "do_work() id: {} > 0, setting status to Online", id );
            status_holder::set_status( Status::Online ).unwrap();
            return false; // Done.
        } else {
            println!( "do_work() id: {} <= 0, setting status to Offline", id );
            status_holder::set_status( Status::Offline ).unwrap();
            return true; // Keep going.
        }
    }
}

export!(XchangeComponent);