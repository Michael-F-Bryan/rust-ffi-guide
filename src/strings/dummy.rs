use std::ffi::CStr;
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn print(s: *const c_char) {
    let s = unsafe { CStr::from_ptr(s) };

    match s.to_str() {
        Ok(rust_str) => println!("Printing \"{}\" from Rust", rust_str),
        Err(e) => eprintln!("Error interpreting the C string as UTF8, {}", e),
    }
}
