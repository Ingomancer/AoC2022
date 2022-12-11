fn main() {
    if cfg!(windows) {
        println!("cargo:rustc-link-lib=Crypt32");
        println!("cargo:rustc-link-lib=User32");
    }
}
