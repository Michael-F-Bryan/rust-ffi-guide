# Arrays

Now that you've got the basics, lets try to do something a little more
interesting. Using arrays as function parameters.

For a change, Rust will be used as a library (i.e. the "guest" language) and
C as the "host" language. If you're trying to augment a legacy code base with
safer code which is less prone to memory issues and security vulnerabilities,
this is something you might end up doing often.


## The Rust Library

This program is going to be fairly stock standard. There will be a Rust function
which receives an array of integers and returns their average as a `f64`
([averages.rs](./arrays/averages.rs)).

```rust
use std::slice;

#[no_mangle]
pub extern "C" fn average(array: *const i64, length: i32) -> f64 {
    let numbers = unsafe { slice::from_raw_parts(array, length as usize) };

    let sum = numbers.iter()
        .fold(0.0, |acc, &elem| acc + elem as f64);

    sum / numbers.len() as f64
}
```

Most of the code will be quite familiar, except for the
[slice::from_raw_parts()][from-raw-parts]. This is a function which takes a
C-style array (pointer to the first element and a length) and turns it into a
Rust-style slice. Notably, the slice doesn't own the data it points to, so when
`numbers` goes out of scope and gets dropped, the original array won't be
affected.

The `extern "C"` bit in the function signature is important because it tells
`rustc` to generate a function which uses the `"C"` calling convention. `pub`
is necessary to make sure the function is exported `pub`, and the
`#[no_mangle]` attribute prevents the usual name mangling so the linker can
find the symbol `average` (check out [name mangling][mangling] for more
details).

There are several ways this innocent four-line `averages()` function could fail
or violate memory safety, here are just a few:

* The caller passes in a null pointer instead of a pointer to a valid array.
  this leads to a segfault the moment you try to iterate over it because you
  don't own the memory at address 0.
* The caller passes in a length which is longer than the actual array that was
  allocated. When you do your iteration you then start reading into memory you
  don't own and either the OS will make you segfault (if you're lucky), or you
  read bytes from the next thing in memory.
* The caller gives you a pointer to an array of floats (or bools, or structs,
  or some other type which **isn't** an `i64`). Because the linker's job is to
  hook up symbols with their call sites it doesn't (and can't) verify the
  function signatures are correct. If a user accidentally declares `averages`
  to take an array of `char`s, `slice::from_raw_parts()` will calculate
  incorrect offsets and memory bounds and you'll probably end up trying to read
  memory you don't own.

Issues like these are important to keep in mind when writing `unsafe` rust,
although they are probably quite familiar for people who've written C/C++ code
before.

Without further ado, lets compile the library.
<span id="rustc-static-notes"></span>
```bash
$ rustc --crate-type staticlib -o libaverages.a averages.rs
note: link against the following native artifacts when linking against this static library

note: the order and any duplication can be significant on some platforms, and so may need to be preserved

note: library: dl
note: library: rt
note: library: pthread
note: library: gcc_s
note: library: c
note: library: m
note: library: rt
note: library: util
```

As well as compiling `averages.rs` into a static library, `rustc` has also
emitted some helpful notes needed for linking the library into one executable.


## The C Program

To use the new Rust library in C, just declare and call it like any other
C library, then pass in the appropriate parameters.
Here are the contents of [main.c](./arrays/main.c):

```c
// main.c

#include <stdio.h>

double average(long *array, int length);

int main() {
    long arr[20];

    for (int i = 0; i < 20; i++) {
        arr[i] = i*i;
    }

    double avg = average(arr, 20);

    printf("The average is %f\n", avg);
}
```

And now let's compile everything.

```bash
$ gcc main.c libaverages.a
libaverages.a(std-9a66b6a343d52844.0.o): In function `std::sys::imp::mutex::{{impl}}::init':
/checkout/src/libstd/sys/unix/mutex.rs:56: undefined reference to `pthread_mutexattr_init'
/checkout/src/libstd/sys/unix/mutex.rs:58: undefined reference to `pthread_mutexattr_settype'
/checkout/src/libstd/sys/unix/mutex.rs:62: undefined reference to `pthread_mutexattr_destroy'
/checkout/src/libstd/sys/unix/mutex.rs:56: undefined reference to `pthread_mutexattr_init'
...
...
/checkout/src/libstd/sys/unix/thread.rs:240: undefined reference to `pthread_attr_getstack'
clang: error: linker command failed with exit code 1 (use -v to see invocation)
```

Oops! When clang tried to compile the `libaverages.a` library and `main.c` 
program into one executable it wasn't able to find a bunch of symbols.

Remember those [notes] from earlier? That's what `rustc` was trying to warn
about. When everything is statically compiled, **all** your dependencies must
be included. This wasn't an issue when dynamically linking because the loader
found everything for you.

Okay, lets try again...

```bash
$ clang -l dl \
    -l rt \
    -l pthread \
    -l gcc_s \
    -l c \
    -l m \
    -l util \
    -o main \
    main.c \
    libaverages.a
$ ls
main  averages.rs  libaverages.a  main.c
$ ./main
The average is 123.500000
```

Looks like it's finally working, but passing in all those `-l` arguments to
keep the linker happy was hard work! You can see why people came up with tools
like `make` to help them build everything.

Luckily in Rust, there's a better way...

[notes]: arrays/#rustc-static-notes
[from-raw-parts]: https://doc.rust-lang.org/nightly/std/slice/fn.from_raw_parts.html
[mangling]: https://en.wikipedia.org/wiki/Name_mangling
