# Using Unsafe for Fun and Profit

Given Rust's popularity and position as a systems programming language, there's
a good chance you'll want to use it with other languages at some point. Whether
you want to leverage an existing C++ library or write a Rust DLL to help speed
up some Python code, you'll find an understanding of Rust's *Foreign Function 
Interface* (FFI) invaluable.

## Overall Structure

This guide is broken up into roughly three sections. The first section shows you
the nuts and bolts of doing FFI in Rust. These are the everyday tasks like 

- calling a C function from Rust (and vice versa)
- passing around references
- exposing an object (be it a Rust struct, a C++ class, or whatever) to other 
  languages, or
- techniques which are necessary for interoperating between languages

The second section gives you a more high-level overview of FFI. This helps when
designing larger systems and helps you answer more esoteric questions like:

- "*How should I structure my API?*" 
- "*What is an ergonomic way to do error handling?*", and
- "*What are some potential footguns to look out for?*"

The third section consists of several worked examples which show how to apply
the knowledge gained in the previous two sections, and the thinking which often
goes on behind the scenes.

## Useful Links and Resources

Here are a couple links and resources which you may find useful along the way.

- [Rendered Guide](https://michael-f-bryan.github.io/rust-ffi-guide/)
- [The GitHub Repo](https://github.com/Michael-F-Bryan/rust-ffi-guide)
- [Beginner's Guide to Linkers](http://www.lurklurk.org/linkers/linkers.html)
- [Foreign Function Interfaces for Fun & Industry](https://spin.atomicobject.com/2013/02/15/ffi-foreign-function-interfaces/)
- [The Rust FFI Omnibus](http://jakegoulding.com/rust-ffi-omnibus/)
- ["Unsafe Rust" chapter from *The Book*](https://doc.rust-lang.org/book/second-edition/ch19-01-unsafe-rust.html)
- [Calling Rust from C and Java](https://speakerdeck.com/dbrgn/calling-rust-from-c-and-java)
