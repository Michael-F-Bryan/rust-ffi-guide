use std::ffi::CString;
use std::os::raw::c_char;


extern "C" {
    fn say_hello(name: *const c_char);
}

fn main() {
    let me = CString::new("World").unwrap();

    unsafe {
        say_hello(me.as_ptr());
    }
}
