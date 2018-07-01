use std::ffi::CStr;
use std::os::raw::c_char;

#[no_mangle]
pub unsafe extern "C" fn print(msg: *const c_char) {
    let msg = CStr::from_ptr(msg);
    let as_str = msg.to_str().expect("The message is always valid UTF8");

    println!("{}", as_str);
}
