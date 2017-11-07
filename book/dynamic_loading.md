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
object (`Box<Plugin>`) with some pre-defined symbol (e.g. `_plugin_create()`). 

> **Note:** This is actually the exact pattern used by the Linux kernel for 
> loading device drivers. Each driver must expose a function which returns a
> vtable (struct of function pointers) that define the various commands 
> necessary for talking with a device (read, write, etc).

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
bound also means you need to use the appropriate synchronisation mechanisms 
(e.g. a `Mutex`).

We also define a convenience macro that users can call to export their `Plugin`
in a safe manner. This just declares a new `extern "C"` function called 
`_plugin_create()` which will call the constructor and return a new boxed 
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
    ($plugin_type:ty, $constructor:path) => {
        #[no_mangle]
        pub extern "C" fn _plugin_create() -> *mut $crate::Plugin {
            // make sure the constructor is the correct type.
            let constructor: fn() -> $plugin_type = $constructor;

            let object = constructor();
            let boxed: Box<$crate::Plugin> = Box::new(object);
            Box::into_raw(boxed)
        }
    };
}
```

Another thing we're going to need is a way to manage plugins and make sure they
are called at the appropriate time. This is usually done with a `PluginManager`.

Something we need to keep in mind is that any `Library` we load will need to 
outlive our plugins. This is because they contain the code for executing the 
various `Plugin` methods, so if the `Library` is dropped too early our plugins'
vtable could end up pointing at garbage... Which would be bad.

First lets add the struct definition and a constructor,

```rust
// client/src/plugins.rs

pub struct PluginManager {
    plugins: Vec<Box<Plugin>>,
    loaded_libraries: Vec<Library>,
}

impl PluginManager {
    pub fn new() -> PluginManager {
        PluginManager {
            plugins: Vec::new(),
            loaded_libraries: Vec::new(),
        }
    }
```

Next comes the actual plugin loading part. Make sure to add `libloading` as a 
dependency to your `Cargo.toml`, then we can use it to dynamically load the 
plugin and call the `_plugin_create()` function. We also need to make sure
the `on_plugin_load()` callback is fired so the plugin has a chance to do any
necessary initialization.

```rust
// client/src/plugins.rs

    pub unsafe fn load_plugin<P: AsRef<OsStr>>(&mut self, filename: P) -> Result<()> {
        type PluginCreate = unsafe fn() -> *mut Plugin;

        let lib = Library::new(filename.as_ref()).chain_err(|| "Unable to load the plugin")?;

        // We need to keep the library around otherwise our plugin's vtable will
        // point to garbage. We do this little dance to make sure the library
        // doesn't end up getting moved.
        self.loaded_libraries.push(lib);

        let lib = self.loaded_libraries.last().unwrap();

        let constructor: Symbol<PluginCreate> = lib.get(b"_plugin_create")
            .chain_err(|| "The `_plugin_create` symbol wasn't found.")?;
        let boxed_raw = constructor();

        let plugin = Box::from_raw(boxed_raw);
        debug!("Loaded plugin: {}", plugin.name());
        plugin.on_plugin_load();
        self.plugins.push(plugin);


        Ok(())
    }
```

Now our `PluginManager` can load plugins, we need to make sure it has methods 
for firing the various plugin callbacks.

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

    /// Unload all plugins and loaded plugin libraries, making sure to fire 
    /// their `on_plugin_unload()` methods so they can do any necessary cleanup.
    pub fn unload(&mut self) {
        debug!("Unloading plugins");

        for plugin in self.plugins.drain(..) {
            trace!("Firing on_plugin_unload for {:?}", plugin.name());
            plugin.on_plugin_unload();
        }

        for lib in self.loaded_libraries.drain(..) {
            drop(lib);
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
        if !self.plugins.is_empty() || !self.loaded_libraries.is_empty() {
            self.unload();
        }
    }
}
```

A thing to keep in mind is something called [panic-on-drop]. Basically, if the
program is panicking it'll unwind the stack, calling destructors when necessary.
However, because our `PluginManager` tries to unload plugins if it hasn't 
already, a `Plugin` who's `unload()` method **also** panics will result in a 
second panic. This usually results in aborting the entire program because your 
program is most probably FUBAR.

To prevent this, we'll want to make sure the C++ code explicitly unloads the
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
pub unsafe extern "C" fn plugin_manager_post_receive(
    pm: *mut PluginManager,
    response: *mut Response,
) {
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
pub unsafe extern "C" fn plugin_manager_load_plugin(
    pm: *mut PluginManager,
    filename: *const c_char,
) -> c_int {
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

Next we need to add a `PluginManager` wrapper class to our `wrappers.hpp`. We 
should also say that `PluginManager` is a `friend` of `Request` and `Response`
so it can access their raw pointers.

```cpp
// gui/wrappers.hpp

class Request {
  friend class PluginManager;
  ...
};

class Response {
  friend class PluginManager;
  ...
};

class PluginManager {
public:
  PluginManager();
  ~PluginManager();
  void unload();
  void pre_send(Request& req);
  void post_receive(Response& res);

private:
  ffi::PluginManager *raw;
};
```

Similar to when we were writing the Rust FFI bindings, on the C++ side you just
need to make sure the arguments are in the right shape before deferring to the
corresponding functions.

```cpp
// gui/wrappers.cpp

PluginManager::PluginManager() { raw = ffi::plugin_manager_new(); }

PluginManager::~PluginManager() { ffi::plugin_manager_destroy(raw); }

void PluginManager::unload() { ffi::plugin_manager_unload(raw); }

void PluginManager::pre_send(Request& req) {
  ffi::plugin_manager_pre_send(raw, req.raw);
}

void PluginManager::post_receive(Response& res) {
  ffi::plugin_manager_post_receive(raw, res.raw);
}
```


## Hooking Up The Plugin Manager

Now that our `PluginManager` is *finally* accessible from the GUI we can thread 
it through the request sending process.

First we'll need to add the `PluginManager` to our main window.

```diff
// gui/main_window.hpp

class MainWindow : public QMainWindow {
  ...

private:
  ...
  PluginManager pm;
};
```

Next we need to make sure that whenever we send a request we also pass it to the
plugin manager so it can do the appropriate pre/post processing.

```cpp
...

pm.pre_send(req);
Response res = req.send();
pm.post_receive(res);

...
```

We also want to make sure that plugins are unloaded when the window is closed,
the easiest way to do this is to override `MainWindow`'s `closeEvent()` method.

To do this we update the `main_window.hpp` header file:

```cpp
// gui/main_window.hpp

class MainWindow : public QMainWindow {
  ...

public:
  ...
  void closeEvent(QCloseEvent *event);
  ...
};
```

Then add the implementation to `main-window.cpp`.

```cpp
// gui/main_window.cpp

void MainWindow::closeEvent(QCloseEvent *event) { pm.unload(); }
```

Now the plugin manager is plumbed into the existing request pipeline, we need a
way of actually loading plugins at runtime. We'll use a simple [file dialog] and
button for this.

> **TODO:** Once the main UI is done, step through adding a "load plugin" button
> and hooking it up to the plugin manager.


## Lets Make A Plugin

Now we have all the plugin infrastructure set up lets actually make (and load) a
plugin! This plugin will inject a special header into each request, then if it's
also present in the response we'll remove it so it's not viewable by the end
user.

First lets create a new library.

```
$ cargo new injector-plugin
```

We also want to update the `Cargo.toml` to depend on the `client` library and 
generate a `cdylib` so it's loadable by our plugin manager. While we're at it,
add the `log` crate so we can log what's happening.

```diff
// injector-plugin/Cargo.toml

[package]
name = "injector-plugin"
version = "0.1.0"
authors = ["Michael Bryan <michaelfbryan@gmail.com>"]
+ description = "A plugin which will stealthily inject a special header into your requests."

[dependencies]
+ log = "0.3.8"
+ client = { path = "../client"}
+
+ [lib]
+ crate-type = ["cdylib", "rlib"]
```

We also want to add a `cmake` build rule so the `injector-plugin` crate is built
along with the rest of the project. The `CMakeLists.txt` file for this crate is
identical to the one we wrote for `client` so just copy that across and change
the relevant names.

```
$ cp ./client/CMakeLists.txt ./inject-plugin/CMakeLists.txt
```

Don't forget make sure `cmake` includes the `inject-plugin` directory!

```diff
# ./CMakeLists.txt

add_subdirectory(client)
+ add_subdirectory(injector-plugin)
add_subdirectory(gui)
```

And then we do a quick build as a sanity check to make sure everything built.

```
$ mkdir build && cd build
$ cmake -DCMAKE_BUILD_TYPE=Debug ..
$ make

...
```

The plugin body itself isn't overly interesting.

```rust
// injector-plugin/src/lib.rs

#[macro_use]
extern crate log;
#[macro_use]
extern crate client;


#[derive(Debug, Default)]
pub struct Injector;

impl Plugin for Injector {
    fn name(&self) -> &'static str  {
        "Header Injector"
    }

    fn on_plugin_load(&self) {
        info!("Injector loaded");
    }

    fn on_plugin_unload(&self) {
        info!("Injector unloaded");
    }

    fn pre_send(&self, req: &mut Request) {
        req.headers.set_raw("some-dodgy-header", "true");
        debug!("Injected header into Request, {:?}", req);
    }

    fn post_receive(&self, res: &mut Response) {
        debug!("Received Response");
        debug!("Headers: {:?}", res.headers);
        if res.body.len() < 100 && log_enabled!(log::LogLevel::Debug) {
            if let Ok(body) = str::from_utf8(&res.body) {
                debug!("Body: {:?}", body);
            }
        }
        res.headers.remove_raw("some-dodgy-header");
    }
}
```

Finally, to make this plugin library actually work we need to call the 
`declare_plugin!()` macro.


```rust
// injector-plugin/src/lib.rs

declare_plugin!(Injector, Injector::default);
```

If you then compile this and inspect it with our trusty `nm` tool you'll see 
that the library contains our `_plugin_create` symbol. 

```
$ cd build
$ make
$ nm injector-plugin/libinjector_plugin.so | grep ' T '

...
0000000000030820 T _plugin_create
...
```


## Running The Plugin

Now that we've got a plugin and everything is hooked up to the GUI, we can try 
it out and benefit from all the hard work put in so far.

Make sure to do one last compile,

```
$ cd build
$ make
```

Then run the GUI and load the plugin from 
`build/injector-plugin/libinjector_plugin.so`. To see what headers are sent you
can send a `GET` request to http://httpbin.org/get. With any luck you should
see something like this:

```
$ RUST_LOG=client=debug,injector_plugin=debug ./gui/gui

DEBUG:client::ffi: Loading plugin, "/home/michael/Documents/ffi-guide/build/injector-plugin/libinjector_plugin.so"
DEBUG:client::plugins: Loaded plugin: Header Injector
INFO:injector_plugin: Injector loaded
Creating the request
Sending Request
DEBUG:client::plugins: Firing pre_send hooks
DEBUG:injector_plugin: Injected header into Request, Request { destination: "http://httpbin.org/get", method: Get, headers: {"some-dodgy-header": "true"}, cookies: CookieJar { original_cookies: {}, delta_cookies: {} }, body: None }
INFO:client: Sending a GET request to http://httpbin.org/get
DEBUG:client: Sending 1 Headers
DEBUG:client: 	some-dodgy-header: true
DEBUG:client::ffi: Received Response
DEBUG:client::plugins: Firing post_receive hooks
DEBUG:injector_plugin: Received Response
DEBUG:injector_plugin: Headers: {"Connection": "keep-alive", "Server": "meinheld/0.6.1", "Date": "Tue, 07 Nov 2017 14:29:39 GMT", "Content-Type": "application/json", "Access-Control-Allow-Origin": "*", "Access-Control-Allow-Credentials": "true", "X-Powered-By": "Flask", "X-Processed-Time": "0.000864028930664", "Content-Length": "303", "Via": "1.1 vegur"}
Received Response
Body:
{
  "args": {}, 
  "headers": {
    "Accept": "*/*", 
    "Accept-Encoding": "gzip", 
    "Connection": "close", 
    "Cookie": "", 
    "Host": "httpbin.org", 
    "Some-Dodgy-Header": "true", 
    "User-Agent": "reqwest/0.8.0"
  }, 
  "origin": "122.151.115.164", 
  "url": "http://httpbin.org/get"
}

DEBUG:client::plugins: Unloading plugins
INFO:injector_plugin: Injector unloaded
```

Now if you look *very* carefully you'll see that the plugin was indeed fired at
the correct time, and `httpbin` replied saying we had `Some-Dodgy-Header` in our
headers. If you've stayed with us up to this point then give yourself a pat on
the back, you just accomplished one of the most difficult FFI tasks possible!

If dynamic loading is still confusing you, you may want to check out some of 
these links:

- [Plugins in C](https://eli.thegreenplace.net/2012/08/24/plugins-in-c)
- [Building a Simple C++ Cross-platform Plugin System](https://sourcey.com/building-a-simple-cpp-cross-platform-plugin-system/)
- [GetProcAddress (for loading DLLs on Windows)][GetProcAddress]
- [dlsym (the Linux equivalent)][dlsym]
- [Wikipedia also has a pretty accurate article on the topic][article]


[dl]: https://michael-f-bryan.github.io/rust-ffi-guide/dynamic_loading/index.html
[libloading]: https://crates.io/crates/libloading
[article]: https://en.wikipedia.org/wiki/Dynamic_loading
[GetProcAddress]: https://msdn.microsoft.com/en-us/library/windows/desktop/ms683212(v=vs.85).aspx
[dlsym]: https://linux.die.net/man/3/dlsym
[panic-on-drop]: https://www.reddit.com/r/rust/comments/4a9vu6/what_are_the_semantics_of_panicondrop/
[file dialog]: http://doc.qt.io/qt-5/qfiledialog.html