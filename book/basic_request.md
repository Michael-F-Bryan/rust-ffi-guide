# Construct a Basic Request

In this step we want to construct a very simple `Request` which we can later use
to tell the `client` module to fetch http://google.com/. This requires roughly
two steps:

- Create a C interface which exposes our Rust `Request` in a way that can be used
  and manipulated from our C++ application, 
- Write a thin C++ wrapper class which gives us an abstraction over the raw 
  C-style interface, and
- Update the form so it can accept user inputs and create our `Request`.


## Creating the C Interface

First we need to add a couple small `extern "C"` functions to the Rust `client` 
module. The easiest way to do this is by creating a separate `ffi.rs` module 
to isolate all `unsafe` code to one place.

The bare minimum we need to do at this point is create a constructor and 
destructor for `Request`. The constructor can take in the target URL (as a 
`char *` string) and then fill in all the other fields with their defaults. 

Because our `Request` contains Rust-specific things like generics we need to 
hide it behind a raw pointer. This is actually pretty easy to do, you move the 
`Request` to the heap with `Box::new()`, then call `Box::into_raw()` to get a 
raw pointer to the `Request`. The dangerous part here is that the compiler will
no longer make sure the `Request` is destroyed once it goes out of scope, so 
we need to `drop` it manually.

By far the most annoying bit in the constructor will be converting a raw C 
string into a valid `Url`. This requires a couple transformations along the way,
all of which may fail, and we need to make sure this is dealt with correctly so
the program doesn't blow up at runtime.

```rust
// client/src/ffi.rs

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
```

That looks like a large chunk of code, but the vast majority is either 
documentation or error handling. You can see that we use the `CStr` type from 
the `std::ffi` module which acts as a safe wrapper around a C string. We then
convert the `CStr` to a normal `str` which may fail if the string isn't UTF-8,
returning a null pointer (using the `ptr::null_mut()` helper) to indicate 
failure. 

Converting from a `str` to a `Url` is almost identical.

Finally we can create the `Request` using `Request::new()`, then box it and 
return a raw pointer to the `Request` to the caller.

We also inserted a check for null pointers at the top as a bit of a sanity 
check.

The destructor is significantly easier to write. All we need to do is accept
a raw pointer to some `Request`, convert it back to a `Box` with
`Box::from_raw()`, then let the `Box<Request>` fall out of scope and be
destroyed like normal. 

```rust
// client/src/ffi.rs

/// Destroy a `Request` once you are done with it.
#[no_mangle]
pub unsafe extern "C" fn request_destroy(req: *mut Request) {
    if !req.is_null() {
        let _ = Box::from_raw(req);
    }
}
```

You will notice that both functions were prefixed with `request_`. This is a 
useful convention to use to indicate that the function "belongs" to some type,
conceptual the equivalent of a normal method.
