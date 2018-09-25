//! C-friendly error handling.

use std::cell::RefCell;
use std::error::Error as StdError;
use std::ffi::CString;
use std::os::raw::c_char;
use std::ptr;

/// An error occurred.
pub const ERROR_GENERAL: u32 = 0;
pub const ERROR_UTF8: u32 = 1;
pub const ERROR_NOT_FOUND: u32 = 2;
pub const ERROR_PERMISSION_DENIED: u32 = 3;
pub const ERROR_PARSE: u32 = 4;
pub const ERROR_BUFFER_TOO_SMALL: u32 = 5;

/// Get a short description of an error's category.
#[no_mangle]
pub extern "C" fn category_name(category: u32) -> *const c_char {
    // NOTE: Update this every time a new category constant is added
    let s: &[u8] = match category {
        ERROR_GENERAL => b"General\0",
        ERROR_UTF8 => b"UTF-8 Error\0",
        ERROR_NOT_FOUND => b"Not Found\0",
        ERROR_PERMISSION_DENIED => b"Permission Denied\0",
        ERROR_PARSE => b"Parse Error\0",
        ERROR_BUFFER_TOO_SMALL => b"Buffer is too small\0",
        _ => b"Unknown\0",
    };

    s.as_ptr() as *const c_char
}

thread_local! {
    /// An `errno`-like thread-local variable which keeps track of the most
    /// recent error to occur.
    static LAST_ERROR: RefCell<Option<LastError>> = RefCell::new(None);
}

#[derive(Debug)]
struct LastError {
    pub error: Box<StdError>,
    pub c_string: CString,
    pub category: u32,
}

/// Extra information about an error.
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Error {
    /// A human-friendly error message (`null` if there wasn't one).
    pub msg: *const c_char,
    /// The general error category.
    pub category: u32,
}

impl Default for Error {
    fn default() -> Error {
        // A default `Error` for when no error has actually occurred
        Error {
            msg: ptr::null(),
            category: ERROR_GENERAL,
        }
    }
}

/// Call `set_last_error()`, with a default error category.
pub fn set_general_error<E: StdError + 'static>(err: E) {
    set_last_error(err, ERROR_GENERAL);
}

pub fn set_last_error<E: StdError + 'static>(err: E, category: u32) {
    LAST_ERROR.with(|l| {
        let c_string = CString::new(err.to_string()).unwrap_or_default();

        let new_error = LastError {
            error: Box::new(err),
            c_string,
            category,
        };

        *l.borrow_mut() = Some(new_error);
    });
}

/// Retrieve the most recent `Error` from the `LAST_ERROR` variable.
///
/// # Safety
///
/// The error message will be freed if another error occurs. It is the caller's
/// responsibility to make sure they're no longer using the `Error` before
/// calling any function which may set `LAST_ERROR`.
#[no_mangle]
pub unsafe extern "C" fn last_error() -> Error {
    LAST_ERROR.with(|l| match l.borrow().as_ref() {
        Some(err) => Error {
            msg: err.c_string.as_ptr(),
            category: err.category,
        },
        None => Error::default(),
    })
}

/// Clear the `LAST_ERROR` variable.
#[no_mangle]
pub extern "C" fn clear_error() {
    LAST_ERROR.with(|l| l.borrow_mut().take());
}
