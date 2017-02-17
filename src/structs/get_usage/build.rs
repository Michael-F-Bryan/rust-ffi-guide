extern crate gcc;

fn main() {
    gcc::compile_library("libusage.a", &["src/usage.c"]);
}
