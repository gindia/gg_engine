extern crate bindgen;
extern crate cc;

use std::env;
use std::path::PathBuf;

fn main() {
    let bindings = bindgen::Builder::default()
        .header("src/utils.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("utils_bindings.rs"))
        .expect("Couldn't write bindings!");

    cc::Build::new()
        .file("src/utils.c")
        .compile("utils");

    println!("cargo:rerun-if-changed=build.rs");

    println!("cargo:rerun-if-changed=src/utils.h");
    println!("cargo:rerun-if-changed=src/utils.c");

    println!("cargo:rustc-flags=-lutils");
}
