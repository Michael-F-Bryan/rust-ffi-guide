# Problems

## Problem 1

Here's an easy one to get you started. It contains a Rust library:

```rust
// adder.rs

pub extern "C" fn add(a: u32, b: u32) -> u32 {
    a + b
}
```

And a C/C++ program which uses it.

```cpp
// main.cpp

#include <iostream>
#include <cstdint>

extern "C" {
    uint32_t add(uit32_t, uit32_t);
}

int main() {
    uint32_t a = 5, b = 10;

    uint32_t sum = add(a, b);
    std::cout << "The sum of " << a 
              << " and " << b 
              << " is " << sum 
              << std::endl;
}
```

Building and running:

```
$ rustc --crate-type cdylib adder.rs
$ clang++ -std=c++14 -c main.cpp
$ clang++ -std=c++14 -o main -L. -ladder main.o
$ ./main
```


## Problem 2

This problem is similar to the previous one in that it has a Rust library called
by a C++ program.

```rust
// foo.rs

#[no_mangle]
pub extern "C" fn foo() {
    panic!("Oops...");
}
```

The main program:

```cpp
// main.cpp

extern "C" {
    void foo();
}

int main() {
    foo();
}
```

Compiling and running is also pretty similar:

```
$ rustc --crate-type cdylib foo.rs
$ clang++ -std=c++14 -c main.cpp
$ clang++ -std=c++14 -o main -L. -lfoo main.o
$ ./main
```


## Problem 3

```rust
// home.rs

use std::ffi::CString;
use std::env;
use std::ptr;
use std::os::c_char;


#[no_mangle]
pub extern "C" fn home_directory() -> *const c_char {
    let home = match env::home_dir() {
        Some(p) => p,
        None => return ptr::null(),
    };

    let c_string = match CString::new(home){
        Ok(s) => s,
        Err(_) => return ptr::null(),
    };

    c_string.as_ptr()
}
```


```cpp
// main.cpp

#include <iostream>

extern "C" {
    char *home_directory();
}

int main() {
    char* home = home_directory();

    if (home == nullptr) {
        std::cout << "Unable to find the home directory" << std::endl;
    } else {
        std::cout << "Home directory is " << home << std::endl; 
    }
}
```

Compiling and running:

```
$ rustc --crate-type cdylib home.rs
$ clang++ -std=c++14 -c main.cpp
$ clang++ -std=c++14 -o main -L. -lhome main.o
$ ./main
```


## Problem 4

```rust
// logging.rs

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
```

```cpp
// main.cpp

#include <iostream>
#include <string>

extern "C" {
void log_message(int, const char *);
}

int main() {
  std::string message = "Hello World";
  log_message(0x04 | 0x01, message.c_str());
}
```