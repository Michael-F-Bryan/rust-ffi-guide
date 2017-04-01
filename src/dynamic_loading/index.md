# Dynamic Loading

Dynamic loading enables you to load a library (often called Dynamic-Link
Library, aka DLL) at run time instead of needing all your dependencies up front 
when compiling, it's a very common practice in Windows and can allow you to 
upgrade a library once and let any program which uses it benefit from new bug 
fixes.

The way dynamic loading is done is by searching for a particular symbol in your 
DLL (a symbol is just some string corresponding to the address for a particular
function), assuming it has a function signature which is known ahead of time,
then it gets called just like any other function. Obviously this could all go 
hideously wrong if you try to use the function with one signature and it was 
compiled with another, but generally it is assumed that the user knows what
they're doing.

Windows and Unix use two completely different ways of dynamically loading a
library, so often a good idea to use an established crate to abstract over
these implementation details. One particularly popular crate for this is
[libloading][libloading].


## Setting Up

Instead of diving in and loading a proper library installed on your system it's
a good idea to familiarize yourself with the concept by trying it out on a toy
example. For example, [here][adder] is a really simple rust library which
exports a function for adding two integers.

```rust
#[no_mangle]
pub extern "C" fn add(a: isize, b: isize) -> isize {
    a + b
}
```

This then gets compiled into a `cdylib`:

```bash
$ rustc --crate-type cdylib adder.rs
```

The symbols exported by this dynamic library can now be inspected using the
`nm` tool from GNU `binutils`.

```bash
$ nm libadder.so | grep 'add'
00000000000005f0 T add
```


## Loading the Library

Loading a function from this library and then calling it is then surprisingly
easy. 

First you need the usual imports:

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

The entire [main.rs] looks like this:

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


## Useful Applications

Linking to a library dynamically at runtime (compared to compiling it in as a
static lib or linking when an executable gets loaded into memory) gives the
programmer a lot of flexibility because now users can provide their own library
and as long as all the right symbols are present everything *just works*. This
is great for something like a plugin system where users can compile their own
plugins in whatever language they want, then have their plugins run alongside
the rest of an application.

If that doesn't really make much sense think of it like this; pretend you are
making your own text editor and instead of inventing your own scripting
language you want to allow people to write plugins in Rust. The plugin code may
look something like this:

```rust
pub struct PluginRegistry {
    ...
}

impl PluginRegistry {
    pub fn load_plugin(&mut self, plugin_path: &Path) -> Result<(), Error> {
        // load the library dynamically
        // Find the `get_plugin()` function
        // Call it
        // Then save the plugin 
    }
}

pub trait Plugin {
    fn on_save(...);
    fn on_select(...);
    fn on_keypress(...);
    ...
}
```

Then you can say that any plugin library must export a function which will
return a boxed object which implements the `Plugin` trait. For example:

```rust
#[no_mangle]
pub extern fn get_plugin() -> Box<Plugin> {
    ...
}
```

> **Interesting Fact:** This is actually the exact same mechanism the Linux 
> kernel uses so it can load and unload kernel modules without needing to
> reboot.
>
> According to [The Linux Documentation Project][tldp]:
>
> > Kernel modules must have at least two functions: a "start" (initialization)
> > function called `init_module()` which is called when the module is 
> > insmoded into the kernel, and an "end" (cleanup) function called 
> > `cleanup_module()` which is called just before it is rmmoded.


[libloading]: https://crates.io/crates/libloading
[adder]: https://crates.io/crates/libloading
[main.rs]: ./dynamic_loading/loading/src/main.rs
[tldp]: http://www.tldp.org/LDP/lkmpg/2.6/html/x121.html
