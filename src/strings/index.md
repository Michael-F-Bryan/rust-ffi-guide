# Strings


## String Arguments

Lets imagine you have an awesome Rust function which lets you count the number
of characters in any UTF-8 string, and you want to make it usable from your C
code. How would you do this?

First, lets define the Rust function:

```rust
fn count_chars(s: &str) -> usize {
    s.chars().count()
}
```

Next, a wrapper is written around `count_chars()` to make it callable from
other languages, but without taking ownership of the string passed in. 
Luckily, there's a `&str` equivalent for working with null-terminated 
strings called [CStr][cstr].

```rust
use std::ffi::CStr;
use std::os::raw::c_int;

#[no_mangle]
pub unsafe extern "C" fn count_characters(s: *const c_char) -> c_int {
    let str_slice = CStr::from_ptr(s);
    count_chars(str_slice.to_str().unwrap()) as c_int
}
```

You can see that instead of exposing the `count_chars()` function directly,
I created a thin wrapper around it which does all the relevant casting between
a `CStr` and a `&str`, and then from `usize` to `c_int`.


Calling our [shim](./strings/chars.rs) from C is then almost trivial 
([main_chars.c](./strings/main_chars.c)):

```c
#include <stdio.h>

int count_characters(char *s);

int main() {
    char *s = "hello world!";
    int num_chars = count_characters(s);

    printf("There are %d characters in \"%s\"\n", num_chars, s);
}
```


## Returning Strings

One issue I found early on is that returning strings isn't a trivial thing
to do. You need to make sure that strings are correctly deallocated after 
you're done with them. Lets consider a naive implementation:

```rust
use std::ffi::CString;
use std::os::raw::c_char;


#[no_mangle]
pub extern "C" fn version() -> *mut c_char {
    CString::new("0.1.0").unwrap().into_raw()
}
```

We don't want to end up leaking memory so the next question you need to ask 
yourself is, "how do I deallocate this once I'm done?". For this, we consult 
the docs: 

> fn into_raw(self) -> *mut c_char
> 
> Transfers ownership of the string to a C caller.
> 
> The pointer must be returned to Rust and reconstituted using from_raw to be 
> properly deallocated. Specifically, one should not use the standard C free 
> function to deallocate this string.  
> 
> Failure to call from_raw will lead to a memory leak.

Okay, that's easy enough. All we need to do is provide a generic string 
destructor for our crate which will let us clean up strings when we're done
with them.

```rust
#[no_mangle]
pub extern "C" fn string_destroy(s: *mut c_char) {
    if !s.is_null() {
        unsafe {
            CString::from_raw(s);
        }
    }
}
```

This definitely works, but what happens if the caller forgets to deallocate the
string afterwards? Or an exception is thrown? The string will never get 
destroyed and we'll just leak memory. Likewise, the caller (in this case, C) 
can't deallocate the string itself with `free()` because that's likely to leak
memory or invoke undefined behaviour.

There's also a second approach which, while possibly more difficult for a Rust
library designer, is considerably nicer to work with when you're calling it 
from other languages. Instead of the `version()` function returning a string 
which needs to be deallocated afterwards, the caller will pass our function a
buffer which the callee writes into, returning the number of bytes written.

```rust
#[no_mangle]
pub unsafe extern "C" fn version_with_buffer(buf: *mut u8, len: c_int) -> c_int {
    if buf.is_null() {
        return -1;
    }

    let mut buffer = slice::from_raw_parts_mut(buf, len as usize);
    let version_number = CString::new("0.1.0").unwrap();
    buffer.write(version_number.as_bytes_with_nul())
        .map(|n| n as c_int)
        .unwrap_or(-1)
}
```

Now anyone wanting to get the version number for my Rust library just needs to
allocate their own buffer, pass it in, then read the (UTF-8) bytes afterwards.

Here's roughly how you'd call the Rust functions from C 
([main_versions.c](./strings/main_versions.c)):

```c
#include <stdio.h>

char* version();
void string_destroy(char *s);
int version_with_buffer(char *buf, int len);


int main() {
    char *version_1 = version();
    printf("Version from method 1: %s\n", version_1);
    string_destroy(version_1);

    char buffer[10];
    version_with_buffer(buffer, 10);
    printf("Version from method 2: %s\n", buffer);
}
```

This second version is a lot more robust in the face of errors because the 
buffer is usually just an array allocated on the stack. Then if ever there's an
exception or a panic it'll automatically get deallocated when the function 
returns. It also avoids an extra heap allocation, if you're into those kinds of
micro-optimisations.


[cstr]: https://doc.rust-lang.org/std/ffi/struct.CStr.html
