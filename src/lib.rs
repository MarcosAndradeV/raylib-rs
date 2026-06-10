use std::ffi::CString;
use std::os::raw::{c_char, c_int};

pub mod rlgl;

#[macro_export]
/// NOTE: only &'static str
macro_rules! cstr {
    ($s:literal) => {{
        use std::str::FromStr;
        std::ffi::CString::from_str($s).unwrap()
    }};
}

#[macro_export]
/// NOTE: only &'static str
macro_rules! screen_size {
    () => {{
        Vector2 {
            x: get_screen_width() as f32
            y: get_screen_height() as f32
        },
    }};
}

unsafe extern "C" {
    #[link_name = "InitWindow"]
    fn InitWindow(width: c_int, height: c_int, title: *const c_char);
}
pub fn init_window(width: i32, height: i32, title: CString) {
    unsafe {
        InitWindow(width, height, title.as_ptr());
    }
}

unsafe extern "C" {
    #[link_name = "CloseWindow"]
    fn CloseWindow();
}
pub fn close_window() {
    unsafe {
        CloseWindow();
    }
}

unsafe extern "C" {
    #[link_name = "WindowShouldClose"]
    fn WindowShouldClose() -> bool;
}
pub fn window_should_close() -> bool {
    unsafe { WindowShouldClose() }
}

unsafe extern "C" {
    #[link_name = "BeginDrawing"]
    fn BeginDrawing();
}
pub fn begin_drawing() {
    unsafe { BeginDrawing() }
}

unsafe extern "C" {
    #[link_name = "EndDrawing"]
    fn EndDrawing();
}
pub fn end_drawing() {
    unsafe { EndDrawing() }
}

unsafe extern "C" {
    #[link_name = "ClearBackground"]
    fn ClearBackground(color: Color);
}
pub fn clear_background(color: Color) {
    unsafe { ClearBackground(color) }
}

unsafe extern "C" {
    #[link_name = "DrawFPS"]
    fn DrawFPS(posX: c_int, posY: c_int);
}
pub fn draw_fps(pos_x: i32, pos_y: i32) {
    unsafe { DrawFPS(pos_x, pos_y) }
}

unsafe extern "C" {
    #[link_name = "SetTargetFPS"]
    fn SetTargetFPS(fps: c_int);
}
pub fn set_target_fps(fps: i32) {
    unsafe { SetTargetFPS(fps) }
}

unsafe extern "C" {
    #[link_name = "IsMouseButtonDown"]
    fn IsMouseButtonDown(button: c_int) -> bool;
}
pub fn is_mouse_button_down(button: MouseButton) -> bool {
    unsafe { IsMouseButtonDown(button as i32) }
}

unsafe extern "C" {
    #[link_name = "GetMouseDelta"]
    fn GetMouseDelta() -> Vector2;
}
pub fn get_mouse_delta() -> Vector2 {
    unsafe { GetMouseDelta() }
}

unsafe extern "C" {
    #[link_name = "GetMouseWheelMove"]
    fn GetMouseWheelMove() -> f32;
}
pub fn get_mouse_wheel_move() -> f32 {
    unsafe { GetMouseWheelMove() }
}

unsafe extern "C" {
    #[link_name = "GetMousePosition"]
    fn GetMousePosition() -> Vector2;
}
pub fn get_mouse_position() -> Vector2 {
    unsafe { GetMousePosition() }
}

unsafe extern "C" {
    #[link_name = "GetScreenToWorld2D"]
    fn GetScreenToWorld2D(position: Vector2, camera: Camera2D) -> Vector2;
}
pub fn get_screen_to_world_2d(position: Vector2, camera: Camera2D) -> Vector2 {
    unsafe { GetScreenToWorld2D(position, camera) }
}

unsafe extern "C" {
    #[link_name = "BeginMode2D"]
    fn BeginMode2D(camera: Camera2D);
}
pub fn begin_mode_2d(camera: Camera2D) {
    unsafe { BeginMode2D(camera) }
}

unsafe extern "C" {
    #[link_name = "EndMode2D"]
    fn EndMode2D();
}
pub fn end_mode_2d() {
    unsafe { EndMode2D() }
}

unsafe extern "C" {
    #[link_name = "DrawGrid"]
    fn DrawGrid(slices: c_int, spacing: f32);
}
pub fn draw_grid(slices: i32, spacing: f32) {
    unsafe { DrawGrid(slices, spacing) }
}

unsafe extern "C" {
    #[link_name = "DrawCircleV"]
    fn DrawCircle(position: Vector2, radius: f32, color: Color);
}
pub fn draw_circle(position: Vector2, radius: f32, color: Color) {
    unsafe { DrawCircle(position, radius, color) }
}

unsafe extern "C" {
    #[link_name = "DrawLineV"]
    fn DrawLineV(startPos: Vector2, endPos: Vector2, color: Color);
}
pub fn draw_line(start: Vector2, end: Vector2, color: Color) {
    unsafe { DrawLineV(start, end, color) }
}

unsafe extern "C" {
    #[link_name = "GetScreenWidth"]
    fn GetScreenWidth() -> c_int;
}
pub fn get_screen_width() -> i32 {
    unsafe { GetScreenWidth() }
}

unsafe extern "C" {
    #[link_name = "GetScreenHeight"]
    fn GetScreenHeight() -> c_int;
}
pub fn get_screen_height() -> i32 {
    unsafe { GetScreenHeight() }
}

unsafe extern "C" {
    #[link_name = "GetFrameTime"]
    fn GetFrameTime() -> f32;
}
pub fn get_frame_time() -> f32 {
    unsafe { GetFrameTime() }
}


#[derive(Default, Clone, Copy)]
#[repr(C)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {}

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
pub const RAYWHITE: Color = Color {
    r: 245,
    g: 245,
    b: 245,
    a: 255,
}; // My own White (raylib logo)

#[derive(Debug, Default, Clone, Copy)]
#[repr(C)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl std::ops::Add for Vector2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Vector2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    pub fn zero() -> Self {
        Self::default()
    }
    pub fn scale(&self, s: f32) -> Vector2 {
        Vector2 {
            x: self.x * s,
            y: self.y * s,
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
#[repr(C)]
/// Camera2D, defines position/orientation in 2d space
pub struct Camera2D {
    /// Camera offset (screen space offset from window origin)
    pub offset: Vector2,
    /// Camera target (world space target point that is mapped to screen space offset)
    pub target: Vector2,
    /// Camera rotation in degrees (pivots around target)
    pub rotation: f32,
    /// Camera zoom (scaling around target), must not be set to 0, set to 1.0f for no scale
    pub zoom: f32,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Default, Clone, Copy)]
#[repr(C)]
/// Mouse buttons
pub enum MouseButton {
    /// Mouse button left
    #[default]
    MOUSE_BUTTON_LEFT = 0,
    /// Mouse button right
    MOUSE_BUTTON_RIGHT = 1,
    /// Mouse button middle (pressed wheel)
    MOUSE_BUTTON_MIDDLE = 2,
    /// Mouse button side (advanced mouse device)
    MOUSE_BUTTON_SIDE = 3,
    /// Mouse button extra (advanced mouse device)
    MOUSE_BUTTON_EXTRA = 4,
    /// Mouse button forward (advanced mouse device)
    MOUSE_BUTTON_FORWARD = 5,
    /// Mouse button back (advanced mouse device)
    MOUSE_BUTTON_BACK = 6,
}
pub use MouseButton::*;

#[derive(Debug, Default, Clone, Copy)]
#[repr(C)]
pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rectangle {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self { x, y, width, height }
    }
}

// Keyboard Keys Constants
pub const KEY_W: i32 = 87;
pub const KEY_A: i32 = 65;
pub const KEY_S: i32 = 83;
pub const KEY_D: i32 = 68;
pub const KEY_Q: i32 = 81;
pub const KEY_E: i32 = 69;
pub const KEY_R: i32 = 82;
pub const KEY_ONE: i32 = 49;
pub const KEY_TWO: i32 = 50;
pub const KEY_THREE: i32 = 51;
pub const KEY_FOUR: i32 = 52;
pub const KEY_FIVE: i32 = 53;

unsafe extern "C" {
    #[link_name = "IsKeyDown"]
    fn IsKeyDown(key: c_int) -> bool;
    
    #[link_name = "IsKeyPressed"]
    fn IsKeyPressed(key: c_int) -> bool;

    #[link_name = "IsMouseButtonPressed"]
    fn IsMouseButtonPressed(button: c_int) -> bool;

    #[link_name = "DrawRectangle"]
    fn DrawRectangle(posX: c_int, posY: c_int, width: c_int, height: c_int, color: Color);

    #[link_name = "DrawRectangleLines"]
    fn DrawRectangleLines(posX: c_int, posY: c_int, width: c_int, height: c_int, color: Color);

    #[link_name = "DrawRectangleLinesEx"]
    fn DrawRectangleLinesEx(rec: Rectangle, lineThick: f32, color: Color);

    #[link_name = "DrawText"]
    fn DrawText(text: *const c_char, posX: c_int, posY: c_int, fontSize: c_int, color: Color);

    #[link_name = "MeasureText"]
    fn MeasureText(text: *const c_char, fontSize: c_int) -> c_int;
}

pub fn is_key_down(key: i32) -> bool {
    unsafe { IsKeyDown(key) }
}

pub fn is_key_pressed(key: i32) -> bool {
    unsafe { IsKeyPressed(key) }
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

pub fn draw_text(text: CString, pos_x: i32, pos_y: i32, font_size: i32, color: Color) {
    unsafe { DrawText(text.as_ptr(), pos_x, pos_y, font_size, color) }
}

pub fn measure_text(text: CString, font_size: i32) -> i32 {
    unsafe { MeasureText(text.as_ptr(), font_size) }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_add() {
        let v1 = Vector2 { x: 5.0, y: 10.0 };
        let v2 = Vector2 { x: 2.0, y: 3.0 };
        let result = v1 + v2;

        assert_eq!(result.x, 7.0);
        assert_eq!(result.y, 13.0);
    }
}
