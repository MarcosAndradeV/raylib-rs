use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Mapping between Cargo feature env suffixes (`CARGO_FEATURE_<KEY>`) and Raylib's `SUPPORT_*` CMake/C macros.
const RAYLIB_FEATURE_MAP: &[(&str, &str)] = &[
    // Modules
    ("MODULE_SHAPES", "SUPPORT_MODULE_RSHAPES"),
    ("MODULE_TEXTURES", "SUPPORT_MODULE_RTEXTURES"),
    ("MODULE_TEXT", "SUPPORT_MODULE_RTEXT"),
    ("MODULE_MODELS", "SUPPORT_MODULE_RMODELS"),
    ("MODULE_AUDIO", "SUPPORT_MODULE_RAUDIO"),
    // Image file formats (rtextures)
    ("PNG", "SUPPORT_FILEFORMAT_PNG"),
    ("BMP", "SUPPORT_FILEFORMAT_BMP"),
    ("TGA", "SUPPORT_FILEFORMAT_TGA"),
    ("JPG", "SUPPORT_FILEFORMAT_JPG"),
    ("GIF", "SUPPORT_FILEFORMAT_GIF"),
    ("QOI", "SUPPORT_FILEFORMAT_QOI"),
    ("PSD", "SUPPORT_FILEFORMAT_PSD"),
    ("DDS", "SUPPORT_FILEFORMAT_DDS"),
    ("HDR", "SUPPORT_FILEFORMAT_HDR"),
    ("PIC", "SUPPORT_FILEFORMAT_PIC"),
    ("PNM", "SUPPORT_FILEFORMAT_PNM"),
    ("KTX", "SUPPORT_FILEFORMAT_KTX"),
    ("ASTC", "SUPPORT_FILEFORMAT_ASTC"),
    ("PKM", "SUPPORT_FILEFORMAT_PKM"),
    ("PVR", "SUPPORT_FILEFORMAT_PVR"),
    ("SVG", "SUPPORT_FILEFORMAT_SVG"),
    ("IMAGE_EXPORT", "SUPPORT_IMAGE_EXPORT"),
    ("IMAGE_GENERATION", "SUPPORT_IMAGE_GENERATION"),
    // Font file formats (rtext)
    ("TTF", "SUPPORT_FILEFORMAT_TTF"),
    ("FNT", "SUPPORT_FILEFORMAT_FNT"),
    ("BDF", "SUPPORT_FILEFORMAT_BDF"),
    // Model file formats & features (rmodels)
    ("OBJ", "SUPPORT_FILEFORMAT_OBJ"),
    ("MTL", "SUPPORT_FILEFORMAT_MTL"),
    ("IQM", "SUPPORT_FILEFORMAT_IQM"),
    ("GLTF", "SUPPORT_FILEFORMAT_GLTF"),
    ("VOX", "SUPPORT_FILEFORMAT_VOX"),
    ("M3D", "SUPPORT_FILEFORMAT_M3D"),
    ("MESH_GENERATION", "SUPPORT_MESH_GENERATION"),
    ("GPU_SKINNING", "SUPPORT_GPU_SKINNING"),
    // Audio file formats (raudio)
    ("WAV", "SUPPORT_FILEFORMAT_WAV"),
    ("OGG", "SUPPORT_FILEFORMAT_OGG"),
    ("MP3", "SUPPORT_FILEFORMAT_MP3"),
    ("QOA", "SUPPORT_FILEFORMAT_QOA"),
    ("FLAC", "SUPPORT_FILEFORMAT_FLAC"),
    ("XM", "SUPPORT_FILEFORMAT_XM"),
    ("MOD", "SUPPORT_FILEFORMAT_MOD"),
    // Core flags (rcore)
    ("TRACELOG", "SUPPORT_TRACELOG"),
    ("CAMERA_SYSTEM", "SUPPORT_CAMERA_SYSTEM"),
    ("GESTURES_SYSTEM", "SUPPORT_GESTURES_SYSTEM"),
    ("RPRAND_GENERATOR", "SUPPORT_RPRAND_GENERATOR"),
    ("MOUSE_GESTURES", "SUPPORT_MOUSE_GESTURES"),
    ("SCREEN_CAPTURE", "SUPPORT_SCREEN_CAPTURE"),
    ("COMPRESSION_API", "SUPPORT_COMPRESSION_API"),
    ("AUTOMATION_EVENTS", "SUPPORT_AUTOMATION_EVENTS"),
    ("CUSTOM_FRAME_CONTROL", "SUPPORT_CUSTOM_FRAME_CONTROL"),
    ("CLIPBOARD_IMAGE", "SUPPORT_CLIPBOARD_IMAGE"),
];

fn is_feature_enabled(feature_key: &str) -> bool {
    env::var(format!("CARGO_FEATURE_{feature_key}")).is_ok()
}

pub fn main() {
    println!("cargo:rerun-if-env-changed=RAYLIB_BUILD_FROM_SOURCE");
    println!("cargo:rerun-if-changed=build.rs");

    if !cfg!(unix) {
        panic!("Unsupported platform");
    }

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let force_from_source = is_feature_enabled("FROM_SOURCE")
        || env::var("RAYLIB_BUILD_FROM_SOURCE")
            .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
            .unwrap_or(false);

    let local_src_dir = ["raylib-src", "raylib-patch"]
        .into_iter()
        .map(|d| manifest_dir.join(d))
        .find(|p| p.join("CMakeLists.txt").exists());

    let (lib_dir, include_dir) = if let Some(src_dir) = local_src_dir {
        build_raylib_with_cmake(&src_dir)
    } else if force_from_source {
        let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
        let cloned_src = out_dir.join("raylib-src");
        if !cloned_src.join("CMakeLists.txt").exists() {
            let status = Command::new("git")
                .args([
                    "clone",
                    "--depth",
                    "1",
                    "https://github.com/raysan5/raylib.git",
                    cloned_src.to_str().unwrap(),
                ])
                .status()
                .expect("Failed to execute git clone for raylib source");
            assert!(status.success(), "Failed to clone raylib repository");
        }
        build_raylib_with_cmake(&cloned_src)
    } else {
        let prebuilt_dir = manifest_dir.join("raylib-6.0_linux_amd64");
        (prebuilt_dir.join("lib"), prebuilt_dir.join("include"))
    };

    println!("cargo:rustc-link-search=native={}", lib_dir.display());
    println!("cargo:rustc-link-lib=static=raylib");

    // Link display backend libraries on Unix
    let use_wayland = is_feature_enabled("WAYLAND");
    let use_x11 = is_feature_enabled("X11") || !use_wayland;
    if use_x11 {
        println!("cargo:rustc-link-lib=X11");
    }
    if use_wayland {
        println!("cargo:rustc-link-lib=wayland-client");
        println!("cargo:rustc-link-lib=wayland-cursor");
        println!("cargo:rustc-link-lib=wayland-egl");
        println!("cargo:rustc-link-lib=xkbcommon");
    }

    let raylib_h = include_dir.join("raylib.h");
    let rlgl_h = include_dir.join("rlgl.h");
    generate_bindings("generated.rs", raylib_h.to_str().unwrap(), &include_dir);
    generate_bindings("rlgl.rs", rlgl_h.to_str().unwrap(), &include_dir);
}

fn build_raylib_with_cmake(src_dir: &Path) -> (PathBuf, PathBuf) {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let build_dir = out_dir.join("raylib-build");

    let mut configure_cmd = Command::new("cmake");
    configure_cmd
        .arg("-B")
        .arg(&build_dir)
        .arg("-S")
        .arg(src_dir)
        .arg("-DCMAKE_BUILD_TYPE=Release")
        .arg("-DCMAKE_POSITION_INDEPENDENT_CODE=ON")
        .arg("-DWITH_PIC=ON")
        .arg("-DCUSTOMIZE_BUILD=ON")
        .arg("-DBUILD_EXAMPLES=OFF");

    if is_feature_enabled("WAYLAND") {
        configure_cmd.arg("-DGLFW_BUILD_WAYLAND=ON");
    }
    if is_feature_enabled("X11") {
        configure_cmd.arg("-DGLFW_BUILD_X11=ON");
    }

    for &(feature_key, cmake_flag) in RAYLIB_FEATURE_MAP {
        let val = if is_feature_enabled(feature_key) {
            "ON"
        } else {
            "OFF"
        };
        configure_cmd.arg(format!("-D{cmake_flag}={val}"));
    }

    let status = configure_cmd
        .status()
        .expect("Failed to run cmake configuration for Raylib");
    assert!(status.success(), "CMake configuration for Raylib failed");

    let status = Command::new("cmake")
        .arg("--build")
        .arg(&build_dir)
        .arg("--config")
        .arg("Release")
        .arg("--parallel")
        .status()
        .expect("Failed to run cmake --build for Raylib");
    assert!(status.success(), "CMake build for Raylib failed");

    let lib_dir = if build_dir.join("raylib/libraylib.a").exists() {
        build_dir.join("raylib")
    } else {
        build_dir.clone()
    };

    let include_dir = if build_dir.join("raylib/include/raylib.h").exists() {
        build_dir.join("raylib/include")
    } else {
        src_dir.join("src")
    };

    (lib_dir, include_dir)
}

pub fn generate_bindings(modname: &str, header_file: &str, include_dir: &Path) {
    let mut builder = bindgen::Builder::default()
        .header(header_file)
        .clang_arg(format!("-I{}", include_dir.display()))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .derive_default(true)
        .derive_partialeq(true)
        .derive_debug(true)
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
        .blocklist_item("TextureWrap")
        .blocklist_item("TraceLogLevel")
        .blocklist_item("MouseCursor")
        .blocklist_item("GamepadButton")
        .blocklist_item("GamepadAxis")
        .blocklist_item("MaterialMapIndex")
        .blocklist_item("ShaderLocationIndex")
        .blocklist_item("ShaderUniformDataType")
        .blocklist_item("ShaderAttributeDataType")
        .blocklist_item("PixelFormat")
        .blocklist_item("CubemapLayout")
        .blocklist_item("FontType")
        .blocklist_item("Gesture")
        .blocklist_item("CameraMode")
        .blocklist_item("CameraProjection")
        .blocklist_item("NPatchLayout");

    for &(feature_key, c_macro) in RAYLIB_FEATURE_MAP {
        let val = if is_feature_enabled(feature_key) { 1 } else { 0 };
        builder = builder.clang_arg(format!("-D{c_macro}={val}"));
    }

    let bindings = builder.generate().expect("Unable to generate bindings");

    let out_path = PathBuf::from("src");
    bindings
        .write_to_file(out_path.join(modname))
        .expect("Couldn't write bindings!");
}
