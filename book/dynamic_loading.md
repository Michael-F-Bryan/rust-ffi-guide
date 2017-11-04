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
if we figure out how dynamic loading works using a contrived example.


## Contrived Example

For this the function being exported doesn't need to be very interesting, seeing 
it's just an example.

```rust
#[no_mangle]
pub extern "C" fn add(a: isize, b: isize) -> isize {
    a + b
}
```

This then can then be compiled into a `cdylib`. 

> **Note:** Up uptil now it hasn't mattered whether you compile as a dynamic 
> library or a static one. However for dynamically loading a library on the fly 
> you **must** compile as a `cdylib`.

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


## Loading the Contrived Example

Loading a function from this library and then calling it is then surprisingly
easy. The key is to use something like the [libloading] crate. This abstracts
over the various mechanisms provided by the operating system for dynamically 
loading a library.

```rust
extern crate libloading;

use std::env;
use libloading::{Library, Symbol};
```

It's also a good idea to add a type alias for the `add()` function's signature.
This isn't required, but when things start getting more complex and having more
interesting arguments the extra readability really pays off.

```rust
type AddFunc = unsafe fn(isize, isize) -> isize;
```

The `main()` function takes the DLL as its first command line argument:

```rust
fn main() {
    let library_path = env::args().nth(1).expect("USAGE: loading <LIB>");
    println!("Loading add() from {}", library_path);
```

Loads the library and gets a symbol (casting the function pointer so it has the
desired signature)

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


## What Just Happened?

Although the example `main.rs` looks quite simple, there's a surprising amount 
of complexity going on behind the scenes!

First we're creating a shared object (or DLL, depending on your platform) which
exposes an `add` function, like we've been doing for the Rust-C++ interop in the
previous chapters.

Next, at runtime we use the `libloading` crate to load the library. This uses
whatever mechanism (usually called the `loader`) is exposed by the Operating 
System to load a library into the address space of the currently running 
process. It will also make sure to load any dependencies our library may have.

Now that the library is in memory we can call into the various functions it may
contain, however before you can call a function from the library you need to 
find its address (otherwise how does the computer know where to jump to?). This
is where the `Library`'s `get()` method comes in. It takes in byte string and
will try to find the symbol with that name (typically by calling 
[GetProcAddress] on Windows or [dlsym] on Linux).

Now that we have an address we can cast it to whatever we want, in this case an
`unsafe fn(isize, isize) -> isize`. This is quite obviously going to be an 
extremely `unsafe` operation, because there's nothing stopping us from using the
function with the wrong signature and invoking UB. 

There are also no guarantees that the address we are given may be valid. If
the library is later unloaded from memory the address will now be pointing
into memory we don't own. This means calling a library's function after it is
unloaded will be the equivalent of a use-after-free. Fortunately,
`libloading` uses Rust's concept of lifetimes to make sure it's impossible
for something like that to happen.

For more details, Wikipedia has a very informative [article] on dynamic loading. 


## Setting Up Plugins

Now that we have a better understanding of how dynamically loading a library on
the fly works, we can start adding plugins to our application.


[dl]: https://michael-f-bryan.github.io/rust-ffi-guide/dynamic_loading/index.html
[libloading]: https://crates.io/crates/libloading
[article]: https://en.wikipedia.org/wiki/Dynamic_loading
[GetProcAddress]: https://msdn.microsoft.com/en-us/library/windows/desktop/ms683212(v=vs.85).aspx
[dlsym]: https://linux.die.net/man/3/dlsym