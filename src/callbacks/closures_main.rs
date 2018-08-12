use std::os::raw::{c_int, c_void};

type Progress = extern "C" fn(*mut c_void, c_int);

extern "C" {
    fn generate_numbers(upper: c_int, progress: Progress, data: *mut c_void);
}

fn main() {
    let mut total = 0;

    unsafe {
        let mut accumulator = |n: c_int| total += n;

        let (closure, callback) = unpack_closure(&mut accumulator);
        generate_numbers(20, callback, closure);
    }

    println!("The total is {}", total);
}

/// Unpack a Rust closure, extracting a `void*` pointer to the data and a
/// trampoline function which can be used to invoke it.
///
/// # Safety
///
/// It is the user's responsibility to ensure the closure outlives the returned
/// `void*` pointer.
///
/// Calling the trampoline function with anything except the `void*` pointer
/// will result in *Undefined Behaviour*.
fn unsafe unpack_closure<F>(closure: &mut F) -> (*mut c_void, Progress)
where
    F: FnMut(c_int),
{
    extern "C" fn trampoline<F>(data: *mut c_void, n: c_int)
    where
        F: FnMut(c_int),
    {
        let closure: &mut F = unsafe { &mut *(data as *mut F) };
        (*closure)(n);
    }

    (closure as *mut F as *mut c_void, trampoline::<F>)
}

