use std::ffi::c_char;
use std::ffi::CString;

#[no_mangle]
pub fn greet() -> *mut c_char {
    let version = voicevox_core::VERSION;
    CString::new(version).unwrap().into_raw()
}
