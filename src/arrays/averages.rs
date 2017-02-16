use std::slice;

#[no_mangle]
pub extern "C" fn average(array: *const i64, length: i32) -> f64 {
    let numbers = unsafe { slice::from_raw_parts(array, length as usize) };

    let sum = numbers.iter()
        .fold(0.0, |acc, &elem| acc + elem as f64);

    sum / numbers.len() as f64
}
