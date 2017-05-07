extern crate gcc;

fn main() {
    gcc::Config::new()
        .file("src/expensive.c")
        .compile("libexpensive.a");
}
