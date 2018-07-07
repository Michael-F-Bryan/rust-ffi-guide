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
text editor and want to let third parties hook into the editing process.

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

> **TODO:** Write This! Mention:
>
> - `dlopen` and `LoadLibrary`
> - The [`libloading` crate]
> - Plugin systems + use a macro to "register" plugins
> - You need to use the same version of compiler when loading Rust functions
>   (binary interface is unspecified and subject to change)
> - Live reload
> - Steal ideas from the [old page] on dynamic loading


[`libloading` crate]: https://github.com/nagisa/rust_libloading
[old page]: https://github.com/Michael-F-Bryan/rust-ffi-guide/blob/80e56e297a8f17d3a722ac83bab6701ef1850567/book/dynamic_loading.md
[man page]: https://linux.die.net/man/3/dlopen
[on msdn]: https://msdn.microsoft.com/en-us/library/windows/desktop/ms684175(v=vs.85).aspx
[vtable]: https://en.wikipedia.org/wiki/Virtual_method_table
