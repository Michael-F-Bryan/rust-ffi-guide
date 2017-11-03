# Asynchronous Operations

At the moment sending our request will block until it returns, meaning the 
entire GUI will lock up. This is bad both from a user experience point of view, 
and because the window is no longer responding to events so the operating system
will think it's gone into zombie mode (popping up the standard "This program is
not responding" dialog).

A much better way of doing things would be to spin the request onto a background
thread, periodically polling it and getting the result if the job is completed.


> **TODO:** create a `Task` abstraction using `futures` and 
> [futures_cpupool][cpupool] to spawn an arbitrary closure.

> **TODO:** Add the `Task` abstraction to [ffi-helpers], as well as maybe a 
> couple macros for generating the `extern "C"` functions for things like 
> polling, creation, and destruction to deal with the fact that generics aren't 
> FFI-safe.


[cpupool]: https://docs.rs/futures-cpupool/0.1.7/futures_cpupool/struct.CpuPool.html#method.spawn_fn
[ffi-helpers]: https://github.com/Michael-F-Bryan/ffi-helpers