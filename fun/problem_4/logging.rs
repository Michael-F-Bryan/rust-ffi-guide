use std::os::raw::c_char;
use std::ffi::CStr;


#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(C)]
pub enum LogLevel {
    Off = 0x00,
    Error = 0x01,
    Warn = 0x02,
    Info = 0x04,
    Debug = 0x08,
    Trace = 0x0a,
}

#[no_mangle]
pub unsafe extern "C" fn log_message(level: LogLevel, message: *const c_char) {
    if level == LogLevel::Off {
        return;
    }

    let message = CStr::from_ptr(message);
    eprintln!("{:?}: {}", level, message.to_string_lossy());
}