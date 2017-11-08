# Constructing a Basic Request

In this step we want to construct a very simple `Request` which we can later use
to tell the `client` module to fetch http://google.com/. This requires roughly
three steps:

- Create a C interface which exposes our Rust `Request` in a way that can be used
  and manipulated from our C++ application, 
- Write a thin C++ wrapper class which gives us an abstraction over the raw 
  C-style interface, and
- Update the form so it can accept user inputs and create our `Request`.

We'll also touch on the following topics:

- Exposing a FFI interface in Rust
- Calling Rust functions from C++
- Passing strings back and forth across the FFI barrier
- Passing an opaque Rust struct to C++ and ensuring it gets free'd at the 
  correct time


## Creating the C Interface

First we need to add a couple small `extern "C"` functions to the Rust `client` 
module. The easiest way to do this is by creating a separate `ffi.rs` module 
to isolate all `unsafe` code to one place.

The bare minimum we need to do at this point is create a constructor and 
destructor for `Request`. The constructor can take in the target URL (as a 
`char *` string) and then fill in all the other fields with their defaults. 

Because our `Request` contains Rust-specific things like generics we need to 
hide it behind a raw pointer. This is actually pretty easy to do; you move the
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
documentation for indicating constraints which need to be maintained, or
error handling. You can see that we use the `CStr` type from the `std::ffi`
module which acts as a safe wrapper around a C string. We then convert the
`CStr` to a normal `str` which may fail if the string isn't UTF-8, returning
a null pointer (using the `ptr::null_mut()` helper) to indicate failure.

Converting from a `str` to a `Url` is almost identical.

Finally we can create the `Request` using `Request::new()`, then box it and 
return a raw pointer to the `Request` to the caller.

We also inserted a check for null pointers at the top as a bit of a sanity 
check.

The destructor is significantly easier to write. All we need to do is accept
a raw pointer to some `Request`, convert it back to a `Box` with
`Box::from_raw()`, then the `Box<Request>` can either be explicitly dropped or
allowed to fall out of scope to destroy it like normal. 

```rust
// client/src/ffi.rs

/// Destroy a `Request` once you are done with it.
#[no_mangle]
pub unsafe extern "C" fn request_destroy(req: *mut Request) {
    if !req.is_null() {
        drop(Box::from_raw(req));
    }
}
```

You will notice that both functions were prefixed with `request_`. This is a 
common convention used to indicate that the function "belongs" to some type,
conceptual the equivalent of a normal method.


## The C++ Wrapper

Although we *could* use the raw C-style FFI bindings throughout this 
application, that usually ends up with non-idiomatic and more error-prone code.
Instead, it'd be really nice if we could use C++'s destructors to ensure memory
gets free'd appropriately, as well as the ability to use methods to group 
functions logically.

We'll put the definition for these wrappers in their own `wrappers.hpp` header 
file so the main application only uses the public interface. For now we'll 
only create a constructor and destructor.


```cpp
// gui/wrappers.hpp

#include <string>

class Request {
public:
  Request(const std::string);
  ~Request();

private:
  void *raw;
};

```

The implementation is equally as trivial. It just declares that there are a 
couple external functions *somewhere* that we want to use, and the linker can 
resolve them for us at link time.

```cpp
// gui/wrappers.cpp

#include "wrappers.hpp"
#include <string>

extern "C" {
void *request_create(const char *);
void request_destroy(void *);
}

Request::~Request() { request_destroy(raw); }

Request::Request(const std::string url) {
  raw = request_create(url.c_str());
  if (raw == nullptr) {
    throw "Invalid URL";
  }
```

> **Note:** You may have noticed that even though `request_create()` accepts a
> raw C-style string (`char *`), the `Request` wrapper's constructor takes in a
> normal `std::string`.
>
> This is what we were talking about earlier about wrappers being more idiomatic
> and easier to use. It may sound like a trivial thing now, but in real projects
> where the application is much more complex and has many moving parts, an 
> idiomatic class is much less likely to introduce bugs because the users won't 
> need to read through a load of source code to see how to use it. Everything 
> will *Just Work*.

We will also need to update the `CMakeLists.txt` file for our `gui/` directory
so that these new files are compiled in.


```cmake
# gui/CMakeLists.txt

set(CMAKE_CXX_STANDARD 14)

set(CMAKE_AUTOMOC ON)
set(CMAKE_AUTOUIC ON)
set(CMAKE_AUTORCC ON)
set(CMAKE_INCLUDE_CURRENT_DIR ON)
find_package(Qt5Widgets)

set(SOURCE main_window.cpp main_window.hpp wrappers.cpp wrappers.hpp main.cpp)

add_executable(gui ${SOURCE})
target_link_libraries(gui Qt5::Widgets ${CMAKE_BINARY_DIR}/libclient.so)
add_dependencies(gui client)
```

As a sanity check to make sure everything is working and that memory is being
free'd properly. By far the easiest way to do this is to update the GUI's
click handler to create a new C++ `Request` and add a bunch of print
statements to `ffi.rs` to see what actually gets called.

The updated `main_window.cpp`:

```cpp
// gui/main_window.cpp

#include "main_window.hpp"
#include "wrappers.hpp"
#include <iostream>

void MainWindow::onClick() {
  std::cout << "Creating the request" << std::endl;
  Request req("https://google.com/");
  std::cout << "Request created in C++" << std::endl;
}

...
```

And `ffi.rs`:

```rust
#[no_mangle]
pub unsafe extern "C" fn request_create(url: *const c_char) -> *mut Request {
    ...

    println!("Request created in Rust: {}", url_as_str);
    Box::into_raw(Box::new(req))
}

...

#[no_mangle]
pub unsafe extern "C" fn request_destroy(req: *mut Request) {
    if !req.is_null() {
        println!("Request was destroyed");
        drop(Box::from_raw(req));
    }
}
```

If you compile and run the GUI program then click our button you should see 
something like the following printed to stdout.

```
$ cmake .. && make
$ ./gui/gui
Creating the request
Request created in Rust: https://google.com/
Request created in C++
Request was destroyed
Creating the request
Request created in Rust: https://google.com/
Request created in C++
Request was destroyed
```

This tells us that the request is being constructed and that the URL was
passed to Rust correctly, and that it is also being destroyed when the C++
`Request` falls out of scope. 

This little test also shows how easy it is to interoperate between C++ and Rust.
Sure, it may be a little annoying to create wrappers and FFI bindings but 
looking at it differently, this allows us to create a very definitive line, 
separating the GUI code from the HTTP client module.
