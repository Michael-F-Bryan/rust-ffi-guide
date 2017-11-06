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
    fn name(&self) -> &'static str;
    fn on_plugin_load(&self) {}
    fn pre_send(&self, _request: &mut Request) {}
    fn post_receive(&self, _response: &mut Response) {}
}
```

The macro would then declare an `extern "C"` constructor which exports a trait 
object (`Box<Plugin>`) with some pre-defined symbol (e.g. `__plugin_create()`). 

Before diving into the complexity of real code, it's probably going to be easier
if we figure out how dynamic loading works using a contrived example.


## Contrived Example

For this the function being exported doesn't need to be very interesting, seeing 
it's just an example.

```rust
#[no_mangle]
pub extern "C" fn add(a: isize, b: isize) -> isize {
    a + b
}
```

This then can then be compiled into a `cdylib`. 

> **Note:** Up uptil now it hasn't mattered whether you compile as a dynamic 
> library or a static one. However for dynamically loading a library on the fly 
> you **must** compile as a `cdylib`.

```bash
$ rustc --crate-type cdylib adder.rs
```

The symbols exported by this dynamic library can now be inspected using the
`nm` tool from GNU `binutils`. 

```bash
$ nm libadder.so | grep 'add'
00000000000005f0 T add
```

As you can see, the `add` function is exposed and fully accessible to other 
programs.


## Loading the Contrived Example

Loading a function from this library and then calling it is then surprisingly
easy. The key is to use something like the [libloading] crate. This abstracts
over the various mechanisms provided by the operating system for dynamically 
loading a library.

```rust
extern crate libloading;

use std::env;
use libloading::{Library, Symbol};
```

It's also a good idea to add a type alias for the `add()` function's signature.
This isn't required, but when things start getting more complex and having more
interesting arguments the extra readability really pays off.

```rust
type AddFunc = unsafe fn(isize, isize) -> isize;
```

The `main()` function takes the DLL as its first command line argument:

```rust
fn main() {
    let library_path = env::args().nth(1).expect("USAGE: loading <LIB>");
    println!("Loading add() from {}", library_path);
```

Loads the library and gets a symbol (casting the function pointer so it has the
desired signature)

``` rust
    let lib = Library::new(library_path).unwrap();

    unsafe {
        let func: Symbol<AddFunc> = lib.get(b"add").unwrap();
```

Then you can finally call the imported function.

``` rust
        let answer = func(1, 2);
        println!("1 + 2 = {}", answer);
    }
}
```

Now compiling and running with cargo gives exactly what we'd expect:

```bash
$ cargo run -- ../libadder.so
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/loading ../libadder.so`
Loading add() from ../libadder.so
1 + 2 = 3
```

The entire `main.rs` looks like this:

```rust
extern crate libloading;

use std::env;
use libloading::{Library, Symbol};

type AddFunc = fn(isize, isize) -> isize;

fn main() {
    let library_path = env::args().nth(1).expect("USAGE: loading <LIB>");
    println!("Loading add() from {}", library_path);

    let lib = Library::new(library_path).unwrap();

    unsafe {
        let func: Symbol<AddFunc> = lib.get(b"add").unwrap();

        let answer = func(1, 2);
        println!("1 + 2 = {}", answer);
    }
}
```


## What Just Happened?

Although the example `main.rs` looks quite simple, there's a surprising amount 
of complexity going on behind the scenes!

First we're creating a shared object (or DLL, depending on your platform) which
exposes an `add` function, like we've been doing for the Rust-C++ interop in the
previous chapters.

Next, at runtime we use the `libloading` crate to load the library. This uses
whatever mechanism (usually called the `loader`) is exposed by the Operating 
System to load a library into the address space of the currently running 
process. It will also make sure to load any dependencies our library may have.

Now that the library is in memory we can call into the various functions it may
contain, however before you can call a function from the library you need to 
find its address (otherwise how does the computer know where to jump to?). This
is where the `Library`'s `get()` method comes in. It takes in byte string and
will try to find the symbol with that name (typically by calling 
[GetProcAddress] on Windows or [dlsym] on Linux).

Now that we have an address we can cast it to whatever we want, in this case an
`unsafe fn(isize, isize) -> isize`. This is quite obviously going to be an 
extremely `unsafe` operation, because there's nothing stopping us from using the
function with the wrong signature and invoking UB. 

There are also no guarantees that the address we are given may be valid. If
the library is later unloaded from memory the address will now be pointing
into memory we don't own. This means calling a library's function after it is
unloaded will be the equivalent of a use-after-free. Fortunately,
`libloading` uses Rust's concept of lifetimes to make sure it's impossible
for something like that to happen.

For more details, Wikipedia has a very informative [article] on dynamic loading. 


## Setting Up Plugins

Now that we have a better understanding of how dynamically loading a library on
the fly works, we can start adding plugins to our application.

First we'll define a `Plugin` trait which all plugins must implement. This has
been copied pretty much verbatim from the beginning of the chapter.

```rust
// client/src/plugins.rs

/// A plugin which allows you to add extra functionality to the REST client.
pub trait Plugin: Any + Send + Sync {
    /// Get a name describing the `Plugin`.
    fn name(&self) -> &'static str;
    /// A callback fired immediately after the plugin is loaded. Usually used 
    /// for initialization.
    fn on_plugin_load(&self) {}
    /// A callback fired immediately before the plugin is unloaded. Use this if
    /// you need to do any cleanup.
    fn on_plugin_unload(&self) {}
    /// Inspect (and possibly mutate) the request before it is sent.
    fn pre_send(&self, _request: &mut Request) {}
    /// Inspect and/or mutate the received response before it is displayed to
    /// the user.
    fn post_receive(&self, _response: &mut Response) {}
}
```

This is all pretty standard. Notice that the `Plugin` *must* be sendable between
threads and that all callbacks take `&self` instead of `&mut self`. This means
that any mutation must be done using interior mutability. the `Send + Sync` 
bound also means that this mutation must be done using the appropriate 
synchronisation mechanisms (e.g. a `Mutex`).

We also define a convenience macro that users can call to export their `Plugin`
in a safe manner. This just declares a new `extern "C"` function called 
`__plugin_create()` which will call the constructor and return a new boxed 
`Plugin`.

```rust
// client/src/plugins.rs

/// Declare a plugin type and its constructor.
///
/// # Notes
///
/// This works by automatically generating an `extern "C"` function with a
/// pre-defined signature and symbol name. Therefore you will only be able to
/// declare one plugin per library.
#[macro_export]
macro_rules! declare_plugin {
    ($plugin_type:ty, $constructor:ident) => {
        #[no_mangle]
        pub extern "C" fn __plugin_create() -> *mut $crate::Plugin {
            // make sure the constructor is the correct type.
            let constructor: fn() -> $plugin_type = $constructor;

            let object = $constructor();
            let boxed: Box<$crate::Plugin> = Box::new(object);
            Box::into_raw(boxed)
        }
    };
}
```

Another thing we're going to need is a way to manage plugins and make sure they
are called at the appropriate time. This is usually done with a `PluginManager`.

First lets add the struct definition and a constructor,

```rust
// client/src/plugins.rs

pub struct PluginManager {
    plugins: Vec<Box<Plugin>>,
}

impl PluginManager {
    pub fn new() -> PluginManager {
        PluginManager {
            plugins: Vec::new(),
        }
    }
```

Next comes the actual plugin loading part. Make sure to add `libloading` as a 
dependency to your `Cargo.toml`, then we can use it to dynamically load the 
plugin and then call the `__plugin_create()` function. We also need to make sure
the `on_plugin_load()` callback is fired so the plugin has a chance to do any
necessary initialization.

```rust
// client/src/plugins.rs

    pub fn load_plugin<P: AsRef<Path>>(&mut self, filename: P) -> Result<()> {
        type PluginCreate = unsafe fn() -> *mut Plugin;

        let lib = Library::new(filename.as_ref())
            .chain_err(|| "Unable to load the plugin")?;

        unsafe {
            let constructor: Symbol<PluginCreate> = lib.get(b"__plugin_create")
                .chain_err(|| "The `__plugin_create` symbol wasn't found.")?;
            let boxed_raw = constructor();

            let plugin = Box::from_raw(boxed_raw);
            debug!("Loaded plugin: {}", plugin.name());
            plugin.on_plugin_load();
            self.plugins.push(plugin);
        }

        Ok(())
    }
```

Now that the hard part is out of the way, we just need to make sure our 
`PluginManager` has methods for firing the various plugin callbacks at the 
appropriate time.

```rust
// client/src/plugins.rs

    /// Iterate over the plugins, running their `pre_send()` hook.
    pub fn pre_send(&mut self, request: &mut Request) {
        debug!("Firing pre_send hooks");

        for plugin in &mut self.plugins {
            trace!("Firing pre_send for {:?}", plugin.name());
            plugin.pre_send(request);
        }
    }

    /// Iterate over the plugins, running their `post_receive()` hook.
    pub fn post_receive(&mut self, response: &mut Response) {
        debug!("Firing post_receive hooks");

        for plugin in &mut self.plugins {
            trace!("Firing post_receive for {:?}", plugin.name());
            plugin.post_receive(response);
        }
    }

    /// Unload all plugins, making sure to fire their `on_plugin_unload()` 
    /// methods so they can do any necessary cleanup.
    pub fn unload(&mut self) {
        debug!("Unloading plugins");

        for plugin in self.plugins.drain(..) {
            trace!("Firing on_plugin_unload for {:?}", plugin.name());
            plugin.on_plugin_unload();
        }
    }
}
```

Those last three methods should be fairly self-explanatory.

Something else we may want to do is add a `Drop` impl so that our plugins are
always unloaded when the `PluginManager` gets dropped. This gives them a chance
to do any necessary cleanup.


```rust
// client/src/plugins.rs

impl Drop for PluginManager {
    fn drop(&mut self) {
        if !self.plugins.is_empty() {
            self.unload();
        }
    }
}
```

A thing to keep in mind is something called [panic-on-drop]. Basically, if the
program is panicking it'll unwind the stack, calling destructors when necessary.
However, because our `PluginManager` tries to unload plugins if it hasn't 
already, a `Plugin` who's `unload()` method **also** panics will result in a 
second panic. This usually results in aborting the entire program (because your 
program is most probably FUBAR).

To deal with this, we'll want to make sure the C++ code explicitly unloads the
plugin manager before destroying it.


## Writing C++ Bindings

As usual, once we've added a piece of functionality to the core Rust crate we'll
need to expose it to C++ in our `ffi` module, then add the C++ bindings to 
`wrappers.cpp`.

Writing FFI bindings should be quite familiar by now. All you are doing is 
converting raw pointers into references, then calling a method.

```rust
// client/src/ffi.rs

/// Create a new `PluginManager`.
#[no_mangle]
pub extern "C" fn plugin_manager_new() -> *mut PluginManager {
    Box::into_raw(Box::new(PluginManager::new()))
}

/// Destroy a `PluginManager` once you are done with it.
#[no_mangle]
pub unsafe extern "C" fn plugin_manager_destroy(pm: *mut PluginManager) {
    if !pm.is_null() {
        let pm = Box::from_raw(pm);
        drop(pm);
    }
}

/// Unload all loaded plugins.
#[no_mangle]
pub unsafe extern "C" fn plugin_manager_unload(pm: *mut PluginManager) {
    let pm = &mut *pm;
    pm.unload();
}

/// Fire the `pre_send` plugin hooks.
#[no_mangle]
pub unsafe extern "C" fn plugin_manager_pre_send(pm: *mut PluginManager, request: *mut Request) {
    let pm = &mut *pm;
    let request = &mut *request;
    pm.pre_send(request);
}

/// Fire the `post_receive` plugin hooks.
#[no_mangle]
pub unsafe extern "C" fn plugin_manager_post_receive(pm: *mut PluginManager, response: *mut Response) {
    let pm = &mut *pm;
    let response = &mut *response;
    pm.post_receive(response);
}
```

Plugin loading is a bit more interesting because we need to convert a 
`*const c_char` into a `&str`, but other than that it's all pretty 
straightforward.

```rust
// client/src/ffi.rs

#[no_mangle]
pub unsafe extern "C" fn plugin_manager_load_plugin(pm: *mut PluginManager, filename: *const c_char) -> c_int {
    let pm = &mut *pm;
    let filename = CStr::from_ptr(filename);
    let filename_as_str = match filename.to_str() {
        Ok(s) => s,
        Err(_) => {
            // TODO: proper error handling
            return -1;
        }
    };

    // TODO: proper error handling and catch_unwind
    match pm.load_plugin(filename_as_str) {
        Ok(_) => 0,
        Err(_) => -1,
    }
}
```

---

Useful Links:

- [Plugins in C](https://eli.thegreenplace.net/2012/08/24/plugins-in-c)
- [Building a Simple C++ Cross-platform Plugin System](https://sourcey.com/building-a-simple-cpp-cross-platform-plugin-system/)


> **Note:** This is actually the exact pattern used by the Linux kernel for 
> loading device drivers. Each driver must expose a function which returns a
> vtable (struct of function pointers) that define the various commands 
> necessary for talking with a device (read, write, etc).


[dl]: https://michael-f-bryan.github.io/rust-ffi-guide/dynamic_loading/index.html
[libloading]: https://crates.io/crates/libloading
[article]: https://en.wikipedia.org/wiki/Dynamic_loading
[GetProcAddress]: https://msdn.microsoft.com/en-us/library/windows/desktop/ms683212(v=vs.85).aspx
[dlsym]: https://linux.die.net/man/3/dlsym
[panic-on-drop]: https://www.reddit.com/r/rust/comments/4a9vu6/what_are_the_semantics_of_panicondrop/