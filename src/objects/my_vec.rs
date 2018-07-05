use std::os::raw::c_int;
use std::ptr;

#[derive(Debug)]
pub struct MyVec(Vec<c_int>);

impl Drop for MyVec {
	fn drop(&mut self) {
		// Do something "interesting" in the destructor so we know when it gets
		// called
		println!("Destroying {:?}", self);
	}
}

/// Create a new `MyVec`.
#[no_mangle]
pub extern "C" fn my_vec_new() -> *mut MyVec {
	// first we create a new MyVec
    let obj = MyVec(Vec::new());
	// then copy it to the heap (so we have a stable pointer to it)
    let boxed_obj = Box::new(obj);

	// then return a pointer to our item by converting our `Box<MyVec>` into a
	// raw pointer
    Box::into_raw(boxed_obj)
}

/// Get the number of integers inside `MyVec`.
#[no_mangle]
pub unsafe extern "C" fn my_vec_len(vec: *const MyVec) -> c_int {
    if vec.is_null() {
        return 0;
    }

    (&*vec).0.len() as c_int
}

/// Get a pointer to the array of integers inside `MyVec`.
#[no_mangle]
pub unsafe extern "C" fn my_vec_contents(vec: *mut MyVec) -> *mut c_int {
    if vec.is_null() {
        return ptr::null_mut();
    }

    let vec = &mut *vec;
    vec.0.as_mut_ptr()
}

/// Add an integer to the end of `MyVec`.
#[no_mangle]
pub unsafe extern "C" fn my_vec_push(vec: *mut MyVec, n: c_int) {
    if vec.is_null() {
        return;
    }

    let vec = &mut *vec;
    vec.0.push(n);
}

/// Destroy `MyVec` and free the underlying array of numbers.
#[no_mangle]
pub unsafe extern "C" fn my_vec_destroy(obj: *mut MyVec) {
	// as a rule of thumb, freeing a null pointer is just a noop.
    if obj.is_null() {
        return;
    }

	// convert the raw pointer back to a Box<MyVec>
    let boxed = Box::from_raw(obj);
	// then explicitly drop it (optional)
    drop(boxed);
}
