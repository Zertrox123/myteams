fn main() {
    let lib_dir = "/home/poulpy/epitech/G-NWP-400-LYN-4-1-myteams-5/server/libs";
    println!("cargo:rustc-link-lib=dylib=myteams");
    println!("cargo:rustc-link-search=native={lib_dir}");
    println!("cargo:rustc-link-arg=-Wl,-rpath,{lib_dir}");
}
