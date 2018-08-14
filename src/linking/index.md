# Linking and Building

Up until now we've mostly been invoking `rustc` directly from the terminal,
compiling and linking from source and deliberately trying to keep things
simple. Unfortunately writing code in the real world is a bit more complicated,
forcing you to deal with different build systems, system libraries,
pre-compiled binaries, and import libraries (just to name a few).

These all work together to make it a massive pain to *just get your code to
compile*.

## So What Is Linking Anyway?

> **FIXME:** This section should either be condensed or removed altogether

Probably the best explanation of linking is found in [*The Classical Model for
Linking*]:

> Back in the days when computer programs fit into a single source file, there
> was only one step in producing an executable file: You compile it. The
> compiler takes the source code, parses it according to the rules of the
> applicable language, generates machine code, and writes it to a file, ready
> to be executed.
>
> This model had quite a following, in large part because it was ridiculously
> fast if you had a so-called *one-pass compiler*.
>
> When programs got complicated enough that you couldn't fit them all into a
> single source file, the job was split into two parts. The compiler still did
> all the heavy lifting: It parsed the source code and generated machine code,
> but if the source code referenced a symbol that was defined in another source
> file, the compiler doesn't know what memory address to generate for that
> reference. The compiler instead generated a placeholder address and included
> some metadata that said, "Hey, there is a placeholder address at offset XYZ
> for symbol ABC." And for each symbol in the file, it also generated some
> metadata that said, "Hey, in case anybody asks, I have a symbol called BCD at
> offset WXY." These "99%-compiled" files were called *object modules*.
>
> The job of the linker was to finish that last 1%. It took all the object
> module and glued the dangling bits together. If one object module said "I
> have a placeholder for symbol ABC," it went and looked for any other object
> module that said "I have a symbol called ABC," and it filled in the
> placeholder with the information about ABC, known as *resolving the external
> reference*.
>
> When all the placeholders got filled in, the linker could then write out the
> finished executable module. And if there were any placeholders left over, you
> got the dreaded *unresolved external error*.

The underlying process hasn't changed much in the last 50 years, with the main
difference being *when* symbols get resolved.

*Static Linking* is the name used when symbol references are resolved at
compile time. This requires the compiler to have access to all the necessary
object files and libraries at compile time, with the relevant chunks of code
being copied into the final executable. Because all necessary code is already
bundled into the executable, this tends to make programs more portable (no need
for the user to install libraries) and deployment is a piece of cake (you just
copy the binary to the right spot), but the binaries themselves tend to be
quite bulky.

Alternatively you can use *Dynamic Linking* to defer symbol resolution until
just before the program is executed, as it gets loaded into memory. This
requires the user to have a copy of all dynamically linked libraries (often
called *DLLs*), but tends to lead to binaries that are orders of magnitude
smaller. This also means multiple programs can use the same library, avoiding
unnecessary duplication and allowing all programs to benefit from a library
upgrade without recompilation.

## Basic Static Linking

A static library is essentially just a collection of object files. The `foo`
library will typically be called `libfoo.a` on Unix derivatives (Linux, Mac,
BSD, etc.), or `foo.lib` on Windows.

For this section we'll be using the following C library:

```c
// static.c

{{#include static.c}}
```

It can be compiled with:

```console
$ clang -c static.c -o libstatic.a
$ ls
libstatic.a  static.c
$ file libstatic.a
libstatic.a: ELF 64-bit LSB relocatable, x86-64, version 1 (SYSV), not stripped
```

The general idea behind linking is that you'll tell the compiler which library
you need to link against and provide a list of directories it can check when
looking for the library.

It's easy enough to write a program that calls into `libstatic.a` and compile
everything using `rustc` directly.

```rust
// main.rs

{{#include main.rs}}
```

When compiling, we use the `-L` flag to append the current directory to
`rustc`'s library search path and the `-l` flag to link to the `static`
library.

```console
$ rustc basic_static.rs -L. -lstatic
$ ./basic_static
1 + 2 = 3
```

But when was the last time you used `rustc` to build a real project?

The `cargo` equivalents for `rustc`'s `-L` and `-l` flags are to emit messages
from a [*Build Script*][bs]. A build script is a program that will be compiled
and executed immediately before compiling the main crate, allowing you to
execute arbitrary pre-build operations. Build scripts are named `build.rs` by
convention.

This is the bare minimum build script you'll need for linking to a static
library:

```rust
// build.rs

{{#include cargo_static/build.rs}}
```

> **Note:** You'll probably want to [read the docs][bs] to find out what else
> a build script can do.

To tell `cargo` about the build script, add a `build = "build.rs"` line to your
`Cargo.toml`. It's also a good idea to add a `links = "static"` line as well,
this lets `cargo` ensure at most one crate can link to a native library.

```toml
# Cargo.toml

{{#include cargo_static/Cargo.toml::9}}
```

From here you can compile and run just like any other Rust program.

```console
$ cargo build
  ...
$ cargo run
1 + 2 = 3
```

---

> **TODO:** Mention the following:
>
> - Static linking vs dynamic linking
> - *Import libraries* on Windows
> - Cargo build scripts
> - Linking to system libraries
> - Using pre-compiled artefacts instead of compiling from source

## See Also

- [Build Scripts][bs]
- [Making a `*-sys` Crate](https://kornel.ski/rust-sys-crate)
- [Where should I place a static library so I can link it with a Rust program?](https://stackoverflow.com/questions/43826572/where-should-i-place-a-static-library-so-i-can-link-it-with-a-rust-program)


[*The Classical Model for Linking*]: https://blogs.msdn.microsoft.com/oldnewthing/20091012-00/?p=16413/
[bs]: https://doc.rust-lang.org/cargo/reference/build-scripts.html
