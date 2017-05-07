extern crate gcc;

fn main() {
    gcc::Config::new()
        .file("src/expensive.c")
        .flag("-std=c99")
        .compile("libexpensive.a");
}
