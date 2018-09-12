extern crate toml;

pub mod errors;

use std::error::Error as StdError;
use std::ffi::CStr;
use std::fmt::{self, Display, Formatter};
use std::fs;
use std::io::ErrorKind;
use std::os::raw::c_char;
use std::ptr;
use std::slice;

/// A newtype wrapper around a `toml::Value` so `cbindgen` will declare it as
/// an opaque type in the generated header file.
pub struct Value(pub toml::Value);

/// A helper macro for converting a C-style string (`*const c_char`) into a
/// proper Rust `&str`.
macro_rules! cstr {
    ($ptr:expr) => {{
        match CStr::from_ptr($ptr).to_str() {
            Ok(s) => s,
            Err(e) => {
                errors::set_last_error(e, errors::ERROR_UTF8);
                return ptr::null();
            }
        }
    }};
}

/// Parse a TOML file into a `Value`.
///
/// # Note
///
/// Don't forget to free the `Value` once you're done with it using
/// `value_destroy()`.
#[no_mangle]
pub unsafe extern "C" fn parse_file(filename: *const c_char) -> *const Value {
    let filename = cstr!(filename);

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

/// Destroy a `Value` once it's no longer needed.
#[no_mangle]
pub unsafe extern "C" fn value_destroy(value: *const Value) {
    if value.is_null() {
        return;
    }

    let value = Box::from_raw(value as *mut Value);
    drop(value);
}

/// Get the `element` item out of a `Value`.
#[no_mangle]
pub unsafe extern "C" fn value_get(
    value: *const Value,
    element: *const c_char,
) -> *const Value {
    let value = &(*value).0;
    let index = cstr!(element);

    match value.get(index) {
        Some(v) => {
            // Using pointer casts like this is kinda hacky. The only reason
            // we can get away with it is because `Value` is a newtype wrapper
            // around a `toml::Value` and will have the same representation in
            // memory.
            v as *const toml::Value as *const Value
        }
        None => ptr::null(),
    }
}

/// If the TOML node is an integer, extract its value.
#[no_mangle]
pub unsafe extern "C" fn value_as_integer(
    value: *const Value,
    out: *mut i64,
) -> bool {
    match (*value).0.as_integer() {
        Some(i) => {
            *out = i;
            true
        }
        _ => false,
    }
}

/// Extract the inner value if this TOML node is a float.
#[no_mangle]
pub unsafe extern "C" fn value_as_float(
    value: *const Value,
    out: *mut f64,
) -> bool {
    match (*value).0.as_float() {
        Some(f) => {
            *out = f;
            true
        }
        _ => false,
    }
}

/// If the TOML node is a string, write it to the provided buffer as a
/// null-terminated UTF-8 string, returning the number of bytes written.
///
/// A return value of `-1` means the buffer wasn't large enough. If the TOML
/// node isn't a string, no bytes will be written.
///
/// # Note
///
/// To help determine an appropriate buffer size, calling `value_as_str()` with
/// a `null` buffer will return the number of bytes required without writing
/// anything.
#[no_mangle]
pub unsafe extern "C" fn value_as_str(
    value: *const Value,
    buffer: *mut c_char,
    len: i32,
) -> i32 {
    let value = &(*value).0;

    let s = match value.as_str() {
        Some(s) => s,
        None => return 0,
    };

    let bytes_required = s.len() + 1;

    // Let the caller know how big their buffer needs to be
    if buffer.is_null() {
        return bytes_required as i32;
    }

    let buffer = slice::from_raw_parts_mut(buffer as *mut u8, len as usize);

    if buffer.len() < bytes_required {
        errors::set_last_error(
            InsufficientSpace {
                required: bytes_required,
                found: buffer.len(),
            },
            errors::ERROR_BUFFER_TOO_SMALL,
        );
        return -1;
    }

    // Actually copy the string across
    buffer[..bytes_required - 1].copy_from_slice(s.as_bytes());
    // Don't forget the null terminator
    buffer[bytes_required - 1] = 0;

    bytes_required as i32
}

#[derive(Debug, Clone, Copy)]
struct InsufficientSpace {
    pub required: usize,
    pub found: usize,
}

impl StdError for InsufficientSpace {}

impl Display for InsufficientSpace {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "Insufficent space. {} bytes required but received {}",
            self.required, self.found
        )
    }
}
