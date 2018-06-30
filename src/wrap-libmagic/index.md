# Create Bindings for a C Library

Our first example will be creating a Rust interface to a common C library,
`libmagic`. This is the library which is used by the `file` command for
identifying a file's type by inspecting its contents (e.g. looking for *magic*
numbers).

The process of creating a Rust interface to a foreign library is actually quite
common. If you've done much programming with Rust you will have probably seen
several `*-sys` crates scroll past as `cargo` is compiling your project. When
creating bindings for a foreign library, you typically break it up into two
crates:

- A `*-sys` (in this case `magic-sys`) contains Rust declarations for the raw C
  functions and types from `magic.h`
- A more Rust-ic crate (`magic`) which exports the functionality from
  `magic-sys` with a safer and more idiomatic interface

## Creating The *-sys Crate

The `*-sys` crate has two main purposes. It contains the various `extern` 
declarations for `libmagic`, and it needs to tell `rustc` how to link to the
library itself. Often a `*-sys` crate will try to compile the library from
source if it isn't detected on the system, but we'll skip that step for 
simpilicity.

Luckily we can leverage [bindgen] for a lot of this section.

> **Note:** If you aren't already familiar with `bindgen`, you may want to check
> out [their tutorial][bg-tut].

First we'll need a new `crate`

```console
$ cargo new --lib magic-sys
     Created library `magic-sys` project
```

Next we need to create our `wrapper.h` for bindgen to read.

```console
$ cd magic-sys
$ cat wrapper.h 
#include <magic.h>
```

And finally we can tell `bindgen` to create bindings for all functions in
`libmagic` (matching the regex, `magic_.*`), as well as any types they depend
on.

```console
$ bindgen wrapper.h --whitelist-function 'magic_.*' > src/lib.rs
```

To make sure `rustc` links in `libmagic` we'll need to create a [build script].

```console
$ echo 'fn main() { println!("rustc-link-lib=magic"); }' > build.rs
```

From the documentation on build scripts:

> `rustc-link-lib=[KIND=]NAME` indicates that the specified value is a library
> name and should be passed to the compiler as a `-l` flag. The optional `KIND` 
> can be one of `static`, `dylib` (the default), or `framework`, see 
> `rustc --help` for more details.

We also need to update `magic-sys`'s `Cargo.toml` file to tell `cargo` about our
build script and let it know we link to `libmagic` (this prevents accidentally
linking to two versions of `magic-sys` because then bad things can happen).

```console
$ git diff Cargo.toml
 [package]
 name = "magic-sys"
 version = "0.1.0"
 authors = ["Michael Bryan <michaelfbryan@gmail.com>"]
+build = "build.rs"
+link = "magic"

 [dependencies]
```

The `magic-sys` crate is all set up now, although lets compile and view the
generated docs for good measure.

```console
$ cargo build
$ cargo doc --open
```

[bg-tut]: https://rust-lang-nursery.github.io/rust-bindgen/introduction.html
[build script]: https://doc.rust-lang.org/cargo/reference/build-scripts.html