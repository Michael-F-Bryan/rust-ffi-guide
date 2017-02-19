use std::ffi::CStr;
use std::os::raw::{c_char, c_int};


fn count_chars(s: &str) -> usize {
    s.chars().count()
}


#[no_mangle]
pub unsafe extern "C" fn count_characters(s: *const c_char) -> c_int {
    let str_slice = CStr::from_ptr(s);
    count_chars(str_slice.to_str().unwrap()) as c_int
}
