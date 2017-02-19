# FFI Best Practices 

These are some best practices I've picked up while using Rust code from other
languages at work. They most definitely should **not** be taken as gospel, and 
if you believe a point could do with improving or is just downright wrong then
please, please, please [create an issue][issue] and let me know!


## Memory Management

Because passing pointers to objects between languages is such a large part of 
FFI, memory management and memory safety is probably going to be the most error
prone part of using/creating libraries across library boundaries. 

Rust forces the developer to constantly keep memory management in mind with its
concepts of lifetimes and borrowing. This works out to be a massive advantage 
when writing FFI bindings because you constantly ask yourself questions like 
"who currently owns this bit of memory?" and "who's job is it to free this when
I'm done?". Because the compiler can no longer protect you against things like
aliased pointers and double frees the job of enforcing memory safety now falls
to **you** as the developer.

In general, you'll want to:

* Provide explicit constructors and destructors for everything
* Make sure that memory is only free'd from the language it was allocated in
* Check for null pointers. Everywhere.
* Document the assumptions which are normally enforced by the Rust type system 
  like whether a function recieving a pointer gains ownership of the data being
  pointed to or is only taking it as a reference

Where possible, try to enforce memory safety to prevent against accidental 
programming errors. That said, you should explicitly state what is valid and
will work, and if the user violates these assumptions then they're probably 
invoking undefined behaviour and they're on their own. 

I know that's not a particularly satisfying thing to say, but when you are 
working at such a low level you generally have to assume the caller knows what 
they're doing. There isn't all that much you can do against someone who's 
determined to shoot themselves in the foot though...


## Documentation

When you cross the FFI layer, you lose almost all the help you'd usually get 
from the typesystem. The only way to compensate for this is with documentation
so **make sure you document everything**. 

If a constructor returns `NULL` when it fails then that should be part of the 
doc comments. Even something as simple as this is sufficient:

```rust
/// Creates a new `Foo`. If the creation fails then this returns `null`.
#[no_mangle]
pub unsafe extern "C" foo_create() -> *mut Foo {
    // insert implementation here
}
```

Likewise, you must document all assumptions. If a function consumes the 
resource being pointed to by a pointer then you should state that. For 
example you might try to add some `Point` object to the `Foo` object we created
earlier.


```rust
/// Adds a `Point` to the `Foo` object. In the process, consuming the original
/// `Point`.
/// 
/// # Safety
///
/// This function will consume the `point` argument, trying to use it afterwards
/// is undefined behaviour and will probably result in a `use-after-free`.
#[no_mangle]
pub unsafe extern "C" foo_add_point(foo: *mut Foo, point: *mut Point) {
    let p = Box::from_raw(point);
    (&mut *foo).add_point(p);
}
```

Notice how I added an explicit `Safety` section to the doc comment? You can use
these to explicitly bring ownership and memory safety assumptions to the user's
attention. other common section names you might want to use liberally are 
`Errors` and `Examples`.

I don't know about you, but I wouldn't be too happy with the library
author if I had to manually discover this after spending a couple hours with in 
debugger trying to figure out why a particular part of my program segfaults, 
when all that could have been avoided with an extra line of documentation.

You should also make sure your documentation is easily accessible. If your crate
is published on `crates.io` then it's automatically documented on `docs.rs`, 
but if it's something internal to your company then consider setting up a git
hook which will rebuild the docs and push it to an internal server. Rust's 
built in documentation tools are some of the best in the world, so make sure to
use them.

> **Hint:** Another really useful practice is to write a header file for your
> exported symbols. If the caller is using C or C++ then they can just 
> `#include` the header file directly. Likewise, Python's [cffi][cffi] can 
> generate bindings from a header file.
> 
> Distributing a header file alongside your library goes a long way to making 
> it easier to use from other languages.


## Panics and Exceptions

Panics and exceptions should never cross an FFI boundary. **Ever**. Doing so is
undefined behaviour, and will likely result in significant amounts of pain or
time wasted in a debugger. 

If you're lucky then a thrown exception or a Rust panic will crash your program
immediately, otherwise it might go on, but with completely garbled state.

Instead you'll want to make sure that you wrap your code in a `try/catch` block 
or [catch_unwind()][catch_unwind], depending on the language. You should then
make sure to either handle that error or notify a caller/callee appropriately.

For more detail on error handling, check the Error Handling section of this 
book.
<!-- TODO: write the error handling part and provide a link to it -->


## API Design 

TODO: Talk about KISS, opaque pointers, and consistent naming 


## Tests

When you start crossing the language barrier you can no longer rely on the 
compiler to make sure you are using things in the right way. As a result, every
function should have at least one test to ensure that it works correctly under
sane conditions.

This is usually the callee's responsibility because they will often be writing
an idiomatic wrapper around the raw FFI calls to add a layer of safety and 
ergonomics.

In one of the projects I did at work I'd hack on my Rust library, using the 
built in `cargo test` to ensure functionality worked. I also distributed 
bindings for that library in another language, and alongside those bindings was
a test suite which exercised them to ensure correctness and to find any bugs 
which might result in a segfault.


## When Not To Use FFI

<!--
TODO: Rewrite this into something more legit
You need to implement your own low-level or highly-optimized code. Ideally, the functions in the C library you are wrapping will do most of the heavy lifting, but if you need to write some custom code to directly process huge arrays of numerical or binary data, you might need to write code in C or another lower-level language to get the performance you want.
You need to perform some delicate callbacks from the guest language into the host language. Although it’s sometimes possible (depending on the host language’s FFI support) to perform callbacks, some kinds of complex callback function signatures can be quite tricky to satisfy through FFI.
The library makes heavy use of compile-time or preprocessor features, such as C macros. In the case of simple macros, you may be able to reimplement its behavior as a function in your language of choice. But if the library does some serious macro-fu, you might be better off just writing a C extension. -->


## Callbacks

Allowing the foreign code to run some callback to notify the callee when 
something happens is really useful. For example, say you're writing a Python 
program which calls out to Rust for doing a really expensive calculation or 
simulation then it'd be nice for Rust to notify the Python program of its 
progress every now and then so you know it's still alive.

That said, unless you have very good reasons not to, your callback signatures 
should be simple, shallow, and well-documented. Anything more complex than 
something which takes an event type (enum) and a numerical argument, *possibly*
also returning a bool to indicate whether to continue is a code smell.

i.e. `type progress_callback = fn(e: EventType, arg: f64) -> bool`



[issue]: https://github.com/Michael-F-Bryan/rust-ffi-guide/issues/new
[catch_unwind]: https://doc.rust-lang.org/std/panic/fn.catch_unwind.html
