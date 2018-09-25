# Getting Started

About the simplest thing you can do when working with FFI code is to call a
function. To start off, we'll write a simple Rust program that calls the
`puts()` function from `libc` to print a message to the console.

This is what the entire program looks like:

```rust
// hello_world.rs

{{#include hello_world.rs}}
```

While it's not a lot of code there are a couple tricky concepts in that program,
so lets step through it section by section.

First we write an `extern` block to declare that there is some function called
`puts` that takes a pointer to a string (`*const c_char`, the equivalent of
`char*` in C), and will return the number of characters printed as a `c_int`.

> For convenience, the Rust standard library declares `c_char` and `c_int`  as
> type aliases for the corresponding C types, `char` and `int` respectively.
> This is especially useful seeing as C's integer types can have different
> binary representations depending on the platform.
>
> We're also taking advantage of the fact that the system `libc` will be linked
> into every binary, meaning we don't need to provide `rustc` with linker
> arguments (see [Linking and Building](../linking/index.md) for more on linking).

Next we have the `main()` function. This creates a `CString` (essentially just
a boxed `char*` C-style string with trailing null character) from the string,
`Hello World!`. The `CString::new()` function makes sure our string doesn't have
any internal null characters, but because we don't really care about error
handling just yet (and we also know `"Hello World!"` doesn't contain null) we
can just use `unwrap()`.

Finally we have our call to `puts()`, wrapped in an `unsafe` block. As a general
rule, all calls to foreign code are `unsafe`. This is because `rustc` has no way
to guarantee the C code won't do something to break Rust's safety constraints.

You can compile and run our executable like so:

```console
$ rustc hello_world.rs -o hello_world
$ ./hello_world
Hello World!
```

## Calling Rust From C

Now we've called a C function from Rust it's only natural to try going in the
opposite direction.

To do this we'll need two pieces. A Rust library and a C program that calls into
it.

Lets start off with the C code, seeing as that's simplest.

```c
// main.c

{{#include main.c}}
```

This is almost line-for-line identical to the Rust code from the previous
example. First we declare the external `print()` function, then we create our
string, and finally we pass it to `print()` to print it to the screen.

Unfortunately the Rust code exporting our `print()` function is more
complicated and may contain some unfamiliar code.

```rust
// print.rs

{{#include print.rs}}
```

Lets break that function signature down a little:

- `#[no_mangle]` tells `rustc` not to do [name mangling] for the `print` symbol.
  This lets the linker figure out which symbol to use for `print` when
  generating the final executable.
- `pub` is used to declare `print()` as part of the crate's public API, making
  sure LLVM doesn't accidentally optimise it away
- `unsafe` means this function is `unsafe` to call because we're dereferencing
  a pointer which might point to *anything* (e.g. garbage, something that isn't
  a string, or even memory that doesn't exist on the machine). This isn't
  strictly necessary because C doesn't know (or care) that our function is
  `unsafe`, however it helps to document that the entire function may need a
  little extra attention when auditing or debugging.
- `extern "C"` tells the compiler to use the C [calling convention] when
  generating code for this function
- `msg: *const c_char` means we're expecting to receive a constant raw pointer
  to a `c_char` (C's way of representing a string)

The first two lines of our `print()` function are dedicated to converting the
message from a `*const c_char` into the more familiar `&str`. The `CStr` type is
a useful helper from the standard library for working with borrowed C-style
strings.

Finally, we print out the message using the familiar `println!()` macro.

Compiling this example is also more complicated than before.

First we need to compile `print.rs` as a [static library].

```console
$ rustc print.rs --crate-type staticlib
$ ls
libprint.a  main.c  print.rs
```

Next we need to compile `main.c`, making sure to include our new `libprint.a`.
We'll also link in a couple system libraries which are needed by `libprint.a`.

```console
$ clang main.c libprint.a -lpthread -ldl
```

> **Note:** Compiling `print.rs` with the `--print native-static-libs` flag will
> tell `rustc` to also print out the system libraries we need to link to when
> linking the final executable.
>
> ```console
> $ rustc --print native-static-libs print.rs --crate-type staticlib
> note: Link against the following native artifacts when linking against this
>   static library. The order and any duplication can be significant on some
>   platforms.
>
> note: native-static-libs: -ldl -lrt -lpthread -lgcc_s -lc -lm -lrt -lpthread
>   -lutil -lutil
> ```
>
> We could get away with only specifying `-lpthread` and `-ldl` earlier because
> no other calls to system libraries were used, and linkers are nice in that
> they'll do the equivalent of [dead code elimination][dce] and you won't need
> to link to that library. If you encounter any linker errors, you may want to
> double check that all necessary libraries are specified with the `-l` flag.

And finally we can run our program!

```console
$ ./print
Hello World!
```

## Exercises for the Reader

To get a better understanding of what's going on you may want to try one or more
of the following:

- Remove the `#[no_mangle]` line from `print.rs`
- Run the `nm` tool on `libprint.a` and look for the `print` symbol
- Compile `main.c` without including the `-l` arguments
- Google "*static vs dynamic linking*"

[name mangling]: https://en.wikipedia.org/wiki/Name_mangling
[calling convention]: https://en.wikipedia.org/wiki/Calling_convention
[static library]: https://en.wikipedia.org/wiki/Static_library
[dce]: https://en.wikipedia.org/wiki/Dead_code_elimination
