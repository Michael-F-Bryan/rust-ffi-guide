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

## Basic Linking and Build Scripts

The general idea behind linking is that you'll tell the compiler which library
you need to link against and provide a list of directories it can check when
looking for the library.

For this section we'll be using the following C library:

```c
// add.c

{{#include add.c}}
```

It can be compiled with:

```console
$ clang -c add.c -o libadd.a
$ ls
libadd.a  add.c
$ file libadd.a
libadd.a: ELF 64-bit LSB relocatable, x86-64, version 1 (SYSV), not stripped
```

It's easy enough to write a program that calls into `libadd.a` and compile
everything using `rustc` directly.

```rust
// basic_add.rs

{{#include basic_add.rs}}
```

When compiling, we use the `-L` flag to append the current directory to
`rustc`'s library search path and the `-l` flag to link to the `add`
library.

```console
$ rustc basic_add.rs -L. -ladd
$ ./basic_add
1 + 2 = 3
```

Omitting the `-L` and `-l` flags will result in an ugly linker error (the
useful bit is `undefined reference to 'add'`).

```console
$ rustc basic_add.rs

error: linking with `cc` failed: exit code: 1
  |
    = note: "cc" "-Wl,--as-needed" "-Wl,-z,noexecstack" "-m64" "-L"
    "/home/michael/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib"
    "basic_add.basic_add0.rcgu.o" "basic_add.basic_add1.rcgu.o"
    "basic_add.basic_add2.rcgu.o" "basic_add.basic_add3.rcgu.o"
    "basic_add.basic_add4.rcgu.o" "basic_add.basic_add5.rcgu.o" "-o"
    "basic_add" "basic_add.crate.allocator.rcgu.o" "-Wl,--gc-sections" "-pie"
    "-Wl,-zrelro" "-Wl,-znow" "-nodefaultlibs" "-L"
    "/home/michael/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib"
    "-Wl,--start-group" "-Wl,-Bstatic"
    "/home/michael/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-0f63cb46932eaff0.rlib"
    "/home/michael/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_unwind-1fc163395be28b55.rlib"
    "/home/michael/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc_jemalloc-beaa60c3ca4c0fc5.rlib"
    "/home/michael/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunwind-0ce0fe5d55de4087.rlib"
    "/home/michael/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc_system-88f5405db39d2e0a.rlib"
    "/home/michael/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-55ce729794715ca0.rlib"
    "/home/michael/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-6be05a5d46417ac1.rlib"
    "/home/michael/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-bf36d45fdfb39034.rlib"
    "-Wl,--end-group"
    "/home/michael/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-e38bf62845ca7048.rlib"
    "-Wl,-Bdynamic" "-ldl" "-lrt" "-lpthread" "-lpthread" "-lgcc_s" "-lc" "-lm"
    "-lrt" "-lpthread" "-lutil" "-lutil"
      = note: /usr/sbin/ld: basic_add.basic_add1.rcgu.o: in function `basic_add::main':
                basic_add1-317d481089b8c8fe83113de504472633.rs:(.text._ZN9basic_add4main17h985357baa4d78357E+0xf):
                undefined reference to `add'
                          collect2: error: ld returned 1 exit status


                                    error: aborting due to previous error
```

But when was the last time you used `rustc` to build a real project?

The `cargo` equivalents for `rustc`'s `-L` and `-l` flags are to emit messages
from a [*Build Script*][bs]. A build script is a program that will be compiled
and executed immediately before compiling the main crate, allowing you to
execute arbitrary pre-build operations. Build scripts are named `build.rs` by
convention.

This is the bare minimum build script you'll need for linking to a native
(i.e. non-Rust) library:

```rust
// build.rs

{{#include my_real_project/build.rs}}
```

> **Note:** You'll probably want to [read the docs][bs] to find out what else
> a build script can do.

To tell `cargo` about the build script, add a `build = "build.rs"` line to your
`Cargo.toml`. It's also a good idea to add a `links = "add"` line as well,
this lets `cargo` ensure at most one crate can link to a native library.

```toml
# Cargo.toml

{{#include my_real_project/Cargo.toml::9}}
```

From here you can compile and run just like any other Rust program.

```console
$ cargo build
  ...
$ cargo run
1 + 2 = 3
```

As a bonus, on \*nix machines this setup will work for shared libraries (i.e.
dynamically linked libraries) without any changes.

## Locating Artefacts and Libraries

Now we've got a mechanism for telling `cargo`/`rustc` which libraries a crate
links to, we need to actually find *where* those libraries are and make sure
they're available at compile-time. This is one of the biggest reasons build
scripts exist.

There are a variety of ways for making sure a library is available for linking
to, for example we could

- Embed a copy of its source code in the crate (e.g. as a `git` submodule) and
  then use the local compiler to compile it from source
- Rely on the user having already installed the library and placed it in a
  well-known location (e.g. a system library installed with `apt-get`)
- Ask the user to install the library and pass its location to the build script
  via an environment variable (see [KyleMayes/llvm-sys][llvm-sys])
- Use pre-compiled binaries either distributed with a crate or downloaded on
  the fly

> **Note:** It is strongly frowned upon for a build script to affect the local
> system outside of Cargo's dedicated output directory (`OUT_DIR`) or to make
> network calls. Installing anything on the system is also a big no-no!
>
> The build process is supposed to be entirely self-contained and work
> off-line. If you can't locate an artefact then report an error or fall back
> to something else.

Which path you take will largely depend on the target platform and the library
being linked to.

Here are a few Rust crates in the wild which have already had to deal with this
problem:

- [retep998/winapi-rs](https://github.com/retep998/winapi-rs)
- [KyleMayes/llvm-sys][llvm-sys]
- [alexcrichton/git2-rs](https://github.com/alexcrichton/git2-rs)

## Import Libraries (Windows-only)

> **TODO:** Talk about this!
>
> - What even are import libraries, and how can we use them from Rust?!
> - May need to ask the guys from `retep998/winapi-rs`...

## See Also

- [Build Scripts][bs]
- [Making a `*-sys` Crate](https://kornel.ski/rust-sys-crate)
- [Where should I place a static library so I can link it with a Rust program?](https://stackoverflow.com/questions/43826572/where-should-i-place-a-static-library-so-i-can-link-it-with-a-rust-program)



[*The Classical Model for Linking*]: https://blogs.msdn.microsoft.com/oldnewthing/20091012-00/?p=16413/
[bs]: https://doc.rust-lang.org/cargo/reference/build-scripts.html
[llvm-sys]: https://github.com/KyleMayes/clang-sys#environment-variables
