# Sharing Structs

Believe it or not but it's fairly easy to share structs between Rust and other 
languages. As long as you tell the Rust compiler to lay out structs "how C does 
it" (with `#[repr(C)]`) and have the right type declarations, everything *Just 
Works™*. It's all bytes at the end of the day, anyway.

Remember how I mentioned earlier that there's an easier way to compile and link
everything? Well there is, it's called [gcc-rs][gcc-rs]. All you need to do is 
point it at your C source code and it'll do all the hard parts like compiling
to a static library and then providing the correct linker args to rustc.


## Getting Resource Usage

For this example we'll be asking the kernel how many resources the current 
process is using, and to make this a lot easier to do in a platform dependent 
(-ish) manner, we'll write a small C shim that passes just the information we
care about back to Rust.

The function in particular I'd like to use is [getrusage()][getrusage], which 
is part of the GNU `libc`.

> **Note:** this example will be, quite obviously, Linux-specific. If you're on
> Mac or Windows you might want to look for some other function which returns
> a struct and play around with that. To be honest, I'm only using `getrusage()`
> because it was the first thing to pop up when I searched Google.

The first thing you'll want to do when calling one of the standard C functions
is to consult the man page:

```text
$ man getrusage
NAME
       getrusage - get resource usage

SYNOPSIS
       #include <sys/time.h>
       #include <sys/resource.h>

       int getrusage(int who, struct rusage *usage);

DESCRIPTION
       getrusage() returns resource usage measures for who, which can be one of 
       the following:

       RUSAGE_SELF
              Return resource usage statistics for the calling process, which 
              is the sum of resources used by all threads in the process.

       RUSAGE_CHILDREN
              Return  resource  usage statistics for all children of the calling 
              process that have terminated and been waited for. These 
              statistics will include the resources used by grandchildren, 
              and further removed descendants, if all of the intervening 
              descendants waited on their terminated children.  

       The resource usages are returned in the structure pointed to by usage, 
       which has the following form:

           struct rusage {
               struct timeval ru_utime; /* user CPU time used */
               struct timeval ru_stime; /* system CPU time used */
               long   ru_maxrss;        /* maximum resident set size */
               long   ru_ixrss;         /* integral shared memory size */
               long   ru_idrss;         /* integral unshared data size */
               long   ru_isrss;         /* integral unshared stack size */
               long   ru_minflt;        /* page reclaims (soft page faults) */
               long   ru_majflt;        /* page faults (hard page faults) */
               long   ru_nswap;         /* swaps */
               long   ru_inblock;       /* block input operations */
               long   ru_oublock;       /* block output operations */
               long   ru_msgsnd;        /* IPC messages sent */
               long   ru_msgrcv;        /* IPC messages received */
               long   ru_nsignals;      /* signals received */
               long   ru_nvcsw;         /* voluntary context switches */
               long   ru_nivcsw;        /* involuntary context switches */
           };

```

Obviously the `rusage` struct contains loads of juicy information about a 
process, but we only need a small subset of this so to make things easier we'll 
write a C library which calls `getrusage()` for us and only gives us the info we
want. In this case, all I care about is the resident memory, unshared stack size
and amount of time spent in user mode.

Here's the C shim I came up with:

```c
#include <sys/time.h>
#include <sys/resource.h>

typedef struct stats {
    struct timeval ru_utime;
    long ru_maxrss;
    long ru_isrss;
} stats;

int get_usage_stats(stats *output) {
    struct rusage raw_usage;
    int ret;
    
    ret = getrusage(RUSAGE_SELF, &raw_usage);

    output->ru_utime = raw_usage.ru_utime;
    output->ru_maxrss = raw_usage.ru_maxrss;
    output->ru_isrss = raw_usage.ru_ixrss;

    return ret;
}
```


## The Main Rust Program

Now we get to do the fun stuff, actually using this C shim of ours.

First you'll want to create a new crate:

```bash
$ cargo new --bin get_usage
```

Make sure the C shim is in your `src/` directory. I put it at 
[./get_usage/src/usage.c](./structs/get_usage/src/usage.c).

Next you'll want to declare the C shim and custom struct in 
[main.rs](./structs/get_usage/src/main.rs).

```rust
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
```

You'll see that I'm importing the [libc][libc] crate to help make sure the
basic integer types (e.g. `c_long` and `time_t`) match up. They also have
a definition of `timeval` too, but I felt like declaring my own because it's 
just a struct with a pair of `i64`s (for my platform anyway). As long
as the integer types and sizes match up C won't care, bytes are bytes.

Now to roll this all together there's just one more step. A build script. 

This one's surprisingly easy ([build.rs](./structs/get_usage/build.rs)):

```rust
extern crate gcc;

fn main() {
    gcc::compile_library("libusage.a", &["src/usage.c"]);
}
```

Make sure you add `gcc` and `libc` to your 
[Cargo.toml](./structs/get_usage/Cargo.toml)!

```toml
[package]
name = "get_usage"
version = "0.1.0"
authors = ["Michael Bryan <michaelfbryan@gmail.com>"]
build = "build.rs"

[dependencies]
libc = "*"

[build-dependencies]
gcc = "*"
```

And now we can finally run this thing.

```bash
$ cargo run
   Compiling libc v0.2.20
   Compiling gcc v0.3.43
   Compiling get_usage v0.1.0 
    Finished dev [unoptimized + debuginfo] target(s) in 3.51 secs
     Running `target/debug/get_usage`
Usage statistics for this process: Usage { ru_utime: Timeval { tv_sec: 0, tv_usec: 24000 }, ru_maxrss: 18732, ru_isrss: 0 }
```

Believe it or not, but the hardest part of all this wasn't the FFI, it was 
trawling through various man pages and the `libc` documentation to find out how
the `rusage` and `timeval` structs are laid out.

If you're wondering where `gcc-rs` put the C library or what symbols were
defined:

```bash
$ find -name libusage.a
./target/debug/build/get_usage-3714b2c0389134ed/out/libusage.a

$ nm $(find -name 'libusage.a')
usage.o:
                 U getrusage
0000000000000000 T get_usage_stats
```


[gcc-rs]: https://docs.rs/gcc/0.3.43/gcc/index.html
[getrusage]: https://www.gnu.org/software/libc/manual/html_node/Resource-Usage.html
[libc]: https://doc.rust-lang.org/libc/x86_64-unknown-linux-gnu/libc/
