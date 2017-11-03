# Testing

You'd typically want to be testing the application from the very beginning, but
because of the complexity of this tutorial we've left it until a later chapter 
when you are (hopefully) more familiar with the C++/Rust interop workflow.

This is that chapter.

As well as the usual unit tests which you will be accustomed to writing in your
Rust code, we want to be able to test the entire backend from end-to-end. This
would require using the C++ wrappers to send off requests under various 
conditions and making sure we get the expected behaviour.

We will cover:

- Integrating `cargo test` into `cmake`'s built-in testing facilities
- Creating C++ integration tests to exercise the entire backend under various
  conditions, including
  - The "happy path" (e.g. getting a valid web page like https://google.com/)
  - Sending requests to non-existent locations (e.g. "http://imprettysurethiswebsitedoesntexist.com/")
  - Invalid URLs (i.e. bang on your keyboard)
  - Making sure cookies and headers are actually set
  - [streaming] and [timeouts]

> **TODO:** Flesh out this chapter.

[streaming]: https://httpbin.org/stream/20
[timeouts]: https://httpbin.org/delay/10