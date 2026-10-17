use raylib::prelude::*;
use std::ffi::CStr;

#[test]
fn test_into_cstr_ergonomics_and_backward_compatibility() {
    let from_str_slice = "hello.jpg".into_c_string();
    assert_eq!(from_str_slice.to_str().unwrap(), "hello.jpg");

    let owned_string = String::from("world.png");
    let from_string_ref = (&owned_string).into_c_string();
    assert_eq!(from_string_ref.to_str().unwrap(), "world.png");

    let from_owned_string = owned_string.into_c_string();
    assert_eq!(from_owned_string.to_str().unwrap(), "world.png");

    let legacy_cstring = cstr!("legacy_macro.jpg");
    let from_cstring = legacy_cstring.into_c_string();
    assert_eq!(from_cstring.to_str().unwrap(), "legacy_macro.jpg");

    let c_str_ref: &CStr = from_cstring.as_c_str();
    assert_eq!(c_str_ref.into_c_string().to_str().unwrap(), "legacy_macro.jpg");
}

#[test]
#[cfg(feature = "jpg")]
fn test_jpg_image_generation_export_and_reload() {
    set_trace_log_level(LOG_WARNING);

    // 1. Generate a 16x16 solid RED image in memory (works headlessly without OpenGL window)
    let mut img = gen_image_color(16, 16, RED);
    assert!(is_image_valid(img));
    assert_eq!(img.width, 16);
    assert_eq!(img.height, 16);
    assert_eq!(get_image_color(img, 8, 8), RED);

    // 2. Export the image to a .jpg file on disk using export_image
    let temp_jpg_path = std::env::temp_dir().join("raylib_rs_test_image.jpg");
    let path_str = temp_jpg_path.to_str().unwrap();

    assert!(
        export_image(img, path_str),
        "ExportImage(.jpg) should succeed when JPG support is enabled"
    );
    assert!(temp_jpg_path.exists());

    // Verify JPEG magic header bytes (FF D8 FF)
    let jpg_bytes = std::fs::read(&temp_jpg_path).expect("Should read exported .jpg file");
    assert!(jpg_bytes.len() > 3);
    assert_eq!(&jpg_bytes[0..3], &[0xFF, 0xD8, 0xFF]);

    // 3. Test decoding JPG from memory buffer via LoadImageFromMemory
    let loaded_from_mem = load_image_from_memory(".jpg", &jpg_bytes);
    assert!(
        is_image_valid(loaded_from_mem),
        "LoadImageFromMemory(.jpg) should decode the JPEG buffer"
    );
    assert_eq!(loaded_from_mem.width, 16);
    assert_eq!(loaded_from_mem.height, 16);

    // Check decoded pixel is close to RED (JPEG is lossy compression)
    let px = get_image_color(loaded_from_mem, 8, 8);
    assert!(px.r > 200, "Expected red channel > 200, got {}", px.r);
    assert!(px.g < 80, "Expected green channel < 80, got {}", px.g);
    assert!(px.b < 90, "Expected blue channel < 90, got {}", px.b);
    unload_image(loaded_from_mem);

    // 4. Test loading JPG from disk using both &str and legacy cstr!
    let loaded_from_disk = load_image(cstr!(path_str));
    assert!(is_image_valid(loaded_from_disk));
    assert_eq!(loaded_from_disk.width, 16);
    assert_eq!(loaded_from_disk.height, 16);
    unload_image(loaded_from_disk);

    let _ = std::fs::remove_file(&temp_jpg_path);

    // 5. Test ExportImageToMemory with .png
    let png_bytes = export_image_to_memory(img, ".png")
        .expect("ExportImageToMemory(.png) should return valid PNG bytes");
    assert_eq!(&png_bytes[0..4], &[0x89, b'P', b'N', b'G']);

    // 4. Test resizing image
    image_resize(&mut img, 32, 24);
    assert_eq!(img.width, 32);
    assert_eq!(img.height, 24);

    unload_image(img);
}

#[test]
fn test_math_constants_assign_ops_and_rectangle_helpers() {
    assert!((PI - std::f32::consts::PI).abs() < 1e-6);
    assert!((180.0 * DEG2RAD - PI).abs() < 1e-5);
    assert!((PI * RAD2DEG - 180.0).abs() < 1e-4);

    assert_eq!(clamp(15.0, 0.0, 10.0), 10.0);
    assert_eq!(lerp(0.0, 100.0, 0.25), 25.0);
    assert_eq!(remap(5.0, 0.0, 10.0, 0.0, 100.0), 50.0);

    let mut v2 = Vector2::new(2.0, 4.0);
    v2 += Vector2::new(1.0, 2.0);
    assert_eq!(v2, Vector2::new(3.0, 6.0));
    v2 *= 2.0;
    assert_eq!(v2, Vector2::new(6.0, 12.0));
    v2 /= 3.0;
    assert_eq!(v2, Vector2::new(2.0, 4.0));
    v2 -= Vector2::new(1.0, 1.0);
    assert_eq!(v2, Vector2::new(1.0, 3.0));

    let mut v3 = Vector3::new(1.0, 2.0, 3.0);
    v3 += Vector3::new(2.0, 3.0, 4.0);
    assert_eq!(v3, Vector3::new(3.0, 5.0, 7.0));
    v3 *= 2.0;
    assert_eq!(v3, Vector3::new(6.0, 10.0, 14.0));

    let v4 = Vector4::new(1.0, 2.0, 2.0, 4.0);
    assert_eq!(v4.length_sq(), 1.0 + 4.0 + 4.0 + 16.0);
    assert_eq!(v4.length(), 5.0);

    let rec1 = Rectangle::new(10.0, 20.0, 100.0, 50.0);
    let rec2 = Rectangle::new(50.0, 40.0, 100.0, 50.0);
    let rec3 = Rectangle::new(200.0, 200.0, 10.0, 10.0);

    assert_eq!(rec1.center(), Vector2::new(60.0, 45.0));
    assert!(rec1.contains(Vector2::new(25.0, 35.0)));
    assert!(!rec1.contains(Vector2::new(5.0, 35.0)));
    assert!(rec1.overlaps(&rec2));
    assert!(!rec1.overlaps(&rec3));
    assert!(check_collision_recs(rec1, rec2));
    assert!(!check_collision_recs(rec1, rec3));
}

#[test]
fn test_screen_size_macro_compiles_and_runs() {
    // When no window is initialized, screen width and height are 0
    let size = screen_size!();
    assert_eq!(size, Vector2::new(0.0, 0.0));
}
