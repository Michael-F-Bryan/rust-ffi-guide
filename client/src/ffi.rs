//! The foreign function interface which exposes this library to non-Rust
//! languages.

use std::ffi::CStr;
use std::ptr;
use std::slice;
use libc::{c_char, c_int, size_t};
use reqwest::{Method, Url};

use {send_request, Request, Response, PluginManager};


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
    if url.is_null() {
        return ptr::null_mut();
    }

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
        drop(Box::from_raw(req));
    }
}

/// Take a reference to a `Request` and execute it, getting back the server's
/// response.
///
/// If something goes wrong, this will return a null pointer. Don't forget to
/// destroy the `Response` once you are done with it!
#[no_mangle]
pub unsafe extern "C" fn request_send(req: *const Request) -> *mut Response {
    if req.is_null() {
        return ptr::null_mut();
    }

    let response = match send_request(&*req) {
        Ok(r) => r,
        Err(_) => return ptr::null_mut(),
    };

    Box::into_raw(Box::new(response))
}

/// Destroy a `Response` once you are done with it.
#[no_mangle]
pub unsafe extern "C" fn response_destroy(res: *mut Response) {
    if !res.is_null() {
        drop(Box::from_raw(res));
    }
}

/// Get the length of a `Response`'s body.
#[no_mangle]
pub unsafe extern "C" fn response_body_length(res: *const Response) -> size_t {
    if res.is_null() {
        return 0;
    }

    (&*res).body.len() as size_t
}

/// Copy the response body into a user-provided buffer, returning the number of
/// bytes copied.
///
/// If an error is encountered, this returns `-1`.
#[no_mangle]
pub unsafe extern "C" fn response_body(
    res: *const Response,
    buffer: *mut c_char,
    length: size_t,
) -> c_int {
    if res.is_null() || buffer.is_null() {
        return -1;
    }

    let res = &*res;
    let buffer: &mut [u8] = slice::from_raw_parts_mut(buffer as *mut u8, length as usize);

    if buffer.len() < res.body.len() {
        return -1;
    }

    ptr::copy_nonoverlapping(res.body.as_ptr(), buffer.as_mut_ptr(), res.body.len());

    res.body.len() as c_int
}

/// Create a new `PluginManager`.
#[no_mangle]
pub extern "C" fn plugin_manager_new() -> *mut PluginManager {
    Box::into_raw(Box::new(PluginManager::new()))
}

/// Destroy a `PluginManager` once you are done with it.
#[no_mangle]
pub unsafe extern "C" fn plugin_manager_destroy(pm: *mut PluginManager) {
    if !pm.is_null() {
        let pm = Box::from_raw(pm);
        drop(pm);
    }
}

#[no_mangle]
pub unsafe extern "C" fn plugin_manager_load_plugin(pm: *mut PluginManager, filename: *const c_char) -> c_int {
    let pm = &mut *pm;
    let filename = CStr::from_ptr(filename);
    let filename_as_str = match filename.to_str() {
        Ok(s) => s,
        Err(_) => {
            // TODO: proper error handling
            return -1;
        }
    };

    // TODO: proper error handling and catch_unwind
    match pm.load_plugin(filename_as_str) {
        Ok(_) => 0,
        Err(_) => -1,
    }
}

/// Unload all loaded plugins.
#[no_mangle]
pub unsafe extern "C" fn plugin_manager_unload(pm: *mut PluginManager) {
    let pm = &mut *pm;
    pm.unload();
}

/// Fire the `pre_send` plugin hooks.
#[no_mangle]
pub unsafe extern "C" fn plugin_manager_pre_send(pm: *mut PluginManager, request: *mut Request) {
    let pm = &mut *pm;
    let request = &mut *request;
    pm.pre_send(request);
}

/// Fire the `post_receive` plugin hooks.
#[no_mangle]
pub unsafe extern "C" fn plugin_manager_post_receive(pm: *mut PluginManager, response: *mut Response) {
    let pm = &mut *pm;
    let response = &mut *response;
    pm.post_receive(response);
}
