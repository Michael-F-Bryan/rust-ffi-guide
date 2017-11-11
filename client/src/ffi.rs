//! The foreign function interface which exposes this library to non-Rust
//! languages.

use std::ffi::CStr;
use std::ptr;
use std::slice;
use std::error::Error as StdError;
use std::cell::RefCell;
use libc::{c_char, c_int, size_t};
use reqwest::{Method, Url};

use {send_request, PluginManager, Request, Response};
use errors::*;


thread_local!{
    static LAST_ERROR: RefCell<Option<Box<StdError>>> = RefCell::new(None);
}

/// Update the most recent error, clearing whatever may have been there before.
pub fn update_last_error<E: StdError + 'static>(err: E) {
    error!("Setting LAST_ERROR: {}", err);

    {
        // Print a pseudo-backtrace for this error, following back each error's
        // cause until we reach the root error.
        let mut cause = err.cause();
        while let Some(parent_err) = cause {
            warn!("Caused by: {}", parent_err);
            cause = parent_err.cause();
        }
    }

    LAST_ERROR.with(|prev| {
        *prev.borrow_mut() = Some(Box::new(err));
    });
}

/// Retrieve the most recent error, clearing it in the process.
pub fn take_last_error() -> Option<Box<StdError>> {
    LAST_ERROR.with(|prev| prev.borrow_mut().take())
}

/// Calculate the number of bytes in the last error's error message **not**
/// including any trailing `null` characters.
#[no_mangle]
pub extern "C" fn last_error_length() -> c_int {
    LAST_ERROR.with(|prev| match *prev.borrow() {
        Some(ref err) => err.to_string().len() as c_int + 1,
        None => 0,
    })
}

/// Write the most recent error message into a caller-provided buffer as a UTF-8
/// string, returning the number of bytes written.
///
/// # Note
///
/// This writes a **UTF-8** string into the buffer. Windows users may need to
/// convert it to a UTF-16 "unicode" afterwards.
///
/// If there are no recent errors then this returns `0` (because we wrote 0
/// bytes). `-1` is returned if there are any errors, for example when passed a
/// null pointer or a buffer of insufficient size.
#[no_mangle]
pub unsafe extern "C" fn last_error_message(buffer: *mut c_char, length: c_int) -> c_int {
    if buffer.is_null() {
        warn!("Null pointer passed into last_error_message() as the buffer");
        return -1;
    }

    let last_error = match take_last_error() {
        Some(err) => err,
        None => return 0,
    };

    let error_message = last_error.to_string();

    let buffer = slice::from_raw_parts_mut(buffer as *mut u8, length as usize);

    if error_message.len() >= buffer.len() {
        warn!("Buffer provided for writing the last error message is too small.");
        warn!(
            "Expected at least {} bytes but got {}",
            error_message.len() + 1,
            buffer.len()
        );
        return -1;
    }

    ptr::copy_nonoverlapping(
        error_message.as_ptr(),
        buffer.as_mut_ptr(),
        error_message.len(),
    );

    // Add a trailing null so people using the string as a `char *` don't
    // accidentally read into garbage.
    buffer[error_message.len()] = 0;

    error_message.len() as c_int
}

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
        let err = Error::from("No URL provided");
        update_last_error(err);
        return ptr::null_mut();
    }

    let raw = CStr::from_ptr(url);

    let url_as_str = match raw.to_str() {
        Ok(s) => s,
        Err(e) => {
            let err = Error::with_chain(e, "Unable to convert URL to a UTF-8 string");
            update_last_error(err);
            return ptr::null_mut();
        }
    };

    let parsed_url = match Url::parse(url_as_str) {
        Ok(u) => u,
        Err(e) => {
            let err = Error::with_chain(e, "Unable to parse the URL");
            update_last_error(err);
            return ptr::null_mut();
        }
    };

    let req = Request::new(parsed_url, Method::Get);
    trace!("Created Request, {:?}", req);
    Box::into_raw(Box::new(req))
}

/// Destroy a `Request` once you are done with it.
#[no_mangle]
pub unsafe extern "C" fn request_destroy(req: *mut Request) {
    if !req.is_null() {
        let req = Box::from_raw(req);
        trace!("Destroying Request, {:?}", req);
        drop(req);
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
        update_last_error(Error::from("Received null pointer"));
        return ptr::null_mut();
    }

    let req = &*req;

    let response = match send_request(req) {
        Ok(r) => r,
        Err(e) => {
            update_last_error(Error::with_chain(e, "Sending request failed."));
            return ptr::null_mut();
        }
    };

    debug!("Received Response");
    trace!("{:?}", response);

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
        update_last_error(Error::from("Null pointer passed to response_body_length()"));
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
        update_last_error(Error::from("Null pointer passed to response_body()"));
        return -1;
    }

    let res = &*res;
    let buffer: &mut [u8] = slice::from_raw_parts_mut(buffer as *mut u8, length as usize);

    if buffer.len() < res.body.len() {
        update_last_error(Error::from("Buffer is an insufficient length"));
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
pub unsafe extern "C" fn plugin_manager_load_plugin(
    pm: *mut PluginManager,
    filename: *const c_char,
) -> c_int {
    let pm = &mut *pm;
    let filename = CStr::from_ptr(filename);
    let filename_as_str = match filename.to_str() {
        Ok(s) => s,
        Err(e) => {
            update_last_error(Error::with_chain(
                e,
                "Unable to convert the plugin filename to UTF-8",
            ));
            return -1;
        }
    };

    debug!("Loading plugin, {:?}", filename_as_str);

    // TODO: proper error handling and catch_unwind
    match pm.load_plugin(filename_as_str) {
        Ok(_) => 0,
        Err(e) => {
            update_last_error(Error::with_chain(e, "Loading plugin failed"));
            -1
        }
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
pub unsafe extern "C" fn plugin_manager_post_receive(
    pm: *mut PluginManager,
    response: *mut Response,
) {
    let pm = &mut *pm;
    let response = &mut *response;
    pm.post_receive(response);
}
