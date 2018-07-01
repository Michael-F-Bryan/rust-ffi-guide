# The Nuts and Bolts of FFI

Just like how you'll typically learn the different data types and programming
constructs when learning a new programming language, we're going to start off
by looking at the various day-to-day tasks you may encounter when doing FFI.

The C language is the lingua franca of the programming world, so most of this
section will focus on interoperating between C and Rust. As such you'll want to
have a decent understanding of C, in particular knowledge of pointers, linking,
and function pointers will be useful.