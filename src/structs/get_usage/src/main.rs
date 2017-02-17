extern crate libc;
use std::process;

#[derive(Debug, Default)]
#[repr(C)]
struct Usage {
    ru_utime: Timeval,
    ru_maxrss: libc::c_long,
    ru_isrss: libc::c_long,
}

#[derive(Debug, Default)]
#[repr(C)]
struct Timeval {
    tv_sec: libc::time_t,
    tv_usec: libc::suseconds_t,
}

extern "C" {
    fn get_usage_stats(stats: &mut Usage) -> libc::c_int;
}

fn main() {
    let mut usage = Usage::default();

    let status = unsafe { get_usage_stats(&mut usage) };

    if status != 0 {
        println!("An error occurred! Error code: {}", status);
        process::exit(1);
    } else {
        println!("Usage statistics for this process: {:?}", usage);
    }
}
