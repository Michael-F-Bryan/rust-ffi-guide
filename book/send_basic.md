# Sending the Request

Now we can create a `Request` it'd be nice if we could actually send it and get
back a response that can be displayed to the user. This will require bindings to
the `send_request()` function in our Rust `client` module. While we're at it we
also need a wrapper which lets us access the response body (as a bunch of bytes)
and destroy it when we're done.

This chapter will cover:

- Passing arrays between languages (our response is a byte buffer)
- MOAR wrappers 
- Fleshing out the Qt GUI


## Rust FFI Bindings

The FFI bindings for `send_request()` are dead simple, we do a null pointer 
sanity check, pass the `Request` to our `send_request()` function, then box up
the response so it can be returned to the caller.


```rust
// client/src/ffi.rs

use {Response, Request, send_request};

...

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

    let response = match send_request(&*req){
        Ok(r) => r,
        Err(_) => return ptr::null_mut(),
    };

    Box::into_raw(Box::new(response))
}
```

You'll notice the funny `&*req` when calling `send_request()`. This converts a 
raw pointer into a normal borrow by dereferencing and immediately reborrowing.
The only reason this function is unsafe is because this dereferencing has the 
possibility of blowing up if the pointer passed in points to invalid memory.

The destructor for a `Response` is equally as trivial, in fact it's pretty much 
the exact same as our `Request` destructor.

```rust
/// Destroy a `Response` once you are done with it.
#[no_mangle]
pub unsafe extern "C" fn response_destroy(res: *mut Response) {
    if !res.is_null() {
        let _ = Box::from_raw(res);
    }
}
```

Getting the body response is a little trickier. We *could* give C++ a pointer
to the body and tell it how long the body is, however that introduces the
possibility that C++ will keep a reference to it after the `Response` is
destroyed. Further attempts to read the body will be a use-after-free and cause
the entire application to crash.

Instead, it'd be better to give C++ its own *copy* of the response body so it 
can be destroyed whenever it wants to. This involves a two-stage process where 
we first ask how long the body is so we can allocate a large enough buffer, then
we'll give Rust a pointer to that buffer (and its length) so the body can be 
copied across.

The length function is easiest, so lets create that one first.

```rust
// client/src/ffi.rs

use libc::{c_char, c_int};

...

/// Get the length of a `Response`'s body.
#[no_mangle]
pub unsafe extern "C" fn response_body_length(res: *mut Response) -> c_int {
    if !res.is_null() {
        return 0;
    }

    (&*res).body.len() as c_int
}
```

To copy the response body to some buffer supplied by C++ we'll want to first 
turn it from a pointer and a length into a more Rust-ic `&mut [u8]`. Luckily the
[`slice::from_raw_parts_mut()`] exists for just this purpose. We can then do the
usual length checks before using [`ptr::copy_nonoverlapping()`] to copy the 
buffer contents across.

```rust
// client/src/ffi.rs

use std::slice;

...

/// Copy the response body into a user-provided buffer, returning the number of
/// bytes copied.
///
/// If an error is encountered, this returns `-1`.
#[no_mangle]
pub unsafe extern "C" fn response_body(
    res: *mut Response,
    buffer: *mut c_char,
    length: c_int,
) -> c_int {
    if res.is_null() || buffer.is_null() {
        return -1;
    }

    let res = &*res;
    let buffer: &mut [u8] = slice::from_raw_parts_mut(buffer as *mut u8, 
                                                      length as usize);

    if buffer.len() < res.body.len() {
        return -1;
    }

    ptr::copy_nonoverlapping(res.body.as_ptr(), 
                             buffer.as_mut_ptr(), 
                             res.body.len());

    res.body.len() as c_int
}
```

In general, whenever you are wanting to pass data in the form of arrays from one
language to another, it's easiest to ask the caller to provide some buffer the
data can be written into. If you were to instead return a `Vec<u8>` or similar 
dynamically allocated type native to a particular language, that means the 
caller **must** return that object to the language so it can be free'd 
appropriately. This can get pretty error-prone and annoying after a while.

A good rule of thumb is that if a language creates something on the stack, you 
should return the object to the original language once you're done with it so 
it can be free'd properly. Failing to do this could end up either confusing the
allocator's internal bookkeeping or even result in segfaults because one 
allocator (e.g. libc's `malloc`) is trying to free memory belonging to a 
completely different allocator (e.g. Rust's `jemalloc`).

[`slice::from_raw_parts_mut()`]: https://doc.rust-lang.org/std/slice/fn.from_raw_parts_mut.html
[`ptr::copy_nonoverlapping()`]: https://doc.rust-lang.org/std/ptr/fn.copy_nonoverlapping.html