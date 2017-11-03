# Dynamic Loading & Plugins

What application wouldn't be complete without the ability to add user-defined
plugins? In this chapter we take a small detour to visit the concept of 
dynamically loading a library at runtime and registering it with our parent
application.

The end goal is to allow users to provide a shared library (DLL, `*.so`, etc) 
which contains a set of pre-defined functions. These functions will then allow
us to manipulate a request before it is sent and then manipulate/inspect the
response before displaying it to the user.

From the Rust side of things, by far the easiest way to establish this is to 
define a `Plugin` trait which does the various manipulations, then add in a 
macro users can run which will define all the `unsafe` function declarations.

Our `Plugin` trait may look something like this:

```rust
pub trait Plugin {
    fn before_send(&mut self, request: &mut Request) {
        // noop default
    }

    fn on_receive(&mut self, response: &mut Response) {
        // noop default
    }
}
```

The macro would then declare an `extern "C"` constructor which exports a trait 
object (`Box<Plugin>`) with some pre-defined symbol (e.g. "plugin_create()`). 

> **TODO:** Steal a bunch of stuff from the [dynamic loading][dl] page of the 
> original guide to show how you'd do dynamic loading on a trivial project. Then
> extend it to work with our `client` library.

[dl]: https://michael-f-bryan.github.io/rust-ffi-guide/dynamic_loading/index.html