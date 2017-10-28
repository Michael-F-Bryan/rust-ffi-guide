//! The foreign function interface which exposes this library to non-Rust 
//! languages.

use std::ffi::CStr;
use std::ptr;
use libc::c_char;
use reqwest::{Url, Method};

use Request;


/// Construct a new `Request` which will target the provided URL and fill out 
/// all other fields with their defaults.
/// 
/// # Note
/// 
/// If the string passed in isn't a valid URL this will return a null pointer.
/// 
/// # Safety
/// 
/// Make sure you destroy the request with [`request_destroy()`] once you are
/// done with it.
/// 
/// [`request_destroy()`]: fn.request_destroy.html
#[no_mangle]
pub unsafe extern "C" fn request_create(url: *const c_char) -> *mut Request {
    let raw = CStr::from_ptr(url);

    let url_as_str = match raw.to_str() {
        Ok(s) => s,
        Err(_) => return ptr::null_mut(),
    };

    let parsed_url = match Url::parse(url_as_str) {
        Ok(u) => u,
        Err(_) => return ptr::null_mut(),
    };

    let req = Request::new(parsed_url, Method::Get);
    Box::into_raw(Box::new(req))
}

/// Destroy a `Request` once you are done with it.
#[no_mangle]
pub unsafe extern "C" fn request_destroy(req: *mut Request) {
    if !req.is_null() {
        let _ = Box::from_raw(req);
    }
}