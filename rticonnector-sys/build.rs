use std::env;
use std::path::PathBuf;

fn main() {
    // Specify the library path based on the target architecture
    let target = env::var("TARGET").unwrap();
    let lib_path = match target.as_str() {
        "x86_64-apple-darwin" => "lib/osx-x64",
        "aarch64-apple-darwin" => "lib/osx-arm64",
        "x86_64-unknown-linux-gnu" => "lib/linux-x64",
        "aarch64-unknown-linux-gnu" => "lib/linux-arm64",
        "x86_64-pc-windows-msvc" => "lib/win-x64",
        _ => panic!("Unsupported target architecture: {}", target),
    };

    let bindings = bindgen::builder().header("include/rticonnextdds-connector.h").generate().expect("Unable to generate bindings");

    // Tell Cargo where to find the dynamic libraries
    println!("cargo:rustc-link-search=native={}", lib_path);
    println!("cargo:rustc-link-lib=nddsc");
    println!("cargo:rustc-link-lib=nddscore");
    println!("cargo:rustc-link-lib=rtiddsconnector");

    // Set the runtime library search path
    let output_dir = env::var("OUT_DIR").unwrap();

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
    let runtime_path = PathBuf::from(lib_path);
    println!("cargo:rustc-env=DYLD_LIBRARY_PATH={}", runtime_path.display());

    // Include headers for the binding generation
    println!("cargo:include=include");
}