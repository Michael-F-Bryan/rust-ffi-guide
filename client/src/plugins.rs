use std::ffi::OsStr;
use std::any::Any;
use libloading::{Library, Symbol};

use errors::*;
use {Request, Response};


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

pub struct PluginManager {
    plugins: Vec<Box<Plugin>>,
}

impl PluginManager {
    pub fn new() -> PluginManager {
        PluginManager {
            plugins: Vec::new(),
        }
    }

    /// Load a plugin from a DLL or shared library.
    ///
    /// # Safety
    ///
    /// This function is `unsafe` because there are no guarantees that the
    /// plugin loaded will be correct. In particular, all plugins must be
    /// compiled against the *exact* same version of the library that will be
    /// loading them!
    ///
    /// Failure to ensure ABI compatibility will most probably result in UB
    /// because the vtable we expect to get (from `Box<Plugin>`) and the vtable
    /// we actually get may be completely different.
    pub unsafe fn load_plugin<P: AsRef<OsStr>>(&mut self, filename: P) -> Result<()> {
        type PluginCreate = unsafe fn() -> *mut Plugin;

        let lib = Library::new(filename.as_ref()).chain_err(|| "Unable to load the plugin")?;

        let constructor: Symbol<PluginCreate> = lib.get(b"__plugin_create")
            .chain_err(|| "The `__plugin_create` symbol wasn't found.")?;
        let boxed_raw = constructor();

        let plugin = Box::from_raw(boxed_raw);
        debug!("Loaded plugin: {}", plugin.name());
        plugin.on_plugin_load();
        self.plugins.push(plugin);

        Ok(())
    }

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

impl Drop for PluginManager {
    fn drop(&mut self) {
        if !self.plugins.is_empty() {
            self.unload();
        }
    }
}
