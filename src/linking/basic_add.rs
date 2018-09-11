use std::os::raw::c_int;

extern {
    fn add(a: c_int, b: c_int) -> c_int;
}

fn main() {
    let three = unsafe { add(1, 2) };
    println!("1 + 2 = {}", three);
}
