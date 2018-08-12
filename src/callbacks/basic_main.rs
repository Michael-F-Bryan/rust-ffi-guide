use std::os::raw::c_int;

type Progress = extern "C" fn(f32);

extern {
    fn long_computation(n: c_int, progress: Progress) -> c_int;
}

fn main() {
    println!("Starting a long computation");
    let ret = unsafe { long_computation(6, progress) };
    println!("Computation finished, returning {}", ret);
}

extern "C" fn progress(percent: f32) {
    println!("Progress: {:.2}%", percent);
}
