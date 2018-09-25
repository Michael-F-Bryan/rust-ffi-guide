extern crate cbindgen;

use cbindgen::Language;
use std::env;
use std::path::PathBuf;

fn main() {
    let crate_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let header = crate_dir.join("tomlreader.h");

    cbindgen::Builder::new()
        .with_crate(&crate_dir)
        .with_language(Language::C)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(&header);
}
