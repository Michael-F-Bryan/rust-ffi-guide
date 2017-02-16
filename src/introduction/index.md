# The Rust FFI Guide


Welcome to the Rust FFI Guide, aka **Abusing unsafe for fun and profit**.

> **Note:** I'm going to assume you're already familiar with the `Rust` language
> and have a relatively recent version of the compiler installed. If you're a 
> little rusty, you might want to skim through [the book][book] to refresh your 
> memory.
>
> You'll also need to know some basic C/C++, as I'll be largely using that in 
> my examples.


The main goal of this guide is to show how to interoperate between `Rust` and
other languages with as few segfaults and uses of undefined behaviour as 
possible.

Some things I'm hoping to cover:

* Compiling and linking from the command line
* Calling Rust from various languages, and vice versa
* Packaging your crate as a shared library or DLL so it can be used by other 
  programs
* Passing around structs using opaque pointers
* How to use strings without leaking memory or segfaults (it's harder than 
  you'd think)
* Emulating methods and OO
* Asynchronous operations and threading, and
* Other miscellaneous bits and pieces and best practices I've picked up along
  the way


## Hello World

What would any programming guide be without the obligatory hello world example?

> **Note:** For most of these examples I'll be using `C` to interoperate with 
> my `Rust` code. It's pretty much the lingua franca of the programming world, 
> so most people should be able to understand it what's happening and follow 
> along. 
 
To start off with, we'll try to call a `C` program from `Rust`. Here's the 
contents of my `hello.c`:

```c
#include <stdio.h>

void say_hello(char *name) {
    printf("Hello %s!\n", name);
}
```

And here's the `Rust` code which will be using it (`main.rs`):

```rust
use std::ffi::CString;
use std::os::raw::c_char;


extern "C" {
    fn say_hello(name: *const c_char);
}

fn main() {
    let me = CString::new("Michael").unwrap();

    unsafe {
        say_hello(me.as_ptr());
    }
}
```
Our `say_hello()` function is expecting a pointer to a null-terminated string,
and the easiest way to create one of those is with a [CString][cstring].

Almost all of what we're doing here sidesteps Rust's memory guarantees, so 
expect to see a lot more `unsafe` blocks. In this case, the C function could do
whatever it wants with our string, so you need to wrap the function call in 
`unsafe`.

Next you'll need to compile the C code into a library which can be called by
Rust. In this example I'm going to compile it into a `shared library`.

> **Note:** If you aren't familiar with the difference between a `static` and 
> `dynamic` library, or how you use them you might want to read 
> [this Stack Overflow question][static-vs-dynamic].

```bash
$ clang -shared -fPIC -o libhello.so hello.c
```

The `-shared` flag tells clang to compile as a dynamically linked library 
(typically "\*.so" on Linux or "\*.dll" on Windows). You'll also need the `-fPIC`
flag to tell the compiler to generate Position Independent Code so that the 
generated machine code is not dependent on being located at a specific address 
in order to work. This basically means when invoking a function it'll use 
relative jumps rather than absolute.

Next up is compiling `main.rs`:

```bash
$ rustc -l hello -L . main.rs
```

The `-l` flag tells `rustc` which library it'll need to link against so it can
resolve symbols, and the `-L` flag adds the current directory to the list of 
places to search when finding the `hello` library.

Finally we can actually run the program

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


[book]: https://doc.rust-lang.org/stable/book/
[loader]: https://en.wikipedia.org/wiki/Loader_(computing)
[static-vs-dynamic]: http://stackoverflow.com/questions/2649334/difference-between-static-and-shared-libraries
[cstring]: https://doc.rust-lang.org/nightly/std/ffi/struct.CString.html
