# Working With POD Types

Seeing as a `struct` is just a way to lay data heterogeneous data out in
consecutive bytes of memory, it shouldn't come as a surprise that simple data
types can be directly transferred between languages. 

If you've ever looked at compiled assembly you'll know that complex constructs
like types and inheritance are completely erased during compilation. Therefore,
as long as both parties agree to use the same way of laying out structs,
passing *simple* data types between languages should *Just Work*.

> **Note:** The term *POD type* is often used in C++ to refer to a type which
> is trivially copyable (`Copy` in Rust lingo), short for "*Plain Old Data*".
> This is something which has no constructors, no destructors and is composed
> completely of other POD types.

This is probably easiest explained with an example:

```c
// main.c

{{#include main.c}}
```

Our `main.c` isn't overly special. It simply declares a `Point` type and then
passes it to an `add_points()` function which will be exported by some Rust 
code.

```rust
// add_points.rs

{{#include add_points.rs}}
```

We can then compile and run this to get the expected output:

```console
$ rustc add_points.rs --crate-type cdylib
$ clang  main.c -L. -ladd_points -o main
$ ./main
Sum is Point {x: -1.940000, y: 45.400000}
```

Something important to notice is the `#[repr(C)]` attribute added to the Rust
struct. This tells the compiler "*lay this out just like C would*", and is
essential for making Rust and C (or any other language) represent things
identically in memory. 

Without the attribute specifying how things are laid out, `rustc` is free to
rearrange or adjust things as it sees fit. This is important because by default
the layout is unspecified and the compiler is free to do whatever it wants.
For example, it could insert space between items (often called *padding*)
to provide the correct alignment or it may rearrange items so they pack
together more efficiently.

[pod]: https://stackoverflow.com/a/146454
