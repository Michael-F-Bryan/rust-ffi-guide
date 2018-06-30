use std::ffi::CString;
use std::os::raw::{c_char, c_int};

extern "C" {
    fn puts(data: *const c_char) -> c_int;
}

fn main() {
    let hello_world = CString::new("Hello World!").unwrap();

    unsafe {
        puts(hello_world.as_ptr());
    }
}
