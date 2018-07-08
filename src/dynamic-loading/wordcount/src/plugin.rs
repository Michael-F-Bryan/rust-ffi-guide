use std::ffi::CStr;
use std::os::raw::{c_void, c_char};
use std::path::PathBuf;
use super::WordCount;

type PluginCallback = unsafe extern "C" fn(data: *mut c_void);
type PluginFileSave = unsafe extern "C" fn(data: *mut c_void, filename: *const c_char, contents: *const c_char);
type PluginName = unsafe extern "C" fn(data: *mut c_void) -> *const c_char;

#[derive(Debug)]
#[repr(C)]
pub struct Plugin {
    data: *mut c_void,
    on_plugin_load: PluginCallback,
    on_plugin_unload: PluginCallback,
    on_file_save: PluginFileSave,
    name: PluginName,
}

/// A c-style string which will be embedded in the compiled binary. This is 
/// used as the plugin's name.
static NAME: &[u8] = b"word-count\0";

#[no_mangle]
pub extern "C" fn plugin_register() -> Plugin {
    let wc = WordCount::new();
    let boxed = Box::new(wc);

    Plugin {
        data: Box::into_raw(boxed) as *mut c_void,
        on_plugin_load,
        on_plugin_unload,
        on_file_save,
        name,
    }
}



unsafe extern "C" fn on_plugin_load(_data: *mut c_void) {
    println!("word-count plugin loaded");
}

unsafe extern "C" fn on_plugin_unload(data: *mut c_void) {
    let wc = Box::from_raw(data as *mut WordCount);
    wc.report();
    drop(wc);
}

/// A helper macro to convert a C-style string into an `&str`, returning early
/// if the conversion fails (e.g. the string isn't UTF-8).
macro_rules! try_str {
    ($pointer:expr) => {
        match CStr::from_ptr($pointer).to_str() {
            Ok(s) => s,
            Err(_) => return,
        }
    }
}

unsafe extern "C" fn on_file_save(data: *mut c_void, filename: *const c_char, contents: *const c_char) {
    let wc = &mut *(data as *mut WordCount);
    let filename = try_str!(filename);
    let contents = try_str!(contents);

    wc.count_file(PathBuf::from(filename), contents);
}

unsafe extern "C" fn name(_data: *mut c_void) -> *const c_char {
    NAME.as_ptr() as *const c_char
}
