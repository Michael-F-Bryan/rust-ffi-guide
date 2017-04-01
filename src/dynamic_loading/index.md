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
cargo run -- ../libadder.so
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/loading ../libadder.so`
Loading add() from ../libadder.so
1 + 2 = 3
```

I've also uploaded the [main.rs] for those who want to see the full example.


[libloading]: https://crates.io/crates/libloading
[adder]: https://crates.io/crates/libloading
[main.rs]: ./dynamic_loading/loading/src/main.rs
