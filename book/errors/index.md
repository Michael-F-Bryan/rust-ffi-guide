# Better Error Handling

So far whenever something goes wrong we've just returned a null pointer to 
indicate failure... This isn't overly ideal. Instead, it'd be nice if we could
get some context behind an error and possibly present a nice friendly message 
to the user.

To improve our application's error handling story we're going to use several 
techniques, all of which nicely complement each other. 

We'll add in logging with the [log] crate, and the ability to initialize the
Rust logger from C/C++.

Next we'll add a mechanism which lets C callers detect when an error has
occurred by inspecting return values and then access the most recent error 
message.

We also need to make sure our FFI bindings are [Exception Safe]. This means that
any Rust panics are wholly contained to Rust and we can't accidentally unwind 
across the FFI boundary (which is UB).


[log]:https://github.com/rust-lang-nursery/log 
[Exception Safe]: https://en.wikipedia.org/wiki/Exception_safety