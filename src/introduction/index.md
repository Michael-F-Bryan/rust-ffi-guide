# The Rust FFI Guide


Welcome to the Rust FFI Guide, i.e. **using unsafe for fun and profit**.

> **Note:** This guide assumes familiarity with the [Rust][rust] language and
> that a recent version of the compiler is installed. If you're a little rusty,
you might want to skim through [The Book][book] to refresh your memory.
>
> Most of the examples use either C/C++ or Python, so basic knowledge of either
> is recommended.


The main goal of this guide is to show how to interoperate between `Rust` and
other languages with as few segfaults and uses of undefined behaviour as
possible.

This guide will cover:

* [Compiling and linking from the command line](#Hello-World)
* [Using arrays](./arrays/)
* [Sharing basic structs between languages](./structs/)
* Proper error handling
* Calling Rust from other (i.e. not C) languages
* [How to use strings without leaking memory or segfaults](./strings/)
  (it's harder than you'd think)
* Asynchronous operations and threading
* [Bindgen](./bindgen/)
* [Dynamic Loading](./dynamic_loading/)
* [Callbacks and Function Pointers](./callbacks/)
* [Emulating methods and OO](./pythonic/), and
* [Other miscellaneous bits and pieces or best practices I've picked up along
  the way](./best_practices.html)


## Some Useful Links:

* [This Guide (rendered)](https://michael-f-bryan.github.io/rust-ffi-guide/)
* [The GitHub Repo](https://github.com/Michael-F-Bryan/rust-ffi-guide)
* Pages from *The Book*
    - [FFI Page](https://doc.rust-lang.org/book/ffi.html)
    - [Raw Pointers page](https://doc.rust-lang.org/book/raw-pointers.html)
* [The Rust FFI Omnibus](http://jakegoulding.com/rust-ffi-omnibus/)
* [Complex Types With Rust's FFI](https://medium.com/jim-fleming/complex-types-with-rust-s-ffi-315d14619479)
* [FFI: Foreign Function Interfaces for Fun & Industry](https://spin.atomicobject.com/2013/02/15/ffi-foreign-function-interfaces/)
* [Beginner-s Guide to Linkers](http://www.lurklurk.org/linkers/linkers.html)


## Hello World

What would any programming guide be without the obligatory hello world example?

> **Note:** For most of these examples I'll be using `C` to interoperate with
> my `Rust` code because it's often the lowest common denominator and *lingua
> franca* of the programming world.

To start off with, we'll try to call a `C` program from `Rust`. Here's the
contents of [hello.c](./introduction/hello.c):

```c
#include <stdio.h>

void say_hello(char *name) {
    printf("Hello %s!\n", name);
}
```

And some `Rust` code which calls it ([main.rs](./introduction/main.rs)):

```rust
use std::ffi::CString;
use std::os::raw::c_char;


extern "C" {
    fn say_hello(name: *const c_char);
}

fn main() {
    let me = CString::new("World").unwrap();

    unsafe {
        say_hello(me.as_ptr());
    }
}
```
The `say_hello()` function is expecting a pointer to a null-terminated string,
and the easiest way to create one of those is with a [CString][cstring]. Notice
that we told the compiler that we'll be using an external function called
`say_hello()`. The "C" bit indicates that it should use the "C" calling
convention,

> **Note:** a *calling convention* specifies low level details like how
> arguments are passed around, which registers the callee must preserve
> for the caller, and the other nitty gritty details needed to call the
> function.

Calling foreign functions sidesteps all Rust's memory guarantees, so expect to
see a lot more `unsafe` blocks. This isn't necessarily a bad thing in itself,
it just means you need to pay a little extra attention to how memory is
handled.

Next you'll need to compile the C code into a library which can be called by
Rust. In this example I'm going to compile it into a `shared library` where the
external `say_hello` symbol will be resolved at *load time*.

> **Note:** If you aren't familiar with the difference between a `static` and
> `dynamic` library, or how you use them you might want to read
> [this Stack Overflow question][static-vs-dynamic].

```bash
$ gcc -shared -fPIC -o libhello.so hello.c
```

The `-shared` flag tells clang to compile as a dynamically linked library
(typically "\*.so" on Linux or "\*.dll" on Windows). You'll also need the
`-fPIC` flag to tell the compiler to generate [*Position Independent
Code*][pic]. This is fairly common when creating shared libraries because it
means the same library code can be loaded in a location in each program
address space where it will not overlap any other uses of memory.

Next up is compiling `main.rs`:

```bash
$ rustc -l hello -L . main.rs
```

The `-l` flag tells `rustc` which library it'll need to link against so it can
resolve symbols, and the `-L` flag adds the current directory to the list of
places to search when finding the `hello` library.

Finally we can actually run the program:

```bash
$ LD_LIBRARY_PATH=. ./main
```

> **Note:** When you try to run a program, what actually happens is the
> [loader][loader] loads the binary into memory, then tries to find any symbols
> belonging to shared libraries. Because `libhello.so` isn't in any of the
> standard directories the loader usually searches, we have to explicitly tell
> the loader where it is by overriding `LD_LIBRARY_PATH`.

If we didn't override the `LD_LIBRARY_PATH` then you'd see an error something
like this:

```bash
$ ./main
./main: error while loading shared libraries: libhello.so: cannot open shared object file: No such file or directory
```

There are much more elegant solutions than this, but it'll suffice for now.


[rust]:  https://www.rust-lang.org/
[book]: https://doc.rust-lang.org/stable/book/
[loader]: https://en.wikipedia.org/wiki/Loader_(computing)
[static-vs-dynamic]: http://stackoverflow.com/questions/2649334/difference-between-static-and-shared-libraries
[cstring]: https://doc.rust-lang.org/nightly/std/ffi/struct.CString.html
[pic]: https://en.wikipedia.org/wiki/Position-independent_code
