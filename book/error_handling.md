# Better Error Handling

So far whenever something goes wrong we've just returned a null pointer to 
indicate failure... This isn't overly ideal. Instead, it'd be nice if we could
get some context behind an error and possibly present a nice friendly message 
to the user.

A very powerful error handling mechanism in C-style programs (technically this 
is one because our FFI bindings export a C interface) is modelled on `errno`.

This employs a thread-local variable which holds the most recent error as
well as some convenience functions for getting/clearing this variable. The
theory is if a function fails then it should return an "obviously invalid"
value (typically `-1` or `0` when returning integers or `null` for pointers).
The user can then check for this and consult the most recent error for more
information. Of course that means all fallible operations *must* update the
most recent error if they fail and that you *must* check the returned value of
any fallible operation. 

While it isn't as elegant as Rust's monad-style `Result<T, E>` with `?` and the
various combinators, it actually turns out to be a pretty solid error handling
technique in practice.

> **Note:** It is **highly recommended** to have a skim through libgit2's 
> [error handling docs][libgit]. The error handling mechanism we'll be using 
> takes a lot of inspiration from `libgit2`.

## Working With Errors

We'll start off by defining a thread-local static variable with the 
[`thread_local!()`] macro and put it in the `ffi` module.

```rust
// client/src/ffi.rs

thread_local!{
    static LAST_ERROR: RefCell<Option<Box<Error>>> = RefCell::new(None);
}
```

Notice that we haven't declared the error value public, this is so people are
forced to access the error via getter and setter functions.

```rust
// client/src/ffi.rs

/// Update the most recent error, clearing whatever may have been there before.
pub fn update_last_error<E: Error + 'static>(err: E) {
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
pub fn take_last_error() -> Option<Box<Error>> {
    LAST_ERROR.with(|prev| prev.borrow_mut().take())
}
```

Neither of these are terribly interesting once you look at the 
[`thread_local!()`] macro's documentation. Notice that the actual type we're 
using needs to be `RefCell<Option<_>>` so we can have both interior mutability, 
*and* represent the fact that there may not have been any recent errors. It's
annoying, but luckily due to the API's design the complexity won't leak into 
client code.

While the getters and setters we currently have are quite powerful, it's still 
not possible to use them outside of Rust. To remedy this we're going to add a 
function that is callable from C and will give the caller the most recent error
message. 

The idea is the caller will give us a buffer to write the string into. This
part can be a little tricky because we have a couple edge cases and ergonomics 
issues to deal with.

For example, if the caller passing a buffer which isn't big enough to hold the 
error message we should return an error, obviously. However how does the caller
know what a reasonable buffer size is to begin with? Making them guess isn't 
exactly a practical solution, so it'd be nice if we included a mechanism for 
calculating the error message's length without consuming the error message 
itself.

To deal with this we add an extra `last_error_lengt()` function.

```rust
// client/src/ffi.rs

/// Calculate the number of bytes in the last error's error message **not**
/// including any trailing `null` characters.
#[no_mangle]
pub extern "C" fn last_error_length() -> c_int {
    LAST_ERROR.with(|prev| match *prev.borrow() {
        Some(ref err) => err.to_string().len() as c_int,
        None => 0,
    })
}
```

The second issue is more problematic. For all unix-based systems, the string 
type used pretty much ubiquitously is UTF-8, meaning we should be able to copy
a Rust `String`'s contents directly into the provided buffer without any issues.
*However*, the "unicode" string type most commonly used on Windows is **not** 
UTF-8. Instead they use UTF-16 (well... technically [it's not even valid 
UTF-16][utf16]) which is completely incompatible with UTF-8. 

Therefore on Windows if we want to be correct we should convert the `String`
representation of an error message into a native Windows string with 
`encode_wide()` from [`std::os::windows::ffi::OsStrExt`][OsStrExt] and copy that
into a `&mut [u16]` buffer (not `&mut [u8]`!) that the user gives to us... Eww.

There's no easy way to get around this without a bunch of conditional 
compilation (`#[cfg]`) and adding a lot of complexity to the implementation, 
therefore we're going to cheat and say it's the caller's responsibility to deal
with any UTF-8/UTF-16 conversions.

```rust
// client/src/ffi.rs

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
```

Our `last_error_message()` function turned out to be rather long, although most
of it is taken up by checking for errors and edge cases.

> **Note:** Notice that we're writing into a buffer provided by the caller 
> instead of returning a Rust `String`. This makes memory management a lot 
> easier because the caller can clean up the buffer like normal instead of 
> needing to remember to call some Rust destructor afterwards.
>
> Writing into borrowed buffers instead of returning an owned object is a common
> pattern when doing FFI. It helps simplify things and avoid errors due to 
> object lifetimes (in general, not just in the Rust sense of the word) and 
> forgetting to call destructors.


## C++ Error Bindings


> **TODO:** Add the error handling patterns developed in this chapter to the 
> [ffi-helpers] crate.


[libgit]: https://github.com/libgit2/libgit2/blob/master/docs/error-handling.md
[ffi-helpers]: https://github.com/Michael-F-Bryan/ffi-helpers
[`thread_local!()`]: https://doc.rust-lang.org/std/macro.thread_local.html
[utf16]: https://users.rust-lang.org/t/x-post-how-do-i-integrate-rust-into-other-projects/13507/5?u=michael-f-bryan
[OsStrExt]: https://doc.rust-lang.org/std/os/windows/ffi/trait.OsStrExt.html#tymethod.encode_wide