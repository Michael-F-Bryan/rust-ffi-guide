# More Complex Objects

While you can often get quite far using just *POD types*, there's a good chance
you'll need to work with more complex types when wanting to expose a more
complex library across the FFI boundary. There's typically no meaningful way to
represent these types across different languages, meaning we'll need to hide
them behind an [opaque pointer][op] (more on that later) and create 
FFI-compatible functions for working with the object (e.g. constructors, 
destructors, and methods).

As a rule of thumb, you can assume a type is complex if it doesn't implement
`Copy`. In particular the binary representation of the following Rust
constructs aren't currently specified, meaning we can't expose them directly
across an FFI boundary:

- Generics
- Normal Enums (you can still use `#[repr(...)]` enums that have explicit
  discriminant)
- Zero sized types
- Dynamically sized types (e.g. slices and trait objects)
- References

## Passing Around Opaque Pointers

The standard way to represent something which is a unique type while hiding the
underlying implementation is with an [opaque pointer][op]. According to 
Wikipedia:

> In computer programming, an opaque pointer is a special case of an opaque
> data type, a datatype declared to be a pointer to a record or data structure
> of some unspecified type.
> 
> ...
> 
> Opaque pointers are a way to hide the implementation details of an interface
> from ordinary clients, so that the implementation may be changed without the
> need to recompile the modules using it. This benefits the programmer as well
> since a simple interface can be created, and most details can be hidden in
> another file. This is important for providing binary code compatibility
> through different versions of a shared library, for example.

In C, the way to represent this is by forward declaring a struct.

```C
struct Foo;

void do_something(struct Foo* foo) {
    // do something interesting with a Foo...
}
```

In Rust we can use an [uninhabited type] to declare something which is
impossible to construct (although pointers to one are still valid).

```rust
enum Foo {}

unsafe fn do_something(foo: *mut Foo) {
    // do something interesting with a Foo...
}
```

> **Note:** There is also [a RFC] in the works for introducing a 
> `extern type Foo` syntax. Technically an uninhabited type can never exist,
> meaning a pointer to one is guaranteed to be invalid and could (in theory) be 
> compiled away or result in UB.

## Creating Bindings for Object Methods

Now we have a better idea of how to represent an object across the FFI boundary,
lets expose a complex Rust type (in this case, just a newtype'd `Vec<c_int>`) 
to C. The problem is there's no direct way C can interact with an opaque type,
so we'll need to export some functions to do the interaction for it.

To start with, lets write up a header file that declares the interface exported
by our `my_vec.rs` library.

```C
// my_vec.h

{{#include my_vec.h}}
```

And here's a simple program which creates a `MyVec`, adds some stuff to it,
then frees it at the end.

```C
// main.c

{{#include main.c}}
```

Now we have a better idea of the interface we're designing, we can start
putting it together. You may want to take a couple minutes to read through the
code and understand what it's doing.

```rust
// my_vec.rs

{{#include my_vec.rs}}
```

This example can then be compiled and executed with:

```console
$ rustc my_vec.rs --crate-type cdylib
$ clang main.c -L. -lmy_vec -o main
$ ./main
The initial length is 0
After pushing 42, the length is 1
Iterating over the items in my vec:
my_vec[0] = 42
Destroying MyVec([42])
```

You should have noticed that the FFI bindings, while verbose and tedious, are
extremely simple. As a general rule, **FFI bindings should strictly adhere to
the KISS principle**. These functions are inherently `unsafe` and simply act as
shims for translating parameters to native Rust types and then hand over to
some underlying method for the actual business logic.

Each function is written in a defensive style, checking for possibly invalid
parameters and usage to prevent bugs. In general, there are three schools of 
thought when it comes to this sort of error checking:

1. Detect errors as soon as possible and blow up loudly so the bug can be fixed
   (i.e. use `assert!()` and tear down the process with an error message)
2. Detect errors as soon as possible and return an "*obviously invalid*" value
   so the caller knows there was a problem (requires the caller to check for
   errors)
3. Do nothing, our FFI bindings always expect to be given valid data and it's 
   the caller's responsibility to make sure they don't give us garbage

Each method has its advantages and disadvantages, and knowing when to use which
method is a skill that needs to be learnt. As a rule of thumb you will probably
lean towards option 2, this is the FFI equivalent of returning `Result<T, E>`
or `Option<T>` to let the caller know an error happened, while option 1 is 
closer to `unwrap()`.

## A Note On Destructors

Something to be wary about is the use of destructors. Many types use `Drop` 
(or their language's equivalent) in order to clean up resources. As a rule of
thumb, each complex type `foo` should have a `foo_free()` function which is
used to destroy the object and call this destructor.

There is no guarantee that a library will use the same memory allocator as the
rest of the application. For example, what happens if C (using the system
`malloc`) tries to free memory allocated by Rust (which might link to 
`jemalloc`)? 

In the best case scenario, `malloc` may be smart enough to notice it doesn't
own the memory and silently ignore it (leaking memory). The next best scenario
is to get some sort of segfault or abort, otherwise we risk messing up the
allocator's internal bookkeeping or leaving the application in an inconsistent
state (i.e. FUBAR).

The practice of always defining a `*_free()` function fixes both of these
problems by ensuring an object is always freed by the language which created
it.

## Exercises for the Reader

To get a better understanding of what's happening, you may want to do the 
following:

- Comment out the `my_vec_destroy()` call at the end of `main.c` and see what
  happens. For bonus points, run it under valgrind.
- Use the raw pointer received from `my_vec_contents()` to add numbers at 
  arbitrary locations (e.g. `numbers[100] = 123`)
- Pass a null pointer into one of the `my_vec_*` functions

[op]: https://en.wikipedia.org/wiki/Opaque_pointer
[enum]: https://doc.rust-lang.org/reference/items/enumerations.html#custom-discriminant-values-for-field-less-enumerations
[uninhabited type]: https://doc.rust-lang.org/nomicon/exotic-sizes.html#empty-types
[a RFC]: https://github.com/rust-lang/rfcs/blob/master/text/1861-extern-types.md
