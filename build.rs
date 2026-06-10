pub fn main() {
    // Handle UNIX
    if cfg!(unix) {
        println!("cargo:rustc-link-search=raylib-6.0_linux_amd64/lib");
        println!("cargo::rustc-link-lib=X11");
    }
    // Fail on other platforms
    else {
        panic!("Unsupported platform");
    }

    println!("cargo:rustc-link-lib=static=raylib");
}
