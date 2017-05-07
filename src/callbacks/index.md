# Callbacks and Function Pointers


Imagine you are using a third party library which does some long, expensive
calculation and lets you pass in a callback which it will run periodically so
you can monitor the calculation's progress.

```C
typedef int (*callback)(int intermediate_result);

// Calculate the factorial of `start`, calling the callback after every
// iteration to notify the user of the current progress and check whether we
// should still continue.
void expensive_calculation(int start, callback cb) {
    int result = 1;

    for (int i = 1; i <= start; ++i) {
        result *= i;

        int should_continue = cb(result);
        if (!should_continue) {
            return;
        }
    }

    return;
}
```

Roughly speaking, there are two types of callbacks you'll encounter in the
wild:

- Stateless (usually just a function or a static method)
- Stateful (closures, class methods, etc)


## Simple Callbacks

This style of callback is fairly easy to use in Rust. You just need to pass in
a function which satisfies the function signature required.

The biggest thing to worry about is ensuring the Rust callback's signature is
*exactly* the same as the one your C program is expecting. If it isn't, this is
**undefined behaviour** (roughly point [768] of the C spec) and will probably
result in you recieving garbage then segfaulting when the function returns.

First, lets make a `typedef` for the callback function.

```rust
type Callback = unsafe extern "C" fn(c_int) -> c_int;
```

Next, we'll need to tell the compiler that there's some external function,
`expensive_calculation()`, which will be linked in later on.

```rust
extern "C" {
    fn expensive_calculation(start: c_int, cb: Callback);
}
```

Now that all the boilerplate is out of the way, we can actually define our
progress checking callback. This is just a function which prints the
intermediate result to the screen and will tell the calculation to halt if it
is too big.

```rust
unsafe extern "C" fn check_progress(intermediate_result: c_int) -> c_int {
    println!("Intermediate result: {}", intermediate_result);

    if intermediate_result > 100 {
        println!("{} number is too big!!!", intermediate_result);
        0
    } else {
        1
    }
}

```

And finally, a `main()` function to drive the entire thing (see [main.rs] for
the full program).

```rust
pub fn main() {
    unsafe {
        expensive_calculation(10, check_progress);
    }
}
```

To help make sure the C function is compiled in with the rest of our Rust app,
you'll probably want to use the amazing [`gcc-rs`][gcc] crate and a build
script.

```rust
// build.rs
extern crate gcc;

fn main() {
    gcc::Config::new()
        .file("src/expensive.c")
        .compile("libexpensive.a");
}
```

After compiling and running, you should now see the following:

```
$ cargo run
Intermediate result: 1
Intermediate result: 2
Intermediate result: 6
Intermediate result: 24
Intermediate result: 120
120 number is too big!!!
```

## Stateful Callbacks

This type of callback is more difficult, although you'll probably encounter it
more frequently because a callback which has state that can be inspected by the
outside world is usually a lot more useful.

A very common example of this is event handlers in GUI applications. These are
usually methods on an object, allowing the object to "remember" every time the
callback is called.

Because there are no such thing as object methods in C, we'll need to emulate
them with a function that takes a pointer to its state as its first argument.

```C
typedef int (*stateful_callback)(void *state, int intermediate_result);
```

The `expensive_calculation()` function will also need to be updated to take in
a pointer to our state as well as the callback.

```C
void stateful_expensive_calculation(int start, stateful_callback cb, void *state) {
    int result = 1;

    for (int i = 1; i <= start; ++i) {
        result *= i;

        int should_continue = cb(state, result);
        if (!should_continue) {
            return;
        }
    }

    return;
}
```


We also need to update the type alias and `extern` declarations appropriately.

```rust
type StatefulCallback = unsafe extern "C" fn(*mut c_void, c_int) -> c_int;

extern "C" {
    fn stateful_expensive_calculation(start: c_int, cb: StatefulCallback, state: *mut c_void);
}
```

Lets create a `struct` which we can use in our stateful callback. This is just
something which will keep track of all the intermediate results passed to it,
as well as whether it aborted early.


```rust
#[derive(Debug, Default)]
struct Accumulator {
    intermediate_results: Vec<isize>,
    aborted: bool,
}

impl Accumulator {
    unsafe extern "C" fn callback(this: *mut c_void, intermediate_result: c_int) -> c_int {
        let this = &mut *(this as *mut Accumulator);

        this.intermediate_results.push(intermediate_result as _);

        if intermediate_result > 100 {
            this.aborted = true;
            0
        } else {
            1
        }
    }
}
```

> **Note:** Notice the funny pointer juggling on the first line of
> `callback()`. This is necessary to transform our `void*` into a
> `*mut Accumulator`, and then turn the mutable raw pointer into a mutable
> reference.

Now our `Accumulator` struct is defined and we have a `callback()` method which
satisfies the `StatefulCallback` function signature, we can finally get on to
using it.

`Accumulator`'s `callback()` method can be passed in like you normally would in
Rust, but to pass in a pointer to the `Accumulator` object is a bit more
involved.

First we get a mutable borrow to the object and cast it to a raw pointer (this
just points to some place on the stack), and from there it can be cast to a
`void*`. This indirection is necessary because you can only cast a borrow of
some `T` to a mutable pointer of the same type, but any raw pointer can be cast
to a raw pointer of any other type (or mutability).

```rust
pub fn main() {
    unsafe {
        println!("Running stateless callback:");
        expensive_calculation(10, check_progress);
    }

    let mut acc = Accumulator::default();

    unsafe {
        println!();
        println!("Running stateful callback:");
        stateful_expensive_calculation(10,
                                       Accumulator::callback,
                                       &mut acc as *mut Accumulator as *mut c_void);
    }

    println!("Intermediate Results: {:?}", acc.intermediate_results);

    if acc.aborted {
        println!("Calculation was aborted early");
    } else {
        println!("Calculation ran to completion");
    }
}
```

Compiling and running this updated program should now show this:

```
$ cargo run
Running stateless callback:
Intermediate result: 1
Intermediate result: 2
Intermediate result: 6
Intermediate result: 24
Intermediate result: 120
120 number is too big!!!

Running stateful callback:
Intermediate Results: [1, 2, 6, 24, 120]
Calculation was aborted early
```


[768]: http://c0x.coding-guidelines.com/6.3.2.3.html
[main.rs]: ./callbacks/app/src/main.rs
[gcc]: https://docs.rs/gcc/0.3.45/gcc/
