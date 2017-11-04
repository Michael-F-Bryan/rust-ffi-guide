# Solutions

## Problem 1

> **TODO:** Explore name mangling and the `#[no_mangle]` attribute
>
> The Cherno has a [video](https://www.youtube.com/watch?v=H4s55GgAg0I) which 
> explains the linker quite well.


## Problem 2

> **TODO:** mention exception safety and why unwinding across the FFI behaviour
> is a bad idea


## Problem 3

> **TODO:** Talk about why [Cstring.as_ptr()] is unsafe because you are passing
> back a dangling pointer.

## Problem 4

> **TODO:** Talk about how it's UB to create a Rust enum from an integer with 
> an invalid variant

[CString.as_ptr()]: https://users.rust-lang.org/t/cstring-as-ptr-is-incredibly-unsafe/11431