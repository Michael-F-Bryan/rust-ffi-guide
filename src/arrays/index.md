# Arrays

At some point you're almost certainly going to want to pass an array from Rust
to another language, or vice versa. The convention used when working with
arrays is to provide a pointer to the start of the array and the total number
of elements (so the receiver can do any necessary bounds checks). 

> **Note:** Perhaps unsurprisingly, under the hood a slice (`&[T]`) is 
> essentially a two-element struct containing a pointer and a length. 
>
> You can think of `&[T]` as being syntactic sugar for:
>
> ```rust
> pub struct Slice<T> {
>   first_element: *mut T,
>   length: usize,
> }
> ```

## Passing an Array to C

As an example, imagine we're writing a Rust application and want to leverage an
existing C library for some complicated operation (in this case, adding each of
the elements in a `&[i32]`).

```c
// array-utils.c

{{#include array_utils.c}}
```

The `array_utils.c` library can be compiled into a [shared object][so] (a
dynamically linked library which is resolved when the program gets loaded into
memory) with the following command:

```console
$ clang  -shared array_utils.c -o libarray_utils.so
$ ls
array_utils.c  index.md  libarray_utils.so  main.rs Makefile
$ file libarray_utils.so
libarray_utils.so: ELF 64-bit LSB pie executable x86-64, version 1 (SYSV), 
    dynamically linked, BuildID[sha1]=8893b08fed2e87e300e52a629e5d6fa1895ca4c2, 
    not stripped
```

Now we've got a library, lets have a look at the code which uses it.

```rust
// main.rs

{{#include main.rs}}
```

You can see that it's very similar to the last chapter. At the very start we
declare a `sum()` function inside an `extern "C"` block telling `rustc` it'll
need to link to one or more external libraries and to use the C calling
convention.

Within the `main()` function an array `[c_int: 8]` called `numbers` is allocated
on the stack. We then pass it to the `sum()` function and print out the result.

> **Note:** Notice that we need an `unsafe` block here. This is because even
> though we said `sum` accepts a `*const c_int`, there's absolutely nothing
> stopping `libarray_utils.so` from doing whatever it wants to the array 
> contents, making it almost trivial to undermine Rust's assumptions and 
> invariants.
>
> This is a very common theme you'll see when doing FFI. The compiler can't
> necessarily guarantee that a foreign library will uphold the assumptions and
> invariants expected by Rust in order to provide memory- and type-safety,
> therefore it becomes the developer's responsibility to maintain correctness
> (hence `unsafe`).

The Rust code can be compiled using:

```console
$ rustc main.rs -o arrays -L. -larray_utils
```

The `-L.` argument tells `rustc` to add `.` (the current directory) to the list
of paths used when searching for a library. `-larray_utils` instructs `rustc`
to link against the `array_utils` library (our `libarray_utils.so`).

The `arrays` program can then be executed using:

```console
$ ./arrays
The total is 36
```

## Exercises for the Reader

To get a better feel for arrays, see what happens when you:

- Pass in an incorrect array length
- "Accidentally" mix up the type of an array (e.g. use an array of `f64`s when
  an array of `i32` is expected)

[so]: https://en.wikipedia.org/wiki/Library_(computing)#Shared_libraries