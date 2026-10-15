#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

use std::ffi::CString;

pub mod enums;
pub mod generated;
pub mod rlgl;

pub use enums::*;
pub use generated::*;

#[macro_export]
macro_rules! cstr {
    ($s:expr) => {{
        use std::str::FromStr;
        std::ffi::CString::from_str($s).unwrap()
    }};
}

#[macro_export]
macro_rules! screen_size {
    () => {{
        Vector2 {
            x: get_screen_width() as f32
            y: get_screen_height() as f32
        },
    }};
}

pub fn init_window(width: i32, height: i32, title: CString) {
    unsafe {
        InitWindow(width, height, title.as_ptr());
    }
}

pub fn close_window() {
    unsafe {
        CloseWindow();
    }
}

pub fn window_should_close() -> bool {
    unsafe { WindowShouldClose() }
}

pub fn begin_drawing() {
    unsafe { BeginDrawing() }
}

pub fn end_drawing() {
    unsafe { EndDrawing() }
}

pub fn clear_background(color: Color) {
    unsafe { ClearBackground(color) }
}

pub fn draw_fps(pos_x: i32, pos_y: i32) {
    unsafe { DrawFPS(pos_x, pos_y) }
}

pub fn set_target_fps(fps: i32) {
    unsafe { SetTargetFPS(fps) }
}

pub fn is_mouse_button_down(button: MouseButton) -> bool {
    unsafe { IsMouseButtonDown(button as i32) }
}

pub fn get_mouse_delta() -> Vector2 {
    unsafe { GetMouseDelta() }
}

pub fn get_mouse_wheel_move() -> f32 {
    unsafe { GetMouseWheelMove() }
}

pub fn get_mouse_position() -> Vector2 {
    unsafe { GetMousePosition() }
}

pub fn get_screen_to_world_2d(position: Vector2, camera: Camera2D) -> Vector2 {
    unsafe { GetScreenToWorld2D(position, camera) }
}

pub fn begin_mode_2d(camera: Camera2D) {
    unsafe { BeginMode2D(camera) }
}

pub fn end_mode_2d() {
    unsafe { EndMode2D() }
}

pub fn draw_grid(slices: i32, spacing: f32) {
    unsafe { DrawGrid(slices, spacing) }
}

pub fn draw_circle_v(position: Vector2, radius: f32, color: Color) {
    unsafe { DrawCircleV(position, radius, color) }
}

pub fn draw_line(start: Vector2, end: Vector2, color: Color) {
    unsafe { DrawLineV(start, end, color) }
}

pub fn get_screen_width() -> i32 {
    unsafe { GetScreenWidth() }
}

pub fn get_screen_height() -> i32 {
    unsafe { GetScreenHeight() }
}

pub fn get_frame_time() -> f32 {
    unsafe { GetFrameTime() }
}

pub fn is_key_down(key: KeyboardKey) -> bool {
    unsafe { IsKeyDown(key as i32) }
}

pub fn is_key_pressed(key: KeyboardKey) -> bool {
    unsafe { IsKeyPressed(key as i32) }
}

pub fn is_mouse_button_pressed(button: MouseButton) -> bool {
    unsafe { IsMouseButtonPressed(button as i32) }
}

pub fn draw_rectangle(pos_x: i32, pos_y: i32, width: i32, height: i32, color: Color) {
    unsafe { DrawRectangle(pos_x, pos_y, width, height, color) }
}

pub fn draw_rectangle_lines(pos_x: i32, pos_y: i32, width: i32, height: i32, color: Color) {
    unsafe { DrawRectangleLines(pos_x, pos_y, width, height, color) }
}

pub fn draw_rectangle_lines_ex(rec: Rectangle, line_thick: f32, color: Color) {
    unsafe { DrawRectangleLinesEx(rec, line_thick, color) }
}

pub fn draw_text(text: impl AsRef<str>, pos_x: i32, pos_y: i32, font_size: i32, color: Color) {
    if let Ok(c_text) = CString::new(text.as_ref()) {
        unsafe { DrawText(c_text.as_ptr(), pos_x, pos_y, font_size, color) }
    }
}

pub fn measure_text(text: impl AsRef<str>, font_size: i32) -> i32 {
    if let Ok(c_text) = CString::new(text.as_ref()) {
        unsafe { MeasureText(c_text.as_ptr(), font_size) }
    } else {
        0
    }
}

pub fn measure_text_ex(font: Font, text: CString, fontSize: f32, spacing: f32) -> Vector2 {
    unsafe { MeasureTextEx(font, text.as_ptr(), fontSize, spacing) }
}

pub fn measure_text_ex_str(font: Font, text: impl AsRef<str>, font_size: f32, spacing: f32) -> Vector2 {
    if let Ok(c_text) = CString::new(text.as_ref()) {
        unsafe { MeasureTextEx(font, c_text.as_ptr(), font_size, spacing) }
    } else {
        Vector2 { x: 0.0, y: 0.0 }
    }
}

pub fn load_texture(path: CString) -> Texture {
    unsafe { LoadTexture(path.as_ptr()) }
}
pub fn get_color(hex: u32) -> Color {
    unsafe { GetColor(hex) }
}
pub fn load_music_stream(path: CString) -> Music {
    unsafe { LoadMusicStream(path.as_ptr()) }
}
pub fn init_audio_device() {
    unsafe {
        InitAudioDevice();
    }
}
pub fn close_audio_device() {
    unsafe {
        CloseAudioDevice();
    }
}
pub fn get_font_default() -> Font {
    unsafe { GetFontDefault() }
}
pub fn draw_text_ex(
    font: Font,
    text: CString,
    position: Vector2,
    font_size: f32,
    spacing: f32,
    tint: Color,
) {
    unsafe {
        DrawTextEx(font, text.as_ptr(), position, font_size, spacing, tint);
    }
}

pub fn draw_text_ex_str(
    font: Font,
    text: impl AsRef<str>,
    position: Vector2,
    font_size: f32,
    spacing: f32,
    tint: Color,
) {
    if let Ok(c_text) = CString::new(text.as_ref()) {
        unsafe {
            DrawTextEx(font, c_text.as_ptr(), position, font_size, spacing, tint);
        }
    }
}
pub fn is_cursor_on_screen() -> bool {
    unsafe { IsCursorOnScreen() }
}
pub fn disable_cursor() {
    unsafe { DisableCursor() }
}
pub fn enable_cursor() {
    unsafe { EnableCursor() }
}
pub fn maximize_window() {
    unsafe { MaximizeWindow() }
}
pub fn check_collision_circle_rec(center: Vector2, radius: f32, rec: Rectangle) -> bool {
    unsafe { CheckCollisionCircleRec(center, radius, rec) }
}

pub fn is_texture_valid(texture: Texture2D) -> bool {
    unsafe { IsTextureValid(texture) }
}

pub fn unload_texture(texture: Texture2D) {
    unsafe {
        UnloadTexture(texture);
    }
}
pub fn get_fps() -> i32 {
    unsafe { GetFPS() }
}

pub fn draw_texture_ex(
    texture: Texture2D,
    position: Vector2,
    rotation: f32,
    scale: f32,
    tint: Color,
) {
    unsafe {
        DrawTextureEx(texture, position, rotation, scale, tint);
    }
}

// Some Basic Colors
// NOTE: Custom raylib color palette for amazing visuals on WHITE background
pub const LIGHTGRAY: Color = Color {
    r: 200,
    g: 200,
    b: 200,
    a: 255,
}; // Light Gray
pub const GRAY: Color = Color {
    r: 130,
    g: 130,
    b: 130,
    a: 255,
}; // Gray
pub const DARKGRAY: Color = Color {
    r: 80,
    g: 80,
    b: 80,
    a: 255,
}; // Dark Gray
pub const YELLOW: Color = Color {
    r: 253,
    g: 249,
    b: 0,
    a: 255,
}; // Yellow
pub const GOLD: Color = Color {
    r: 255,
    g: 203,
    b: 0,
    a: 255,
}; // Gold
pub const ORANGE: Color = Color {
    r: 255,
    g: 161,
    b: 0,
    a: 255,
}; // Orange
pub const PINK: Color = Color {
    r: 255,
    g: 109,
    b: 194,
    a: 255,
}; // Pink
pub const RED: Color = Color {
    r: 230,
    g: 41,
    b: 55,
    a: 255,
}; // Red
pub const MAROON: Color = Color {
    r: 190,
    g: 33,
    b: 55,
    a: 255,
}; // Maroon
pub const GREEN: Color = Color {
    r: 0,
    g: 228,
    b: 48,
    a: 255,
}; // Green
pub const LIME: Color = Color {
    r: 0,
    g: 158,
    b: 47,
    a: 255,
}; // Lime
pub const DARKGREEN: Color = Color {
    r: 0,
    g: 117,
    b: 44,
    a: 255,
}; // Dark Green
pub const SKYBLUE: Color = Color {
    r: 102,
    g: 191,
    b: 255,
    a: 255,
}; // Sky Blue
pub const BLUE: Color = Color {
    r: 0,
    g: 121,
    b: 241,
    a: 255,
}; // Blue
pub const DARKBLUE: Color = Color {
    r: 0,
    g: 82,
    b: 172,
    a: 255,
}; // Dark Blue
pub const PURPLE: Color = Color {
    r: 200,
    g: 122,
    b: 255,
    a: 255,
}; // Purple
pub const VIOLET: Color = Color {
    r: 135,
    g: 60,
    b: 190,
    a: 255,
}; // Violet
pub const DARKPURPLE: Color = Color {
    r: 112,
    g: 31,
    b: 126,
    a: 255,
}; // Dark Purple
pub const BEIGE: Color = Color {
    r: 211,
    g: 176,
    b: 131,
    a: 255,
}; // Beige
pub const BROWN: Color = Color {
    r: 127,
    g: 106,
    b: 79,
    a: 255,
}; // Brown
pub const DARKBROWN: Color = Color {
    r: 76,
    g: 63,
    b: 47,
    a: 255,
}; // Dark Brown
pub const WHITE: Color = Color {
    r: 255,
    g: 255,
    b: 255,
    a: 255,
}; // White
pub const BLACK: Color = Color {
    r: 0,
    g: 0,
    b: 0,
    a: 255,
}; // Black
pub const BLANK: Color = Color {
    r: 0,
    g: 0,
    b: 0,
    a: 0,
}; // Blank (Transparent)
pub const MAGENTA: Color = Color {
    r: 255,
    g: 0,
    b: 255,
    a: 255,
}; // Magenta
/// (raylib WHITE)
pub const RAYWHITE: Color = Color {
    r: 245,
    g: 245,
    b: 245,
    a: 255,
};

impl std::ops::Add for Vector2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Vector2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Sub for Vector2 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Vector2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl std::ops::Mul for Vector2 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Vector2 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl std::ops::Div for Vector2 {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Vector2 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl std::ops::Mul<f32> for Vector2 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        Vector2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl std::ops::Mul<Vector2> for f32 {
    type Output = Vector2;
    fn mul(self, rhs: Vector2) -> Self::Output {
        Vector2 {
            x: self * rhs.x,
            y: self * rhs.y,
        }
    }
}

impl std::ops::Div<f32> for Vector2 {
    type Output = Self;
    fn div(self, rhs: f32) -> Self::Output {
        Vector2 {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl std::ops::Neg for Vector2 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Vector2 {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl Eq for Vector2 {}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    pub fn zero() -> Self {
        Self::default()
    }
    pub fn one() -> Self {
        Self { x: 1.0, y: 1.0 }
    }
    pub fn scale(&self, s: f32) -> Vector2 {
        Vector2 {
            x: self.x * s,
            y: self.y * s,
        }
    }
    pub fn is_zero(&self) -> bool {
        self.x == 0.0 && self.y == 0.0
    }
    pub fn dot(&self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y
    }
    pub fn length_sq(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }
    pub fn length(&self) -> f32 {
        self.x.hypot(self.y)
    }
    pub fn distance(&self, other: Self) -> f32 {
        (*self - other).length()
    }
    pub fn lerp(&self, other: Self, amount: f32) -> Self {
        *self + (other - *self) * amount
    }
    pub fn normalize(&self) -> Self {
        let length = self.length();
        if length > 0.0 {
            let ilength = 1.0 / length;
            Vector2 {
                x: self.x * ilength,
                y: self.y * ilength,
            }
        } else {
            Vector2::zero()
        }
    }
}

impl std::ops::Add for Vector3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Vector3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl std::ops::Sub for Vector3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Vector3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl std::ops::Mul<Vector3> for Vector3 {
    type Output = Self;
    fn mul(self, rhs: Vector3) -> Self::Output {
        Vector3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl std::ops::Mul<f32> for Vector3 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        Vector3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl std::ops::Mul<Vector3> for f32 {
    type Output = Vector3;
    fn mul(self, rhs: Vector3) -> Self::Output {
        Vector3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl std::ops::Div<Vector3> for Vector3 {
    type Output = Self;
    fn div(self, rhs: Vector3) -> Self::Output {
        Vector3 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl std::ops::Div<f32> for Vector3 {
    type Output = Self;
    fn div(self, rhs: f32) -> Self::Output {
        Vector3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl std::ops::Neg for Vector3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Vector3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Eq for Vector3 {}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0, z: 0.0 }
    }

    pub fn one() -> Self {
        Self { x: 1.0, y: 1.0, z: 1.0 }
    }

    pub fn dot(&self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: Self) -> Self {
        Vector3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn length_sq(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f32 {
        self.length_sq().sqrt()
    }

    pub fn normalize(&self) -> Self {
        let length = self.length();
        if length > 0.0 {
            let inv = 1.0 / length;
            Vector3 {
                x: self.x * inv,
                y: self.y * inv,
                z: self.z * inv,
            }
        } else {
            Self::zero()
        }
    }

    pub fn distance(&self, other: Self) -> f32 {
        (*self - other).length()
    }

    pub fn lerp(&self, other: Self, amount: f32) -> Self {
        *self + (other - *self) * amount
    }

    pub fn scale(&self, s: f32) -> Self {
        *self * s
    }
}

impl Matrix {
    pub fn identity() -> Self {
        Self {
            m0: 1.0, m1: 0.0, m2: 0.0, m3: 0.0,
            m4: 0.0, m5: 1.0, m6: 0.0, m7: 0.0,
            m8: 0.0, m9: 0.0, m10: 1.0, m11: 0.0,
            m12: 0.0, m13: 0.0, m14: 0.0, m15: 1.0,
        }
    }

    pub fn from_basis(right: Vector3, up: Vector3, forward: Vector3, position: Vector3) -> Self {
        Self {
            m0: right.x, m1: up.x, m2: forward.x, m3: position.x,
            m4: right.y, m5: up.y, m6: forward.y, m7: position.y,
            m8: right.z, m9: up.z, m10: forward.z, m11: position.z,
            m12: 0.0,    m13: 0.0,  m14: 0.0,       m15: 1.0,
        }
    }
}

impl std::ops::Mul for Matrix {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            m0: self.m0*rhs.m0 + self.m1*rhs.m4 + self.m2*rhs.m8 + self.m3*rhs.m12,
            m1: self.m0*rhs.m1 + self.m1*rhs.m5 + self.m2*rhs.m9 + self.m3*rhs.m13,
            m2: self.m0*rhs.m2 + self.m1*rhs.m6 + self.m2*rhs.m10 + self.m3*rhs.m14,
            m3: self.m0*rhs.m3 + self.m1*rhs.m7 + self.m2*rhs.m11 + self.m3*rhs.m15,

            m4: self.m4*rhs.m0 + self.m5*rhs.m4 + self.m6*rhs.m8 + self.m7*rhs.m12,
            m5: self.m4*rhs.m1 + self.m5*rhs.m5 + self.m6*rhs.m9 + self.m7*rhs.m13,
            m6: self.m4*rhs.m2 + self.m5*rhs.m6 + self.m6*rhs.m10 + self.m7*rhs.m14,
            m7: self.m4*rhs.m3 + self.m5*rhs.m7 + self.m6*rhs.m11 + self.m7*rhs.m15,

            m8: self.m8*rhs.m0 + self.m9*rhs.m4 + self.m10*rhs.m8 + self.m11*rhs.m12,
            m9: self.m8*rhs.m1 + self.m9*rhs.m5 + self.m10*rhs.m9 + self.m11*rhs.m13,
            m10: self.m8*rhs.m2 + self.m9*rhs.m6 + self.m10*rhs.m10 + self.m11*rhs.m14,
            m11: self.m8*rhs.m3 + self.m9*rhs.m7 + self.m10*rhs.m11 + self.m11*rhs.m15,

            m12: self.m12*rhs.m0 + self.m13*rhs.m4 + self.m14*rhs.m8 + self.m15*rhs.m12,
            m13: self.m12*rhs.m1 + self.m13*rhs.m5 + self.m14*rhs.m9 + self.m15*rhs.m13,
            m14: self.m12*rhs.m2 + self.m13*rhs.m6 + self.m14*rhs.m10 + self.m15*rhs.m14,
            m15: self.m12*rhs.m3 + self.m13*rhs.m7 + self.m14*rhs.m11 + self.m15*rhs.m15,
        }
    }
}

impl Rectangle {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}

pub fn set_config_flags(flags: &[ConfigFlags]) {
    let flags = flags.iter().map(|f| *f as u32).fold(0, |acc, f| acc | f);
    unsafe {
        SetConfigFlags(flags);
    }
}

impl Color {
    /// Linearly interpolates between `self` and `target` by `factor` in `[0.0, 1.0]`.
    pub fn lerp(&self, target: Color, factor: f32) -> Color {
        let f = factor.clamp(0.0, 1.0);
        Color {
            r: (self.r as f32 + (target.r as f32 - self.r as f32) * f).round() as u8,
            g: (self.g as f32 + (target.g as f32 - self.g as f32) * f).round() as u8,
            b: (self.b as f32 + (target.b as f32 - self.b as f32) * f).round() as u8,
            a: (self.a as f32 + (target.a as f32 - self.a as f32) * f).round() as u8,
        }
    }
}

/// Draws a texture in 3D space facing the camera.
pub fn draw_billboard(
    camera: Camera3D,
    texture: Texture2D,
    position: Vector3,
    size: f32,
    tint: Color,
) {
    unsafe {
        DrawBillboard(camera, texture, position, size, tint);
    }
}

/// Converts a 3D world position to 2D screen coordinates with custom viewport width and height.
pub fn get_world_to_screen_ex(
    position: Vector3,
    camera: Camera3D,
    width: i32,
    height: i32,
) -> Vector2 {
    unsafe {
        GetWorldToScreenEx(position, camera, width, height)
    }
}

