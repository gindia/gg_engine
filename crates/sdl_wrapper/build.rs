/// SDL2 build.rs

extern crate bindgen;
extern crate cc;

use std::env;
use std::path::PathBuf;

fn main() {
    let bindings = bindgen::Builder::default()
        .header("src/system.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("sdl2_bindings.rs"))
        .expect("Couldn't write bindings!");

    #[cfg(debug_assertions)]
    cc::Build::new()
        .file("src/system.c")
        .debug(true)
        .define("DEBUG", "1")
        .compile("sdl2_wrapper");

    #[cfg(not(debug_assertions))]
    cc::Build::new()
        .file("src/system.c")
        .compile("sdl2_wrapper");

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/system.c");
    println!("cargo:rerun-if-changed=src/system.h");

    println!("cargo:rustc-flags=-lSDL2 -lsdl2_wrapper");
}
