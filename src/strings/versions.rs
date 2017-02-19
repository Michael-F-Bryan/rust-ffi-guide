use std::io::Write;
use std::slice;
use std::ffi::CString;
use std::os::raw::{c_int, c_char};


#[no_mangle]
pub extern "C" fn version() -> *mut c_char {
    CString::new("0.1.0").unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn string_destroy(s: *mut c_char) {
    if !s.is_null() {
        unsafe {
            CString::from_raw(s);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn version_with_buffer(buf: *mut u8, len: c_int) -> c_int {
    if buf.is_null() {
        return -1;
    }

    let mut buffer = slice::from_raw_parts_mut(buf, len as usize);
    let version_number = CString::new("0.1.0").unwrap();
    buffer.write(version_number.as_bytes_with_nul())
        .map(|n| n as c_int)
        .unwrap_or(-1)
}
