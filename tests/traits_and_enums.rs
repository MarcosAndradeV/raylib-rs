use raylib::*;

#[test]
fn test_generated_structures_default() {
    let v2 = Vector2::default();
    assert_eq!(v2.x, 0.0);
    assert_eq!(v2.y, 0.0);

    let v3 = Vector3::default();
    assert_eq!(v3.x, 0.0);
    assert_eq!(v3.y, 0.0);
    assert_eq!(v3.z, 0.0);

    let v4 = Vector4::default();
    assert_eq!(v4.x, 0.0);
    assert_eq!(v4.y, 0.0);
    assert_eq!(v4.z, 0.0);
    assert_eq!(v4.w, 0.0);

    let col = Color::default();
    assert_eq!(col.r, 0);
    assert_eq!(col.g, 0);
    assert_eq!(col.b, 0);
    assert_eq!(col.a, 0);

    let rec = Rectangle::default();
    assert_eq!(rec.x, 0.0);
    assert_eq!(rec.y, 0.0);
    assert_eq!(rec.width, 0.0);
    assert_eq!(rec.height, 0.0);

    let mat = Matrix::default();
    assert_eq!(mat.m0, 0.0);
    assert_eq!(mat.m15, 0.0);

    let img = Image::default();
    assert!(img.data.is_null());
    assert_eq!(img.width, 0);
    assert_eq!(img.height, 0);
}

#[test]
fn test_generated_structures_partial_eq() {
    let v2_a = Vector2 { x: 10.0, y: 20.0 };
    let v2_b = Vector2 { x: 10.0, y: 20.0 };
    let v2_c = Vector2 { x: 15.0, y: 20.0 };
    assert_eq!(v2_a, v2_b);
    assert_ne!(v2_a, v2_c);

    let col_a = Color { r: 255, g: 0, b: 0, a: 255 };
    let col_b = Color { r: 255, g: 0, b: 0, a: 255 };
    let col_c = Color { r: 0, g: 255, b: 0, a: 255 };
    assert_eq!(col_a, col_b);
    assert_ne!(col_a, col_c);

    let rec_a = Rectangle { x: 0.0, y: 0.0, width: 100.0, height: 50.0 };
    let rec_b = Rectangle { x: 0.0, y: 0.0, width: 100.0, height: 50.0 };
    let rec_c = Rectangle { x: 1.0, y: 0.0, width: 100.0, height: 50.0 };
    assert_eq!(rec_a, rec_b);
    assert_ne!(rec_a, rec_c);
}

#[test]
fn test_generated_structures_debug() {
    let v2 = Vector2 { x: 1.5, y: -2.5 };
    assert_eq!(format!("{:?}", v2), "Vector2 { x: 1.5, y: -2.5 }");

    let col = Color { r: 128, g: 64, b: 32, a: 255 };
    assert!(format!("{:?}", col).contains("128"));
}

#[test]
fn test_enums_reimplemented_values_and_traits() {
    // 1. ConfigFlags
    assert_eq!(FLAG_VSYNC_HINT as u32, 0x00000040);
    assert_eq!(FLAG_FULLSCREEN_MODE as u32, 0x00000002);
    assert_eq!(FLAG_WINDOW_RESIZABLE as u32, 0x00000004);
    assert_eq!(format!("{:?}", FLAG_VSYNC_HINT), "FLAG_VSYNC_HINT");
    assert_eq!(ConfigFlags::FLAG_VSYNC_HINT, FLAG_VSYNC_HINT);

    // 2. TraceLogLevel
    assert_eq!(TraceLogLevel::default(), LOG_ALL);
    assert_eq!(LOG_ALL as i32, 0);
    assert_eq!(LOG_TRACE as i32, 1);
    assert_eq!(LOG_DEBUG as i32, 2);
    assert_eq!(LOG_INFO as i32, 3);
    assert_eq!(LOG_WARNING as i32, 4);
    assert_eq!(LOG_ERROR as i32, 5);
    assert_eq!(LOG_FATAL as i32, 6);
    assert_eq!(LOG_NONE as i32, 7);

    // 3. KeyboardKey
    assert_eq!(KeyboardKey::default(), KEY_NULL);
    assert_eq!(KEY_NULL as i32, 0);
    assert_eq!(KEY_SPACE as i32, 32);
    assert_eq!(KEY_ESCAPE as i32, 256);
    assert_eq!(KEY_ENTER as i32, 257);
    assert_eq!(KEY_A as i32, 65);
    assert_eq!(KEY_KB_MENU as i32, 348);

    // 4. MouseButton
    assert_eq!(MouseButton::default(), MOUSE_BUTTON_LEFT);
    assert_eq!(MOUSE_BUTTON_LEFT as i32, 0);
    assert_eq!(MOUSE_BUTTON_RIGHT as i32, 1);
    assert_eq!(MOUSE_BUTTON_MIDDLE as i32, 2);
    assert_eq!(MOUSE_BUTTON_BACK as i32, 6);
    // Aliases
    assert_eq!(MOUSE_LEFT_BUTTON, MOUSE_BUTTON_LEFT);
    assert_eq!(MOUSE_RIGHT_BUTTON, MOUSE_BUTTON_RIGHT);
    assert_eq!(MOUSE_MIDDLE_BUTTON, MOUSE_BUTTON_MIDDLE);

    // 5. MouseCursor
    assert_eq!(MouseCursor::default(), MOUSE_CURSOR_DEFAULT);
    assert_eq!(MOUSE_CURSOR_DEFAULT as i32, 0);
    assert_eq!(MOUSE_CURSOR_ARROW as i32, 1);
    assert_eq!(MOUSE_CURSOR_NOT_ALLOWED as i32, 10);

    // 6. GamepadButton
    assert_eq!(GamepadButton::default(), GAMEPAD_BUTTON_UNKNOWN);
    assert_eq!(GAMEPAD_BUTTON_UNKNOWN as i32, 0);
    assert_eq!(GAMEPAD_BUTTON_LEFT_FACE_UP as i32, 1);
    assert_eq!(GAMEPAD_BUTTON_RIGHT_THUMB as i32, 17);

    // 7. GamepadAxis
    assert_eq!(GamepadAxis::default(), GAMEPAD_AXIS_LEFT_X);
    assert_eq!(GAMEPAD_AXIS_LEFT_X as i32, 0);
    assert_eq!(GAMEPAD_AXIS_RIGHT_TRIGGER as i32, 5);

    // 8. MaterialMapIndex
    assert_eq!(MaterialMapIndex::default(), MATERIAL_MAP_ALBEDO);
    assert_eq!(MATERIAL_MAP_ALBEDO as i32, 0);
    assert_eq!(MATERIAL_MAP_METALNESS as i32, 1);
    assert_eq!(MATERIAL_MAP_BRDF as i32, 10);
    assert_eq!(MATERIAL_MAP_DIFFUSE, MATERIAL_MAP_ALBEDO);
    assert_eq!(MATERIAL_MAP_SPECULAR, MATERIAL_MAP_METALNESS);

    // 9. ShaderLocationIndex
    assert_eq!(ShaderLocationIndex::default(), SHADER_LOC_VERTEX_POSITION);
    assert_eq!(SHADER_LOC_VERTEX_POSITION as i32, 0);
    assert_eq!(SHADER_LOC_MATRIX_MVP as i32, 6);
    assert_eq!(SHADER_LOC_VERTEX_INSTANCETRANSFORM as i32, 29);
    assert_eq!(SHADER_LOC_MAP_DIFFUSE, SHADER_LOC_MAP_ALBEDO);
    assert_eq!(SHADER_LOC_MAP_SPECULAR, SHADER_LOC_MAP_METALNESS);

    // 10. ShaderUniformDataType
    assert_eq!(ShaderUniformDataType::default(), SHADER_UNIFORM_FLOAT);
    assert_eq!(SHADER_UNIFORM_FLOAT as i32, 0);
    assert_eq!(SHADER_UNIFORM_SAMPLER2D as i32, 12);

    // 11. ShaderAttributeDataType
    assert_eq!(ShaderAttributeDataType::default(), SHADER_ATTRIB_FLOAT);
    assert_eq!(SHADER_ATTRIB_FLOAT as i32, 0);
    assert_eq!(SHADER_ATTRIB_VEC4 as i32, 3);

    // 12. PixelFormat
    assert_eq!(PixelFormat::default(), PIXELFORMAT_UNCOMPRESSED_GRAYSCALE);
    assert_eq!(PIXELFORMAT_UNCOMPRESSED_GRAYSCALE as i32, 1);
    assert_eq!(PIXELFORMAT_UNCOMPRESSED_R8G8B8A8 as i32, 7);
    assert_eq!(PIXELFORMAT_COMPRESSED_ASTC_8x8_RGBA as i32, 24);

    // 13. TextureFilter
    assert_eq!(TextureFilter::default(), TEXTURE_FILTER_POINT);
    assert_eq!(TEXTURE_FILTER_POINT as i32, 0);
    assert_eq!(TEXTURE_FILTER_BILINEAR as i32, 1);
    assert_eq!(TEXTURE_FILTER_ANISOTROPIC_16X as i32, 5);

    // 14. TextureWrap
    assert_eq!(TextureWrap::default(), TEXTURE_WRAP_REPEAT);
    assert_eq!(TEXTURE_WRAP_REPEAT as i32, 0);
    assert_eq!(TEXTURE_WRAP_MIRROR_CLAMP as i32, 3);

    // 15. CubemapLayout
    assert_eq!(CubemapLayout::default(), CUBEMAP_LAYOUT_AUTO_DETECT);
    assert_eq!(CUBEMAP_LAYOUT_AUTO_DETECT as i32, 0);
    assert_eq!(CUBEMAP_LAYOUT_CROSS_FOUR_BY_THREE as i32, 4);

    // 16. FontType
    assert_eq!(FontType::default(), FONT_DEFAULT);
    assert_eq!(FONT_DEFAULT as i32, 0);
    assert_eq!(FONT_BITMAP as i32, 1);
    assert_eq!(FONT_SDF as i32, 2);

    // 17. BlendMode
    assert_eq!(BlendMode::default(), BLEND_ALPHA);
    assert_eq!(BLEND_ALPHA as i32, 0);
    assert_eq!(BLEND_ADDITIVE as i32, 1);
    assert_eq!(BLEND_CUSTOM_SEPARATE as i32, 7);

    // 18. Gesture
    assert_eq!(Gesture::default(), GESTURE_NONE);
    assert_eq!(GESTURE_NONE as i32, 0);
    assert_eq!(GESTURE_TAP as i32, 1);
    assert_eq!(GESTURE_PINCH_OUT as i32, 512);

    // 19. CameraMode
    assert_eq!(CameraMode::default(), CAMERA_CUSTOM);
    assert_eq!(CAMERA_CUSTOM as i32, 0);
    assert_eq!(CAMERA_THIRD_PERSON as i32, 4);

    // 20. CameraProjection
    assert_eq!(CameraProjection::default(), CAMERA_PERSPECTIVE);
    assert_eq!(CAMERA_PERSPECTIVE as i32, 0);
    assert_eq!(CAMERA_ORTHOGRAPHIC as i32, 1);

    // 21. NPatchLayout
    assert_eq!(NPatchLayout::default(), NPATCH_NINE_PATCH);
    assert_eq!(NPATCH_NINE_PATCH as i32, 0);
    assert_eq!(NPATCH_THREE_PATCH_HORIZONTAL as i32, 2);
}

#[test]
fn test_vector2_math_and_scalar_traits() {
    let a = Vector2::new(3.0, 4.0);
    let b = Vector2::new(1.0, 2.0);

    assert_eq!(a + b, Vector2::new(4.0, 6.0));
    assert_eq!(a - b, Vector2::new(2.0, 2.0));
    assert_eq!(a * b, Vector2::new(3.0, 8.0));
    assert_eq!(a * 2.0, Vector2::new(6.0, 8.0));
    assert_eq!(2.0 * a, Vector2::new(6.0, 8.0));
    assert_eq!(a / 2.0, Vector2::new(1.5, 2.0));
    assert_eq!(-a, Vector2::new(-3.0, -4.0));

    assert_eq!(a.dot(b), 3.0 * 1.0 + 4.0 * 2.0);
    assert_eq!(a.length_sq(), 25.0);
    assert_eq!(a.length(), 5.0);
    assert_eq!(a.normalize(), Vector2::new(0.6, 0.8));
    assert_eq!(a.distance(Vector2::new(0.0, 0.0)), 5.0);
    assert_eq!(a.lerp(Vector2::new(5.0, 6.0), 0.5), Vector2::new(4.0, 5.0));
}

#[test]
fn test_vector3_math_and_operators() {
    let a = Vector3::new(1.0, 2.0, 3.0);
    let b = Vector3::new(4.0, 5.0, 6.0);

    // Addition, subtraction, element-wise multiplication
    assert_eq!(a + b, Vector3::new(5.0, 7.0, 9.0));
    assert_eq!(b - a, Vector3::new(3.0, 3.0, 3.0));
    assert_eq!(a * b, Vector3::new(4.0, 10.0, 18.0));
    assert_eq!(b / a, Vector3::new(4.0, 2.5, 2.0));

    // Scalar ops
    assert_eq!(a * 2.0, Vector3::new(2.0, 4.0, 6.0));
    assert_eq!(3.0 * a, Vector3::new(3.0, 6.0, 9.0));
    assert_eq!(a / 2.0, Vector3::new(0.5, 1.0, 1.5));
    assert_eq!(-a, Vector3::new(-1.0, -2.0, -3.0));

    // Dot and cross product
    assert_eq!(a.dot(b), 1.0 * 4.0 + 2.0 * 5.0 + 3.0 * 6.0); // 4 + 10 + 18 = 32
    let x_axis = Vector3::new(1.0, 0.0, 0.0);
    let y_axis = Vector3::new(0.0, 1.0, 0.0);
    let z_axis = x_axis.cross(y_axis);
    assert_eq!(z_axis, Vector3::new(0.0, 0.0, 1.0));

    // Length, normalize, distance, lerp
    let v = Vector3::new(0.0, 3.0, 4.0);
    assert_eq!(v.length_sq(), 25.0);
    assert_eq!(v.length(), 5.0);
    assert_eq!(v.normalize(), Vector3::new(0.0, 0.6, 0.8));
    assert_eq!(v.distance(Vector3::zero()), 5.0);
    assert_eq!(Vector3::zero().lerp(Vector3::new(10.0, 20.0, 30.0), 0.5), Vector3::new(5.0, 10.0, 15.0));
}

#[test]
fn test_matrix_helpers_and_multiplication() {
    let identity = Matrix::identity();
    assert_eq!(identity.m0, 1.0);
    assert_eq!(identity.m5, 1.0);
    assert_eq!(identity.m10, 1.0);
    assert_eq!(identity.m15, 1.0);
    assert_eq!(identity.m1, 0.0);

    // Identity multiplied by identity is identity
    let mul_id = identity * identity;
    assert_eq!(mul_id, identity);

    // Basis matrix
    let right = Vector3::new(1.0, 0.0, 0.0);
    let up = Vector3::new(0.0, 1.0, 0.0);
    let forward = Vector3::new(0.0, 0.0, 1.0);
    let pos = Vector3::new(10.0, 20.0, 30.0);
    let basis = Matrix::from_basis(right, up, forward, pos);
    assert_eq!(basis.m0, 1.0);
    assert_eq!(basis.m5, 1.0);
    assert_eq!(basis.m10, 1.0);
    assert_eq!(basis.m3, 10.0);
    assert_eq!(basis.m7, 20.0);
    assert_eq!(basis.m11, 30.0);
}

#[test]
fn test_color_lerp() {
    let black = Color { r: 0, g: 0, b: 0, a: 255 };
    let white = Color { r: 255, g: 255, b: 255, a: 255 };

    let gray = black.lerp(white, 0.5);
    assert_eq!(gray.r, 128);
    assert_eq!(gray.g, 128);
    assert_eq!(gray.b, 128);
    assert_eq!(gray.a, 255);

    // Clamp behavior
    assert_eq!(black.lerp(white, -1.0), black);
    assert_eq!(black.lerp(white, 2.0), white);
}
