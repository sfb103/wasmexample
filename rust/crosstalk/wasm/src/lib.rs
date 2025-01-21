use std::ffi::CString;
use std::os::raw::c_char;

static TALK: &'static str = "Hello World!!";

#[no_mangle]
pub fn get_talk_ptr() -> *mut c_char {
    let talk_ptr = CString::new(TALK).unwrap();
    talk_ptr.into_raw()
}

#[no_mangle]
pub fn get_talk_len() -> usize {
    TALK.len()
}

