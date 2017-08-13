# Making Rust Pythonic

Lets give C a rest for a while and try to speed up our Python programs. For 
this example we will build a *pythonic* interface to the amazing
[primal][primal] crate.


## Exposing The Primal API

Because of the way shared libraries and DLLs work, you can't export classes or
structs with methods attached. Instead all you get to work with are functions,
this isn't a massive issue though because when you think about it methods are
just functions who's first parameter (`self`) has a special meaning in that 
language. 

Using an object oriented design across FFI boundaries is just a case of writing 
shims on either side which will flatten methods into a bunch of **similarly 
named** functions which all take a pointer to `self` as their first parameter.
On the other side, the caller does the opposite. 

The main thing you need to be aware of is who's job it is to deallocate 
something at the end of the day. As a rule of thumb, if a language allocates 
some piece of data you need to hand it back to them so it can be deallocated. 

Without further ado, here's [some code](./pythonic/primes/src/lib.rs):


```rust
extern crate primal;
extern crate libc;

use primal::{Primes, Sieve};

#[no_mangle]
pub extern "C" fn sieve_new(limit: libc::c_uint) -> *mut Sieve {
    let s = Sieve::new(limit as usize);
    Box::into_raw(Box::new(s))
}

#[no_mangle]
pub unsafe extern "C" fn sieve_destroy(sieve: *mut Sieve) {
    if !sieve.is_null() {
        Box::from_raw(sieve);
    }
}

#[no_mangle]
pub unsafe extern "C" fn sieve_upper_bound(sieve: *const Sieve) -> libc::c_uint {
    (&*sieve).upper_bound() as libc::c_uint
}

/// Checks whether a number is prime. A non-zero response indicates `true`.
#[no_mangle]
pub unsafe extern "C" fn sieve_is_prime(sieve: *const Sieve, n: libc::c_uint) -> libc::int8_t {
    (&*sieve).is_prime(n as usize) as libc::int8_t
}


#[no_mangle]
pub extern "C" fn primes_new() -> *mut Primes {
    let iterator = Primes::all();
    Box::into_raw(Box::new(iterator))
}

#[no_mangle]
pub unsafe extern "C" fn primes_destroy(primes: *mut Primes) {
    if !primes.is_null() {
        Box::from_raw(primes);
    }
}

/// Get the next prime in the series.
///
/// # Remarks
///
/// If zero is returned then there the iterator is finished.
///
/// # Panics
///
/// If `n` is greater than `upper_bound`
#[no_mangle]
pub unsafe extern "C" fn primes_next(primes: *mut Primes) -> libc::c_uint {
    (&mut *primes).next().unwrap_or(0)
}
```

You can see that almost all of this is trivial code. Apart from the odd check 
for a `NULL` pointer, we're just allocating structs on the heap with `Box`, 
then passing back a pointer to it. `Box::into_raw()` and `Box::from_raw()` are
in charge of converting from a raw pointer to a `Box` and back again, allowing
us to take advantage of Rust's [Drop][drop] trait to clean up for us.

There are a couple strange points though, you may have noticed the use of 
`&*sieve` once or twice. This just lets you convert from a `*mut T` (or 
`*const T`) to a `&T` so you can use the object's normal methods. You need to 
explicitly convert to a borrow like this because raw pointers don't let you
use a struct's methods. likewise the `&mut *primes` converts to a mutable
borrow, the parentheses are mainly there to make it clearer to the parser what
we are doing.

If any of you have heard of the [nullable pointer optimisation][npo] you'll 
know that one way of returning (or receiving) a pointer which may be `null` is 
to represent it as an `Option<Box<T>>`. A good example of when you could use 
this is to skip the `some_ptr.is_null()` check used in the destructors and 
only recreate the `Box` if `Some` was passed in. This is more a matter of style
than anything else, the use of the `is_null()` check feels more explicit
for conveying what you're doing.

> **Hint:** when crossing the FFI boundary you tend to play fast and loose with
> your pointers and data types. You'll notice that any function 
> which is recieving raw pointers from an untrusted source (i.e. Python/C) 
> has been marked as `unsafe`. Typically you'd go to great lengths to document under what 
> conditions the user will violate memory safety. The `# Safety` and `# Remarks` headers
> in these doc-comments are used for each function which could provoke unsafe behaviour.
>
> Also, make sure you document your exported functions. From experience, working with 
> libraries with non-existent documentation is not fun. Please don't add to the problem!

The contents of [Cargo.toml](./pythonic/primes/Cargo.toml) are below:
```
[package]
name = "primes"
version = "0.1.0"
authors = ["Michael Bryan <michaelfbryan@gmail.com>"]

[dependencies]
primal = "*"
libc = "*"

[lib]
crate-type = ["cdylib", "rlib"]
```

Adding `cdylib` to the `lib` section compiles the crate as a system library
usable by other languages.

## Wrapping It With Python

The [cffi][cffi] library will be used for calling our exported functions 
from Python, it's a lot less verbose than [ctypes][ctypes] (from the standard
library), and if you have a header file handy then you essentially get FFI 
bindings for free. It also manages a lot of the low level coersion between 
Python and C types (i.e. a using a Python byte string as a `char *`).

First you'll need to make sure `cffi` is installed:

```bash
$ pip3 install cffi
```

The next python bit will be broken down into chunks to make it easier to digest. 
This is the contents of [main.py](./pythonic/main.py).


```python
import itertools
from cffi import FFI

ffi = FFI()

ffi.cdef("""
    void* sieve_new(unsigned int limit);
    void sieve_destroy(void *sieve);
    unsigned int sieve_upper_bound(void *sieve);
    unsigned int sieve_is_prime(void *sieve, unsigned int n);

    void* primes_new();
    void primes_destroy(void *primes);
    unsigned int primes_next(void *primes);
    """)

primal = ffi.dlopen('./primes/target/debug/libprimes.so')
```

Here we're importing `cffi` and declaring the functions we want to use. If you
look carefully you'll notice that this is the exact same thing you'd usually 
put in a C header file.

Next we make a nice wrapper around the `Sieve`. A [context manager][cm] is used 
to make sure that resources get initialized at the start of the `with` block, 
then they're freed again upon leaving it. This means that even if an exception
is thrown, the `Sieve` destructor will still get called.

```python
class Sieve:
    def __init__(self, limit):
        self.limit = limit
        self.sieve = None

    def __enter__(self):
        self.sieve = primal.sieve_new(self.limit)
        return self

    def __exit__(self, *args):
        primal.sieve_destroy(self.sieve)
```
A Python `Exception` is preferable over rust panicking and
unwinding, so `is_prime` checks if `n` exceeds `upper_bound`.
```Python
    def is_prime(self, n):
        if n > self.upper_bound():
            raise IndexError("{} not in upper bound {}"
                             .format(n, self.upper_bound()))
        return primal.sieve_is_prime(self.sieve, n) != 0

    def upper_bound(self):
        return primal.sieve_upper_bound(self.sieve)
```

A similar wrapper is written around the prime number iterator, converting the
repetitive `primes_next()` call into a more pythonic iterator with 
`__iter__()`.

```python
class Primes:
    def __enter__(self):
        self.iterator = primal.primes_new()
        return self

    def __exit__(self, *args):
        primal.primes_destroy(self.iterator)

    def next(self):
        return primal.primes_next(self.iterator)

    def __iter__(self):
        prime = self.next()
        while prime != 0:
            yield prime
            prime = self.next()
```

And finally the code to run it:

```python
if __name__ == "__main__":
    with Sieve(10000) as s:
        print(s.is_prime(5))

    with Primes() as p:
        n = 20
        primes = list(itertools.islice(p, n))
        print('The first {} prime numbers are {}'
              .format(n, ', '.join([str(p) for p in primes])))
```

If you were paying close attention when the Rust functions were first defined
you may have noticed the use of `void *` instead of `*mut Sieve` and
`*mut Primes`. This is sometimes known as an [opaque pointer][op] and allows 
you to pass some pointer to someone without letting them know the type or how
the thing being pointed to is laid out in memory. You can think of this as a 
form of information hiding, forcing the caller to go through just the methods
your API exposes.

> **Note:** You should have noticed how small these FFI bindings were, with most
> of them being 1 or 2 lines long. **This was no accident**. When passing 
> data across FFI boundaries all you want to do is basic sanity checks like
> `ptr.is_null()` and casts, then defer to the the relevant functions/methods.
>
> The more logic in your FFI bindings, the higher the risk of bugs. Considering
> a bug in FFI bindings has the potential for memory corruption or segfaults,
> the last thing you should be doing is putting business logic in FFI bindings.


[primal]: https://github.com/huonw/primal
[drop]: https://doc.rust-lang.org/std/ops/trait.Drop.html
[npo]: https://doc.rust-lang.org/book/ffi.html#the-nullable-pointer-optimization
[cffi]: http://cffi.readthedocs.io/en/latest/overview.html
[ctypes]: https://docs.python.org/3/library/ctypes.html
[cm]: http://eigenhombre.com/introduction-to-context-managers-in-python.html
[op]: https://en.wikipedia.org/wiki/Opaque_pointer
