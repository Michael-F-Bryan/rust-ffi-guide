extern crate libc;
use std::process;

#[derive(Debug, Default)]
#[repr(C)]
struct Timeval {
    sec: libc::time_t,
    usec: libc::suseconds_t,
}

#[derive(Debug, Default)]
#[repr(C)]
struct Usage {
    utime: Timeval,
    maxrss: libc::c_long,
    isrss: libc::c_long,
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
