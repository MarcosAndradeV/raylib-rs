use std::path::PathBuf;

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

    generate_bindings(
        "generated.rs",
        "raylib-6.0_linux_amd64/include/raylib.h",
    );
    generate_bindings(
        "rlgl.rs",
        "raylib-6.0_linux_amd64/include/rlgl.h",
    );
}

pub fn generate_bindings(modname: &str, header_file: &str) {
    let builder = bindgen::Builder::default()
        .header(header_file)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .blocklist_item("DEG2RAD")
        .blocklist_item("PI")
        .blocklist_item("RAD2DEG")
        .blocklist_item("__GNUC_VA_LIST")
        .blocklist_item("__bool_true_false_are_defined")
        .blocklist_item("false_")
        .blocklist_item("true_")
        .blocklist_item("KeyboardKey")
        .blocklist_item("MouseButton")
        .blocklist_item("ConfigFlags")
        .blocklist_item("BlendMode")
        .blocklist_item("TextureFilter")
        .blocklist_item("TextureWrap");

    let bindings = builder.generate().expect("Unable to generate bindings");

    let out_path = PathBuf::from("src");
    bindings
        .write_to_file(out_path.join(modname))
        .expect("Couldn't write bindings!");
}
