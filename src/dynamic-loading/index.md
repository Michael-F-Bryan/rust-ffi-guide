# Dynamic Loading and Plugins

Dynamic loading is a really cool concept. It essentially allows you to load a
library at runtime, extract pointers to desired symbols (e.g. functions or
static variables), and use them. This is an incredibly powerful technique which
can be used when a dependency isn't exactly known at compile time (imagine it's
provided by the host OS), or if the location is to be provided by the user at
runtime (e.g. as part of a plugin system).

Depending on the operating system being used, there are two main mechanisms for
dynamically loading binary libraries, `dlopen` (unix derivatives) and
`LoadLibrary` (Windows).

> **Note:** For a comprehensive description of `dlopen` with examples and 
> documentation on how it works, you'll want to consult the **[man page]**.
> Microsoft also hosts documentation for `LoadLibrary` [on msdn].

For Rust users, the easiest way to access these APIs is via the 
[`libloading` crate]. This is a more memory-safe wrapper around the system's
dynamic library loading primitives, preventing issues like accidentally using
a function pointer after its library is unloaded from memory.

## Defining a Language-Agnostic Plugin System

To show the power and flexibility of dynamic loading, lets create a 
language-agnostic plugin system! For this scenario, imagine we're writing a
text editor and want to let third parties hook into the "save file" event (e.g.
to kick off a build).

Before we can do much more, we'll need to define the interface that all plugins
must follow:

```c
// plugin.h

{{#include plugin.h}}
```

This declares a single `Plugin` struct which is essentially a table of function
pointers and a pointer to some arbitrary data object. Our application can then
call the various functions in this [vtable] to notify the plugin whenever
something interesting happens.

> **Note:** This is almost identical to the exact pattern used by the Linux
> kernel for loading device drivers. Each driver must expose a function which
> returns a vtable (struct of function pointers) that define the various
> commands necessary for talking with a device (read, write, etc).

In order to load a plugin, all plugin libraries must expose a `plugin_register`
function that returns our plugin vtable.

## Creating A Rust Plugin

For the sake of simplicity, our plugin will track the number of words in a file
over time, printing out statistics at the end. To make things easier, we'll
create a `cargo` project instead of putting everything into one file and 
compiling it with `rustc`.

```console
$ cargo new --lib wordcount
$ cd wordcount
$ cat Cargo.toml
[package]
name = "wordcount"
version = "0.1.0"
authors = ["Michael Bryan <michaelfbryan@gmail.com>"]

[dependencies]

[lib]
crate-type = ["cdylib"]
```

First we'll write the business logic for our plugin. This is an object which
will keep track of filenames and counts for us, and do the actual counting and
reporting.

```rust
// wordcount/src/wordcount.rs

{{#include wordcount/src/wordcount.rs}}
```

Next we need to write the plugin bindings. These are essentially a bunch of
shim functions which will adapt our `WordCount` to the interface defined in
`plugin.h`.

```rust
// wordcount/src/plugin.rs

{{#include wordcount/src/plugin.rs}}
```

You can see that the shim functions have the same name as the `Plugin` field
they correspond to. We've also used the same `typedef`s as `plugin.h` to make
things more readable.

The `on_file_save()` function is a little more complicated than the rest
because we need to convert a `char*` (which is just a pointer to a bunch of
bytes) into an `&str` (a UTF-8 string). For convenience the tricky conversion
is pulled out into a macro that will return early if the file's name or
contents aren't valid UTF-8.

## Injecting Plugins into a C++ Application

To make it easier to look after plugins, manage their lifetimes, and fire the
various callbacks, we'll create a simple `PluginManager` class.

```c++
// plugin_manager.h

{{#include plugin_manager.h}}
```

The actual implementation of `PluginManager` is pretty straightforward. Similar
to what we'd do in Rust, we're using the RAII pattern to make sure plugins get
unloaded from memory in the `PluginManager`'s destructor.

```c++
// plugin_manager.cpp

{{#include plugin_manager.cpp}}
```

You probably wouldn't want to use our application as your day-to-day text 
editor, but it allows us to see how a plugin behaves over its lifetime.

```c++
// main.cpp

{{#include main.cpp}}
```

You can compile and run the above using:

```console
$ clang++ -c -std=c++11 -ldl -Wall -g plugin_manager.cpp
$ clang++ -std=c++11 -ldl -Wall -g -o main main.cpp plugin_manager.o 
$ cargo build --manifest-path wordcount/Cargo.toml 
   Compiling wordcount v0.1.0 (file:///home/michael/Documents/rust-ffi-guide/src/dynamic-loading/wordcount)
    Finished dev [unoptimized + debuginfo] target(s) in 0.58s
$ cp ./wordcount/target/debug/libwordcount.so .
$ ./main libwordcount.so
Starting Editor
Loading Plugin: libwordcount.so
word-count plugin loaded
Exiting Editor
Word count for 1 files
hello_world.txt
	1
	2
	41
	1
```

## Exercise for the Reader

- Implement a "live reload" system which will detect when a plugin library
  changes and automatically reload the plugin
- Instead of returning a C-style struct of function pointers write a Rust 
  plugin that returns a trait object, for consumption by a Rust application
- See what happens when you change the `Plugin` struct's definition in 
  `plugin.h` (i.e. by changing function signatures or adding/removing items) 
  but "forget" to update the Rust definition appropriately
- What happens when your Rust functions don't use the C calling convention?
  (i.e. leave off the `extern "C"`)
- How would you write a plugin that doesn't need to remember any state? (i.e.
  it does all necessary work in the callbacks)

[`libloading` crate]: https://github.com/nagisa/rust_libloading
[man page]: https://linux.die.net/man/3/dlopen
[on msdn]: https://msdn.microsoft.com/en-us/library/windows/desktop/ms684175(v=vs.85).aspx
[vtable]: https://en.wikipedia.org/wiki/Virtual_method_table
