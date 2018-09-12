extern crate toml;

pub mod errors;

use std::ffi::CStr;
use std::fs;
use std::io::ErrorKind;
use std::os::raw::c_char;
use std::ptr;

pub struct Value(pub toml::Value);

#[no_mangle]
pub unsafe extern "C" fn parse_file(filename: *const c_char) -> *const Value {
    assert!(!filename.is_null());

    // First we need to convert the C-style string to a `&str`
    let c_str = CStr::from_ptr(filename);
    let filename = match c_str.to_str() {
        Ok(s) => s,
        Err(e) => {
            errors::set_last_error(e, errors::ERROR_UTF8);
            return ptr::null();
        }
    };

    // then we can try to read the file's contents
    let contents = match fs::read_to_string(filename) {
        Ok(s) => s,
        Err(e) => {
            let category = match e.kind() {
                ErrorKind::NotFound => errors::ERROR_NOT_FOUND,
                ErrorKind::PermissionDenied => errors::ERROR_PERMISSION_DENIED,
                _ => errors::ERROR_GENERAL,
            };

            errors::set_last_error(e, category);
            return ptr::null();
        }
    };

    // and finally we're at the point where we can parse the thing
    let value = match toml::from_str(&contents) {
        Ok(v) => v,
        Err(e) => {
            errors::set_last_error(e, errors::ERROR_PERMISSION_DENIED);
            return ptr::null();
        }
    };

    // allocate a Value on the heap and pass ownership to the caller
    Box::into_raw(Box::new(Value(value)))
}

#[no_mangle]
pub unsafe extern "C" fn value_destroy(value: *const Value) {
    if value.is_null() {
        return;
    }

    let value = Box::from_raw(value as *mut Value);
    drop(value);
}
