# Dynamic Loading & Plugins

What application wouldn't be complete without the ability to add user-defined
plugins? In this chapter we take a small detour to visit the concept of 
dynamically loading a library at runtime and registering it with our parent
application.

The end goal is to allow users to provide a shared library (DLL, `*.so`, etc) 
which contains a set of pre-defined functions. These functions will then allow
us to manipulate a request before it is sent and then manipulate/inspect the
response before displaying it to the user.

From the Rust side of things, by far the easiest way to establish this is to 
define a `Plugin` trait which does the various manipulations, then add in a 
macro users can run which will define all the `unsafe` function declarations.

Our `Plugin` trait may look something like this:

```rust
pub trait Plugin {
    fn name(&self) -> &'static str;
    fn pre_send(&mut self, _request: &mut Request) {}
    fn post_receive(&mut self, _response: &mut Response) {}
}
```

The macro would then declare an `extern "C"` constructor which exports a trait 
object (`Box<Plugin>`) with some pre-defined symbol (e.g. `__plugin_create()`). 

Before diving into the complexity of real code, it's probably going to be easier
if we figure out how dynamic loading works in a contrived example.


## Contrived Example

For this the function being exported doesn't need to be very complex, seeing as
we're not actually testing it.

```rust
#[no_mangle]
pub extern "C" fn add(a: isize, b: isize) -> isize {
    a + b
}
```

This then can then be compiled into a `cdylib`. 

> **Note:** Although up uptil now it hasn't made a difference whether you
> compile as a dynamic library or a static one. However for dynamically loading 
> a library on the fly you **must** compile as a `cdylib`.

```bash
$ rustc --crate-type cdylib adder.rs
```

The symbols exported by this dynamic library can now be inspected using the
`nm` tool from GNU `binutils`. 

```bash
$ nm libadder.so | grep 'add'
00000000000005f0 T add
```

As you can see, the `add` function is exposed and fully accessible to other 
programs.


## Loading the Library

Loading a function from this library and then calling it is then surprisingly
easy. The key is to use something like the [libloading] crate. This abstracts
over the various mechanisms provided by the operating system for dynamically 
loading a library.

```rust
extern crate libloading;

use std::env;
use libloading::{Library, Symbol};
```

It's also a good idea to add a type alias for the `add()` function to make it
more readable.

```rust
type AddFunc = unsafe fn(isize, isize) -> isize;
```

The `main()` function takes the DLL as its first command line argument:

```rust
fn main() {
    let library_path = env::args().nth(1).expect("USAGE: loading <LIB>");
    println!("Loading add() from {}", library_path);
```

Loads the library 

``` rust
    let lib = Library::new(library_path).unwrap();

    unsafe {
        let func: Symbol<AddFunc> = lib.get(b"add").unwrap();
```

Then you can finally call the imported function.

``` rust
        let answer = func(1, 2);
        println!("1 + 2 = {}", answer);
    }
}
```

Now compiling and running with cargo gives exactly what we'd expect:

```bash
$ cargo run -- ../libadder.so
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/loading ../libadder.so`
Loading add() from ../libadder.so
1 + 2 = 3
```

The entire `main.rs` looks like this:

```rust
extern crate libloading;

use std::env;
use libloading::{Library, Symbol};

type AddFunc = fn(isize, isize) -> isize;

fn main() {
    let library_path = env::args().nth(1).expect("USAGE: loading <LIB>");
    println!("Loading add() from {}", library_path);

    let lib = Library::new(library_path).unwrap();

    unsafe {
        let func: Symbol<AddFunc> = lib.get(b"add").unwrap();

        let answer = func(1, 2);
        println!("1 + 2 = {}", answer);
    }
}
```


[dl]: https://michael-f-bryan.github.io/rust-ffi-guide/dynamic_loading/index.html
[libloading]: https://crates.io/crates/libloading