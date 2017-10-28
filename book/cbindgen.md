# Generating a Header File

Instead of having to constantly keep `ffi.rs` and the various `extern` blocks 
scattered through out our C++ code in sync, it'd be really nice if we could 
generate a header file that corresponds to `ffi.rs` and just `#include` that.
Fortunately there exists a tool which does exactly this called [cbindgen]!


> **TODO:** write about how you can get `cbindgen` to generate a header file in
> `build.rs` and hook it up to `cmake` so our C++ code can `#include` it.

[cbindgen]: https://github.com/eqrion/cbindgen