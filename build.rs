use std::env;
use std::path::PathBuf;

// note: build scripts communicate with cargo via stdout, where each line prefixed
// with "cargo::" is interpreted as a comamnd — all other lines are ignored. To
// debug output, invoke cargo with the -vv flag.

fn main() {
    let dst = cmake::Config::new("vendor/librecuda")
        .no_build_target(true)
        .build();

    println!("cargo:rustc-link-search={}", dst.display()); // invoke rustc with -L flag
    println!("cargo:rustc-link-lib=driverapi"); // invoke rustc with -l flag
    let bindings = bindgen::Builder::default()
        .header("vendor/librecuda/driverapi/include/librecuda.h")
        .clang_arg("-x")
        .clang_arg("c++")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new())) // invalidate crate on header change
        .generate()
        .expect("unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("couldn't write bindings");
}
