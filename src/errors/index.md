# Error Handling

When designing error handling in FFI code it's a good idea to follow the ancient
wisdom, "*when in Rome, do as the Romans do*".

For decades, C programmers have been using *Posix-style* error handling with
great success. Similar to error handling in Rust, this is a convention based
around error code rather than exceptions.

A rough summary of Posix-style error handling can be found towards the top of
[Error reporting in libgit2]:

> Functions return an `int` value with `0` (zero) indicating success and
> negative values indicating an error. There are specific negative error codes
> for each "expected failure" (e.g. `GIT_ENOTFOUND` for files that take a path
> which might be missing) and a generic error code (`-1`) for all critical or
> non-specific failures (e.g. running out of memory or system corruption).

Error handling in the Windows API is done in [a very similar way][winapi] to
`libgit2`. If something fails it returns an "*obviously invalid*" value (e.g.
`0`, `-1`, or `NULL`) and sets a thread-local *last-error code*. It is then the
caller's duty to check the return code of each function and handle the error
appropriately.

When planning error handling for a library or application which needs to
interoperate with other languages, there are a couple rules of thumb you should
follow:

1. Keep it consistent
2. Keep it simple
3. Try to follow the same conventions as other code around you

Rust has an advantage over many other languages in that it's error handling is
already based around some form of return value. This makes writing FFI code
considerably easier than a language based around exceptions, because it's
obvious which parts may fail. Keeping FFI bindings as small and boring as
possible helps out with this too.

## Worked Example

To practice our error handling, lets create a simple Rust library for parsing a
TOML file and inspecting it.

First we'll create a new crate and update `Cargo.toml` to pull in the `toml`
library.

```console
$ cargo new --lib tomlreader
$ cd tomlreader && cat Cargo.toml

{{#include tomlreader/Cargo.toml}}
```

Because most of the code for parsing TOML is already done, we just need to focus
on writing FFI bindings to expose the `toml` crate to C. To make things easier
we'll use `cbindgen` to generate a `tomlreader.h` header file. The instructions
for setting up `cbindgen` are identical to the [Binding Generators] chapter and
can be copied straight from their README.

The header file we'll be generating should look something like this:

```c
// tomlreader.h

{{#include tomlreader/tomlreader.h}}
```

We'll also need a C program to call our TOML reading library.

```c
// main.c

{{#include main.c}}
```


[ffi_helpers]: https://crates.io/crates/ffi_helpers
[libgit2]: https://github.com/libgit2/libgit2/blob/master/docs/error-handling.md
[Error reporting in libgit2]: https://github.com/libgit2/libgit2/blob/master/docs/error-handling.md
[winapi]: https://docs.microsoft.com/en-au/windows/desktop/Debug/last-error-code
[Binding Generators]: ../binding-generators/index.md
