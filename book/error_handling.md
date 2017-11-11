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

We're going to expose this error handling mechanism to C++ in two ways, there 
will be a low level C++ equivalent to `last_error_message()` which simply calls
the Rust function and does the necessary work to convert the error message into
a `std::string`.  

There will also be a more high-level `WrapperException` class which can be thrown
whenever an operation fails. This should then be caught higher up by the Qt 
application and an appropriate error message will be displayed to the user.

First we need to add `last_error_message()` to our `wrappers.hpp` header file.

```cpp
// gui/wrappers.hpp

std::string last_error_message();
```

Then we need to implement it.

```cpp
// gui/wrappers.cpp

std::string last_error_message() {
  int error_length = ffi::last_error_length();

  if (error_length == 0) {
    return std::string();
  }

  std::string msg(error_length, '\0');
  int ret = ffi::last_error_message(&msg[0], msg.length());
  if (ret != error_length) {
    // If we ever get here it's a bug
    throw new WrapperException("Fetching error message failed");
  }

  return msg;
}
```

Notice that if there was no error we return an empty `std::string` instead of 
blowing up.

We also want to define a `WrapperException` class in the `wrappers.hpp` header 
file. To make things easier, we're defining a public static helper function 
which will create a new `WrapperException` from the most recent error.

```cpp
// gui/wrappers.hpp

class WrapperException : std::exception {
public:
  WrapperException(std::string msg) : msg(msg){};
  static WrapperException last_error();
  const char * what () const throw () {
      return msg.c_str();
   }

private:
  std::string msg;
};
```

The `last_error()` method has a fairly simple definition, it fetches the last 
error message and creates a new `WrapperException` with it. If the error message
was empty then we use a default message. 

```cpp
// gui/wrappers.cpp

WrapperException WrapperException::last_error() {
  std::string msg = last_error_message();

  if (msg.length == 0) {
    return WrapperException("(no error available)");
  } else {
    return WrapperException(msg);
  }
}
```


## Integrating In The Error Handling Mechanism

Now we've got a proper error handling mechanism, we need to go back and make
sure everything uses it. This is just a case of finding all `throw`
statements in `wrappers.cpp` (using `grep` or your editor's "find" function)
and converting them to use `throw WrapperException::last_error()`.

The easiest way to check our error handling mechanism is to edit the click 
handler we've been using for testing and make it try to send a request to some
invalid URL.

```cpp
// gui/main_window.cpp

void MainWindow::onClick() {
  Request req("this is an invalid URL");
}
```

Now run the program and click the button.

```
$ ./gui/gui
...
terminate called after throwing an instance of 'WrapperException'
[1]    1016 abort (core dumped)  ./gui/gui
```

It... aborts?

This is because Qt, by default, won't try to catch any thrown exceptions, 
meaning they'll just bubble up to the top of the program and crash. 

It'd be a much better user experience if the GUI would catch all thrown 
exceptions and pop up a nice dialog box saying what went wrong.

According to [Qt's documentation on exceptions][qt], it is undefined behaviour
when a handler throws an exception. In this case it looks like the exception 
bubbled up the stack, unhandled, until it hit the program's entry point and 
triggered an abort. This isn't exactly ideal, so how about we wrap the click
hander's contents in a big try/catch block?

```cpp
// gui/main_window.cpp

void MainWindow::onClick() {
  try
  {
    Request req("this is an invalid URL");
  }
  catch (const WrapperException& e)
  {
    QMessageBox::warning(this, "Error", e.what());
  }
}
```

That's much better. Now our application can deal with errors in a sane way, and
is much more robust.


> **TODO:** Add the error handling patterns developed in this chapter to the 
> [ffi-helpers] crate.


[libgit]: https://github.com/libgit2/libgit2/blob/master/docs/error-handling.md
[ffi-helpers]: https://github.com/Michael-F-Bryan/ffi-helpers
[`thread_local!()`]: https://doc.rust-lang.org/std/macro.thread_local.html
[utf16]: https://users.rust-lang.org/t/x-post-how-do-i-integrate-rust-into-other-projects/13507/5?u=michael-f-bryan
[OsStrExt]: https://doc.rust-lang.org/std/os/windows/ffi/trait.OsStrExt.html#tymethod.encode_wide
[SO]: https://stackoverflow.com/a/4661911/7149940
[qt]: http://doc.qt.io/qt-5/exceptionsafety.html#signals-and-slots