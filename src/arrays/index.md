# Arrays 

Now that you've got the basics, lets try to do something a little more 
interesting. We'll pass around some arrays.

For a change, we'll be using Rust as a library (i.e. the "guest" language) and 
C as the "host" language. If you're trying to augment a legacy code base with 
safer code which is less prone to memory issues and security vulnerabilities, 
this is something you might end up doing often.


## Our Rust Library

This program is going to be fairly stock standard. We'll write a Rust function 
which receives an array of integers and gives you back their average as a `f64`.

```rust
// averages.rs

use std::slice;

#[no_mangle]
pub extern "C" fn average(array: *const i64, length: i32) -> f64 {
    let numbers = unsafe { slice::from_raw_parts(array, length as usize) };

    let sum = numbers.iter()
        .fold(0.0, |acc, &elem| acc + elem as f64);

    sum / numbers.len() as f64
}
```

The code itself is quite tame, and should be familiar to most rustaceans, but
that `unsafe` line should have caught your attention. We're using 
[slice::from_raw_parts()][from-raw-parts] to tell the compiler "here's a pointer
to some an `i64` and a number of elements, can you just pretend it's an array 
for me?". 

> **Note:** Notice that I used `slice::from_raw_parts()` here to get a slice 
> instead of getting a `Vec` with `Vec::from_raw_parts()`. I'll leave it as an
> exercise for the reader to figure out why (hint: who owns that chunk of 
> memory?)

Obviously must be unsafe (hence the `unsafe` block), but what exactly could go
wrong here? I'll try to list just a few ways you could end up having a bad 
time.

* The caller passes in a null pointer instead of a pointer to some valid array.  
  this leads to a segfault the moment you try to iterate over it because you
  don't own the memory at address 0.
* The caller passes in a length which is longer than the actual array that was 
  allocated. When you do your iteration you then start reading into array you
  don't own and either the OS will make you segfault (if you're lucky), or you
  read bytes from the next thing in memory.
* The caller gives you a pointer to an array of floats (or bools, or structs, 
  or whatever). `slice::from_raw_parts()` doesn't do any type checking, so it'll
  happily let you read in garbage (assuming you don't segfault).


Now that we have a better idea of what *could* go wrong, lets compile this baby.
Just for fun, lets make this example statically compiled.

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

Woah, what happened there?! What are all these extra notes about?

It turns out that because we're wanting to compile everything into one 
executable, dependencies and all, we'll also need to link in a bunch of other 
stuff. If you don't really understand what I'm talking about, its okay, you'll
see what I mean a bit later.


## The C Program

Using our Rust library is actually fairly easy to do in C. You just declare it
like you would when calling any other C library, then pass in the appropriate
parameters.

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
$ clang main.c libaverages.a
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

Oops! When clang tried to compile our `libaverages.a` library and `main.c` into 
one executable it wasn't able to find a bunch of symbols. 

Remember those notes from earlier? That's what `rustc` was trying to warn us 
about. When you compile everything statically you need to include **all** your 
dependencies. You didn't have this issue when dynamically linking because the 
loader finds everything for you.

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

Luckily in Rust, we can do one better...



[from-raw-parts]: https://doc.rust-lang.org/nightly/std/slice/fn.from_raw_parts.html
