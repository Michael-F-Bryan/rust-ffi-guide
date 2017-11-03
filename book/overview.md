# Using Unsafe for Fun and Profit

Given Rust's popularity and position as a systems programming language,
you'll probably reach a point where you want to integrate a Rust module into
some existing application. This guide was created to fill the current gap in
knowledge when it comes to doing more in-depth FFI tasks than simply calling
one or two functions from a C library.

This guide is written from the perspective of someone implementing a simple REST
client. The client lets you craft custom HTTP messages and send them to some 
server, allowing you to inspect the response. It is composed of a Qt GUI which
calls out to a Rust library for all of the business logic.

We'll be using `cmake` as the host build system, deferring to `cargo` to
manage and compile the Rust components. The guide was originally written on a 
linux machine, but there is no reason why it shouldn't work on Windows or Mac, 
possibly with a couple small platform-specific tweaks (filenames, etc.).

> **TODO:** Insert final screenshot here


## Useful Links and References

Here are a couple links and resources which you may find useful along the way. 

- [Rendered Guide](https://michael-f-bryan.github.io/rust-ffi-guide/)
- [The GitHub Repo](https://github.com/Michael-F-Bryan/rust-ffi-guide)
- [Associated Docker Image](https://hub.docker.com/r/michaelfbryan/ffi-guide/)
- [Beginner's Guide to Linkers](http://www.lurklurk.org/linkers/linkers.html)
- [Foreign Function Interfaces for Fun & Industry](https://spin.atomicobject.com/2013/02/15/ffi-foreign-function-interfaces/)
- [The Rust FFI Omnibus](http://jakegoulding.com/rust-ffi-omnibus/)
- ["Unsafe Rust" chapter from *The Book*](https://doc.rust-lang.org/book/second-edition/ch19-01-unsafe-rust.html)
- [Calling Rust from C and Java](https://speakerdeck.com/dbrgn/calling-rust-from-c-and-java)


## Objectives

The end objectives of this guide are:

- Integrate `cargo` into a wider build system
- Call Rust functions from C++ (or any other language)
- Passing strings, structs, and arrays between Rust and C++
- Robust error handling and exception safety
- Creating a C interface for a Rust library
- Multithreading and asynchronous programming (because we'll need to wait for
  the server's response without blocking the UI)
- Create flexible abstractions which encapsulate common patterns used when 
  writing foreign function interfaces.

The [ffi-helpers] crate was written in parallel with this guide. It takes
advantage of the patterns and abstractions we'll come up with and allows you
to reuse them for your own application.


[ffi-helpers]: https://github.com/Michael-F-Bryan/ffi-helpers