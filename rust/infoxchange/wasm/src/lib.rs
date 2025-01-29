use crate::exports::wasmexample::infoxchange::id_holder::Guest;
use wit_bindgen::generate;

generate!("infoxchange" in "../wit");

struct XchangeComponent;

impl Guest for XchangeComponent {
    fn set_id(_: u32) -> Result<(), String> { todo!() }
    fn get_id() -> Result<u32, String> { todo!() }
}

export!(XchangeComponent);