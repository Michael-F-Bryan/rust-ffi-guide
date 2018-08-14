# Callbacks

Libraries often use callbacks to notify the caller about something or as a
means of dependency injection (e.g. instead of hard-coding how to do something,
get the user to pass in a function to do the operation). We're already quite
familiar with doing this in Rust, with methods named `map`, `and_then`, or
`filter` being used all over the standard library, but callbacks are used
throughout FFI as well.

## Basic Callbacks

Imagine you're doing a long computation and want to be periodically notified
about the operation's progress, the easiest way to do this is by passing the
operation some `progress` callback.

For this first example we'll create a Rust program that calls an "expensive"
function from a C library. To simulate an expensive computation we'll just
sleep for a couple milliseconds in a loop, reporting progress on each iteration.

```c
// basic.c

{{#include basic.c}}
```

Declaring function pointers in C tends to make things hard to read, so we're
using a `typedef` to declare `Progress` as a pointer to a function that takes
in a `float` and doesn't return anything (`fn(f32)` in Rust parlance).

The Rust program then calls this `long_computation()` function, passing in a
function which will print out progress to the screen.

```rust
// basic_main.rs

{{#include basic_main.rs}}
```

Compiling and running:

```console
$ clang -shared basic.c -o libbasic.so
$ rustc basic_main.rs -L. -lbasic -o basic
$ ./basic
Starting a long computation
Progress: 0.00%
Progress: 16.67%
Progress: 33.33%
Progress: 50.00%
Progress: 66.67%
Progress: 83.33%
Computation finished, returning 36
```

> **Note:** For simplicity, we're going to ignore things like error handling
> and panics for now. *Exception Safety* is a fairly advanced topic and has its
> own dedicated chapter.

## Stateful Callbacks

Printing the progress to the screen is all well and good, but at some point
you're going to want to do more complex operations, this is where our previous
approach quickly shows its limitations.

Imagine we wanted to calculate the average of a stream of numbers. We can
use the `Progress` callback from the previous example to update our statistics,
but herein lies the problem... If the number generator function only takes a
single function pointer as an argument, where do we store the state of our
calculations?

The solution is to update our previous reporting method to accept a `void *data`
pointer which will then be passed to the progress callback on every iteration.

To see how it's done in the other direction, let's write a C program that calls
into a Rust library. On the Rust side, this doesn't look too different from our
previous example.

```rust
// stateful.rs

{{#include stateful.rs}}
```

The C side is made a little more complicated due to the presence of a new
`Statistics` struct and the accompanying `increment_statistics()` function which
will cast our `void *data` pointer back to a `Statistics*` so we can update our
statistics.

```c
// stateful_main.c

{{#include stateful_main.c}}
```

This is then compiled and executed similar to before:

```console
$ rustc stateful.rs --crate-type=cdylib
$ clang stateful_main.c -L. -lstateful -o stateful
$ ./stateful
Generating 10 numbers
received 0
received 1
received 8
received 27
received 64
received 125
received 216
received 343
received 512
received 729
Statistics:
    Count: 10
        Average: 202.5
```

You can easily imagine how this could be extended to invoking methods on Rust
types. It just requires writing a small shim function which takes a pointer to
a Rust object as its first argument, converts the pointer to a reference, and
then invokes the object's method just like normal.

If the function being called doesn't accept a data pointer then you'll need to
pass in the information via other channels (e.g. global variables).

## Rust Closures

A Rust closure isn't *just* a function pointer, meaning it's not possible to
directly pass one to C. It's also not possible to get a reference to the `Fn*()`
impl `rustc` creates for a closure, meaning we can't use it as our callback's
function pointer (and it wouldn't help us anyway due to using the wrong calling
convention).

Passing Rust closures to FFI code requires a more "creative" solution.

One solution is to write a generic function that accepts a reference to our
closure, returning a `void *` pointer to the closure's data and a special
"trampoline" function which can be used to invoke the closure.

```rust
// closures_main.rs

{{#include closures_main.rs}}
```

This works because `rustc` will *monomorphise* generics, generating a new
instance of the `trampoline` function for every different closure
`unpack_closure()` is called with.

> **Note:** C++ lambdas have a similar problem in that a lambda is more than
> just a function pointer. In order to pass C++ lambdas into something that's
> expecting a function pointer and `void*` data pointer you'll need to employ
> a similar "trampoline" trick. 

## Exercises for the Reader

- Insert a `panic!()` call in your Rust callback and see what happens when you
  run it
- Write a Rust version of the *Stateful Callbacks* example, where your
  `Statistics` is a Rust `struct` and an update method is called for every
  incoming value
- What happens when you:
  - Use the wrong calling convention for your callback?
  (e.g. `extern "stdcall" fn progress(percent: f32)`)
  - Declare the wrong number/type of parameters in the callback function's
    signature?
  - Do something which would deallocate the state object from within its
    callback?
- Read [Unboxed Closures and FFI Callbacks] for more information on using Rust
  closures with FFI
- Have a look at this [answer on StackOverflow] for an alternate way to pass
  Rust closures to C

[Unboxed Closures and FFI Callbacks]: http://aatch.github.io/blog/2015/01/17/unboxed-closures-and-ffi-callbacks/
[answer on StackOverflow]: https://stackoverflow.com/a/32270215
