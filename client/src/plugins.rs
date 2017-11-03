use {Request, Response};

/// A plugin which allows you to add extra functionality to the REST client.
pub trait Plugin {
    /// Get a name describing the `Plugin`.
    fn name(&self) -> &'static str;
    /// Inspect (and possibly mutate) the request before it is sent.
    fn pre_send(&mut self, _request: &mut Request) {}
    /// Inspect and/or mutate the received response before it is displayed to
    /// the user.
    fn post_receive(&mut self, _response: &mut Response) {}
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
        pub extern "C" fn __create_plugin() -> *mut $crate::Plugin {
            // make sure the constructor is the correct type.
            let constructor: fn() -> $plugin_type = $constructor;

            let object = $constructor();
            let boxed: Box<$crate::Plugin> = Box::new(object);
            Box::into_raw(boxed)
        }
    };
}
