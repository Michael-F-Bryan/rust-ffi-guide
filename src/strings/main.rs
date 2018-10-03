use std::ffi::CString;
use std::os::raw::c_char;

extern "C" {
    fn print(msg: *const c_char);
}

fn main() {
    let msg = "Hello, World!";
    let c_style_msg = CString::new(msg)
        .expect("The string doesn't contain any interior null bytes");

    unsafe {
        print(c_style_msg.as_ptr());
    }
}
