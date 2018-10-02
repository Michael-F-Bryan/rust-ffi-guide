# Strings

In theory working with strings should be almost identical to working with
arrays, but there are a couple small differences which can make life miserable
if not taken into account. These are:

- Encoding, Rust uses UTF-8 strings (stored as `[u8]`) which are completely
  incompatible with UTF-16 (stored as `[u16]`). UTF-16 is ubiquitous in Windows
  environments, where it is often just referred to as *Unicode*
- Strings are almost always heap-allocated and any string created by Rust must
  be passed back to Rust code so it can be deallocated
- People like to copy strings around willy-nilly, which can sometimes make the
  previous point about allocations feel like an unnecessary papercut

## The Basics

Fortunately, for the vast majority of use cases the standard library contains
everything you need for passing strings back and forth between Rust and other
languages.

In C, the *Lingua Franca* which all languages must work with in order to
interoperate, strings are represented as a pointer to one or more characters,
terminated with a null byte. Because Rust strings are represented using a
pointer and the number of characters, we need a set of shims to bridge the gap
between the two representations. These are [`std::ffi::CStr`] and
[`std::ffi::CString`], the FFI-friendly analogue of `&str` and `String`.

To show these two in action, let's create a dummy application which just passes
strings between Rust and C so they can be printed.

Our C "library" for printing strings uses `printf()` for all the heavy lifting.

```c
// dummy.c

{{#include dummy.c}}
```

The vast majority of `main.rs` is taken up by boilerplate which imports the
relevant types and forward-declares the `print()` function from our C library.

```rust
// main.rs

{{#include main.rs}}
```

Compiling and linking is also rather straightforward:

```console
$ clang -shared dummy.c -o libdummy_c.so
$ rustc main.rs -L. -ldummy_c -o dummy_rs
$ ./dummy_rs
Printing "Hello, World!" from C
```

Going in the opposite direction is equally as simple.

```rust
// dummy.rs

{{#include dummy.rs}}
```

```c
// main.c

{{#include main.c}}
```

Due to using similar conventions for compiler flags, compiling and linking looks
almost identical to before.

```console
$ rustc dummy.rs --crate-type cdylib -o libdummy_rs.so
$ clang main.c -L. -ldummy_rs -o dummy_c
$ ./dummy_c
Printing "Hello, World!" from Rust
```

> **Note:** In the `main.rs` application you might be tempted to condense things
> down by skipping the intermediate `c_style_msg` variable, replacing it with
> the more direct `print(CString::new(msg).unwrap().as_ptr())`.
>
> This will probably lead to a segfault (if you're lucky) or insidious UB (if
> you aren't) because we're accidentally creating a dangling pointer.
>
> What is taking place looks something like this:
>
> 1. Create a temporary `CString` by copying `msg` to the heap and appending a
>    trailing null byte to it (that null byte is the whole reason for copying)
> 2. Get a pointer to the temporary `CString`'s heap-allocated buffer
> 3. We've reached the end of the expression, all temporaries are dropped
> 4. The result of the previous expression (our pointer) is passed to `print()`
>
> Unfortunately, step 3 will delete the `CString` and leave the pointer passed
> to `print()` pointing at garbage.
>
> Luckily this footgun will be detected by `clippy` as part of the
> [temporary_cstring_as_ptr] lint. Another subtle take on this bug was mentioned
> by `@kornel` on [the user forums].

---

> **TODO:** Write This. Mention:
>
> - strings are hard
> - UTF-8 vs WTF-16 (windows) is annoying
> - converting to a `CStr` and using `CString`
> - Returning strings (requires a destructor function, especially for
>   `CString`)

## Further Reading

- [The Complete Guide to C++ Strings, Part I - Win32 Character Encodings](https://www.codeproject.com/Articles/2995/The-Complete-Guide-to-C-Strings-Part-I-Win32-Chara)
- [Rust FFI: Sending strings to the outside world](https://thefullsnack.com/en/string-ffi-rust.html)

[`std::ffi::CStr`]: https://doc.rust-lang.org/beta/std/ffi/struct.CStr.html
[`std::ffi::CString`]: https://doc.rust-lang.org/beta/std/ffi/struct.CString.html
[temporary_cstring_as_ptr]: http://rust-lang-nursery.github.io/rust-clippy/current/index.html#temporary_cstring_as_ptr
[the user forums]: https://users.rust-lang.org/t/cstring-as-ptr-is-incredibly-unsafe/11431
