fn main() {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let lib_dir = format!("{manifest_dir}/libs");

    println!("cargo:rustc-link-lib=dylib=myteams");
    println!("cargo:rustc-link-search=native={lib_dir}");
    // $ORIGIN = directory of the binary at runtime
    println!("cargo:rustc-link-arg=-Wl,-rpath,$ORIGIN/libs");
    println!("cargo:rustc-link-arg=-Wl,-rpath,{lib_dir}"); // fallback for `cargo run`
}
