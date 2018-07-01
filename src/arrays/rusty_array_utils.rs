use std::os::raw::c_int;
use std::slice;

#[no_mangle]
pub unsafe extern "C" fn sum(array: *const c_int, length: c_int) -> c_int {
    assert!(!array.is_null(), "Null pointer in sum()");

    let array: &[c_int] = slice::from_raw_parts(array, length as usize);
    array.into_iter().sum()
}
