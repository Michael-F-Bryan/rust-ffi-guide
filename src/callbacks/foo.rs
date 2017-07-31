use std::slice;
use std::panic;

#[no_mangle]
pub unsafe extern "C" fn get_item_10000(buffer: *const u8, len: usize) -> u8 {
    let data = slice::from_raw_parts(buffer, len);
    data[10000]
}

#[no_mangle]
pub unsafe extern "C" fn safe_get_item_10000(buffer: *const u8, len: usize) -> u8 {
    panic::catch_unwind(|| get_item_10000(buffer, len)).unwrap_or(0)
}
