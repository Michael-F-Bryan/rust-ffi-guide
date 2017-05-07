extern crate libc;
use libc::{c_void, c_int};

type Callback = unsafe extern "C" fn(c_int) -> c_int;
type StatefulCallback = unsafe extern "C" fn(*mut c_void, c_int) -> c_int;

extern "C" {
    fn expensive_calculation(start: c_int, cb: Callback);
    fn stateful_expensive_calculation(start: c_int, cb: StatefulCallback, state: *mut c_void);
}


unsafe extern "C" fn check_progress(intermediate_result: c_int) -> c_int {
    println!("Intermediate result: {}", intermediate_result);

    if intermediate_result > 100 {
        println!("{} number is too big!!!", intermediate_result);
        0
    } else {
        1
    }
}

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
