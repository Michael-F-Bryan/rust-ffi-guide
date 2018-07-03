#[repr(C)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[no_mangle]
pub extern "C" fn add_points(left: Point, right: Point) -> Point {
    Point {
        x: left.x + right.x,
        y: left.y + right.y,
    }
}
