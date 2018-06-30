# Create Bindings for a C Library

Our first example will be creating a Rust interface to a common C library,
`libmagic`. This is the library which is used by the `file` command for
identifying a file's type by inspecting its contents (e.g. looking for *magic*
numbers).

The process of creating a Rust interface to a foreign library is actually quite
common. If you've done much programming with Rust you will have probably seen
several `*-sys` crates scroll past as `cargo` is compiling your project. When
creating bindings for a foreign library, you typically break it up into two
crates:

- A `*-sys` (in this case `magic-sys`) contains Rust declarations for the raw C
  functions and types from `magic.h`
- A more Rust-ic crate (`magic`) which exports the functionality from
  `magic-sys` with a safer and more idiomatic interface

## Creating The *-sys Crate

The `*-sys` crate has two main purposes. It contains the various `extern` 
declarations for `libmagic`, and it needs to tell `rustc` how to link to the
library itself. Often a `*-sys` crate will try to compile the library from
source if it isn't detected on the system, but we'll skip that step for 
simpilicity.

Luckily we can leverage [bindgen] for a lot of this section.

> **Note:** If you aren't already familiar with `bindgen`, you may want to check
> out [their tutorial][bg-tut].

First we'll need a new `crate`

```console
$ cargo new --lib magic-sys
     Created library `magic-sys` project
```

Next we need to create our `wrapper.h` for bindgen to read.

```console
$ cd magic-sys
$ cat wrapper.h 
#include <magic.h>
```

And finally we can tell `bindgen` to create bindings for all functions and 
constants in `libmagic` (matching the regex `magic_.*` and `MAGIC_.*`, 
respectively), as well as anything they depend on.

```console
$ bindgen wrapper.h --whitelist-function 'magic_.*' --whitelist-var 'MAGIC_.*' > src/lib.rs
```

To make sure `rustc` links in `libmagic` we'll need to create a [build script].

```console
$ echo 'fn main() { println!("rustc-link-lib=magic"); }' > build.rs
```

From the documentation on build scripts:

> `rustc-link-lib=[KIND=]NAME` indicates that the specified value is a library
> name and should be passed to the compiler as a `-l` flag. The optional `KIND` 
> can be one of `static`, `dylib` (the default), or `framework`, see 
> `rustc --help` for more details.

We also need to update `magic-sys`'s `Cargo.toml` file to tell `cargo` about our
build script and let it know we link to `libmagic` (this prevents accidentally
linking to two versions of `magic-sys` because then bad things can happen).

```console
$ git diff Cargo.toml
 [package]
 name = "magic-sys"
 version = "0.1.0"
 authors = ["Michael Bryan <michaelfbryan@gmail.com>"]
+build = "build.rs"
+link = "magic"

 [dependencies]
```

The `magic-sys` crate is all set up now, although lets compile and view the
generated docs for good measure.

```console
$ cargo build
$ cargo doc --open
```

## Creating a Rust Interface

Now we have bindings to the `libmagic` system library, lets create a more
idiomatic (and *safe*!) wrapper for it.

The first thing to do is get a feel for the library itself. By far the easiest
way to do this is by looking at an example (taken from 
[vivithemage/libmagic_example.c][example]).

```c
// libmagic_example.c

{{#include libmagic_example.c}}
```

The main things to take away from this are that:

1. All state is encapsulated in some `magic_cookie` object (created with 
   `magic_open()`)
2. You need to load a database with `magic_load()`, where passing in `NULL` will 
   use the default database
3. Querying a file's type is done using `magic_file()` which returns a pointer
   to a string (owned by the `magic_cookie`)
4. When you're done, clean up everything with `magic_close()`

> **Note:** More detailed information on each of these functions can be found by
> inspecting the *man* page (`man libmagic`). 

One really important piece of information to know is that internally `libmagic` 
uses a [global string array][strings] that keeps track of all allocated
strings so they can be deallocated in `magic_close()`. This means all strings
given to us by `libmagic` are borrowed and **you can only have one active
magic cookie at a time**.

Now we have a better understanding of things, lets get started with the code!

```console
$ cargo new --lib magic
     Created library `magic` project
$ cd magic
$ echo 'magic-sys = { path = "../magic-sys" }' >> Cargo.toml
```

First off, we're going to need a `struct` to represent a `libmagic` *cookie*
and take care of cleanup (by invoking `magic_close()`) in its destructor.

```rust
extern crate magic_sys;
use magic_sys::magic_t;

#[derive(Debug)]
pub struct Magic {
    cookie: magic_t,
}

impl Drop for Magic {
    fn drop(&mut self) {
        unsafe {
            magic_sys::magic_close(self.cookie);
        }
    }
}
```

We also need the ability to create a new `Magic` object. The constructor is
non-trivial in that we need to prevent having more than one `Magic` instance at
a time. This is done by using a global `AtomicBool` which is set to `true` when
our `Magic` is created and `false` when it gets destroyed (our `Drop` impl will
need updating).

```rust
use std::sync::atomic::{AtomicBool, Ordering, ATOMIC_BOOL_INIT};

/// A flag indicating that an instance of `Magic` is alive.
static MAGIC_CREATED: AtomicBool = ATOMIC_BOOL_INIT;

impl Magic {
    pub fn new() -> Result<Magic, CreationError> {
        // Try to set MAGIC_CREATED to true if it's currently false
        match MAGIC_CREATED.compare_exchange(false, true, Ordering::Relaxed, Ordering::Relaxed) {
            Ok(_) => {
                // We were successful. Now we can create a magic cookie
                let cookie = unsafe { magic_sys::magic_open(MAGIC_MIME) };
                let magic = Magic { cookie };

                if magic.cookie.is_null() {
                    // creation failed. Release MAGIC_CREATED (in drop) and
                    // return an error
                    drop(magic);
                    Err(CreationError::CreationFailed)
                } else {
                    Ok(magic)
                }
            }
            // MAGIC_CREATED was already true, return an error because a Magic
            // instance already exists 
            Err(_) => Err(Error::DuplicateInstances),
        }
    }
}

impl Drop for Magic {
    fn drop(&mut self) {
        unsafe {
            magic_sys::magic_close(self.inner);
        }

        // Unset the MAGIC_CREATED flag because we are no longer using the 
        // library
        MAGIC_CREATED.store(false, Ordering::Relaxed);
    }
}


#[derive(Debug, Copy, Clone, PartialEq)]
pub enum CreationError {
    CreationFailed,
    DuplicateInstances,
}
```

Now we can create a `Magic` object, we need to give it some methods so it can
actually do something. We'll also want a helper method for retrieving the most
recent error from `libmagic`, if there is one.

```rust
impl Magic {
    pub fn load_default_database(&mut self) -> Result<(), MagicError> {
        let outcome = unsafe { magic_sys::magic_load(self.cookie, ptr::null()) };

        if outcome == 0 {
            Ok(())
        } else {
            Err(self.last_error().unwrap_or_default())
        }
    }

    pub fn query_mimetype<P: AsRef<Path>>(&mut self, filename: P) -> Result<String, MagicError> {
        // Getting a CString from a Path is actually non-trivial due to
        // OS-specific details. Libmagic is only really available for *nix, so
        // we can get away with extracting the underlying bytes from an OS
        // string (a *nix path is just a bunch of non-null bytes) and passing
        // that in.
        let filename = CString::new(filename.as_ref().as_os_str().to_owned().into_vec())
            .expect("Paths never have null bytes");

        let mimetype = unsafe { magic_sys::magic_file(self.cookie, filename.as_ptr()) };

        if mimetype.is_null() {
            Err(self.last_error().unwrap_or_default())
        } else {
            let mimetype = unsafe { CStr::from_ptr(mimetype) };
            Ok(mimetype.to_string_lossy().to_string())
        }
    }

    /// Get the most recent `libmagic` error message, if there is one.
    fn last_error(&self) -> Option<MagicError> {
        // Get the last error message, if there was one
        let err = unsafe { magic_sys::magic_error(self.cookie) };

        if err.is_null() {
            None
        } else {
            let msg = unsafe { CStr::from_ptr(err) };
            Some(MagicError {
                msg: msg.to_string_lossy().to_string(),
            })
        }
    }
}
```

Now we've got a decent chunk of the crate's functionality established, it's a
good idea to give it some documentation. As a bonus, the examples can double
as sanity checks to ensure the bindings work correctly.

```rust
//! An interface to `libmagic`, a tool for file type recognition.
//!
//! # Examples
//!
//! You'll typically interact with `libmagic` by creating a `Magic` object and
//! using that to query a file's type.
//!
//! ```rust
//! # extern crate magic;
//! # use magic::Magic;
//! # use std::path::Path;
//!
//! let cargo_toml = Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml");
//!
//! let mut m = Magic::new().unwrap();
//! let mimetype = m.query_mimetype(&cargo_toml).unwrap();
//!
//! assert_eq!(mimetype, "text/plain; charset=us-ascii");
//! ```
```

## Full Source Code

For the sake of completeness, here's the full source code for our bindings:

```rust
// magic/src/lib.rs

{{#include magic/src/lib.rs}}
```

[bg-tut]: https://rust-lang-nursery.github.io/rust-bindgen/introduction.html
[build script]: https://doc.rust-lang.org/cargo/reference/build-scripts.html
[example]: https://gist.github.com/vivithemage/9489378
[strings]: https://github.com/file/file/blob/d14511987263ae3c6f5ad28ed7b81c26afdb422c/src/apprentice.c#L113-L116
[bindgen]: https://github.com/rust-lang-nursery/rust-bindgen