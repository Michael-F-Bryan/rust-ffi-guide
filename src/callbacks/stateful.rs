use std::os::raw::{c_int, c_void};

type Progress = fn(*mut c_void, c_int);

#[no_mangle]
pub extern "C" fn generate_numbers(upper: c_int, progress: Progress, data: *mut c_void) {
    for i in 0..upper {
        let number = i.pow(3);
        progress(data, number);
    }
}

