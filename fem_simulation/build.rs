// build.rs
extern crate bindgen;

use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    // Build the C library
    // Assuming ipctk uses a Makefile or CMake
    // Adjust the build commands as necessary
    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-changed=path/to/ipctk/");

    // Build the C library (example using CMake)
    let dst = cmake::Config::new("path/to/ipctk")
        .build();

    // Link the compiled library
    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=ipctk"); // or dynamic

    // Generate bindings
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        // Tell cargo to invalidate the built crate whenever the wrapper changes
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Include paths for the C library headers
        .clang_arg("-Ipath/to/ipctk/include")
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
