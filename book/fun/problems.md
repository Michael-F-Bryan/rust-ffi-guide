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