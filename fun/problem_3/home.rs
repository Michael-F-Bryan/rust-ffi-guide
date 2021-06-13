
use std::ffi::CString;
use std::env;
use std::ptr;
use std::os::raw::c_char;


#[no_mangle]
pub extern "C" fn home_directory() -> *const c_char {
    let home = match env::home_dir() {
        Some(p) => p.display().to_string(),
        None => return ptr::null(),
    };

    let c_string = match CString::new(home) {
        Ok(s) => s,
        Err(_) => return ptr::null(),
    };

    c_string.as_ptr()
}