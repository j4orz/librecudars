use std::env;
use std::path::PathBuf;

fn main() {
    let vendored = env::var("CARGO_FEATURE_VENDORED").is_ok();

    // 1. compile and link (cmake crate follows cargo's debug/release mode)
    let dst = cmake::Config::new("vendor/librecuda")
        .no_build_target(true)
        .build();

    // build scripts communicate with cargo via stdout cargo: commands
    println!("cargo:rustc-link-search={}", dst.display()); // -L
    println!("cargo:rustc-link-lib=driverapi"); // -l

    // 2. ffi bindgen
    let bindings = bindgen::Builder::default()
        .header("vendor/librecuda/driverapi/include/librecuda.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new())) // bindgen's rerun-if-change
        .clang_arg("-x")
        .clang_arg("c++")
        .layout_tests(false) // TODO: ??
        // allowlisting needed for c++ since bindgen cannot handle many of c++'s std::.* types
        // see: https://rust-lang.github.io/rust-bindgen/cpp.html
        .allowlist_type("LibreCU.*") // only generate bindings for librecuda api. not all c++ types
        .allowlist_function("libreCu.*")
        .allowlist_var("CU_.*")
        .allowlist_type("LibreCuDeviceAttribute")
        .allowlist_type("LibreCuFunctionAttribute")
        .generate()
        .expect("unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("couldn't write bindings");
}
