use std::ffi::{CStr, CString};

use crate::enums::*;
use crate::generated::*;

/// Trait for ergonomically passing either `&str`, `String`, `CString`, or `&CStr`
/// into Raylib wrapper functions while maintaining 100% backward compatibility with `cstr!(...)`.
pub trait IntoCStr {
    fn into_c_string(self) -> CString;
}

impl IntoCStr for CString {
    #[inline]
    fn into_c_string(self) -> CString {
        self
    }
}

impl IntoCStr for &CString {
    #[inline]
    fn into_c_string(self) -> CString {
        self.clone()
    }
}

impl IntoCStr for &CStr {
    #[inline]
    fn into_c_string(self) -> CString {
        self.to_owned()
    }
}

impl IntoCStr for &str {
    #[inline]
    fn into_c_string(self) -> CString {
        CString::new(self).unwrap_or_default()
    }
}

impl IntoCStr for String {
    #[inline]
    fn into_c_string(self) -> CString {
        CString::new(self).unwrap_or_default()
    }
}

impl IntoCStr for &String {
    #[inline]
    fn into_c_string(self) -> CString {
        CString::new(self.as_str()).unwrap_or_default()
    }
}

//------------------------------------------------------------------------------------
// Window and Device Functions
//------------------------------------------------------------------------------------

pub fn init_window(width: i32, height: i32, title: impl IntoCStr) {
    let c_title = title.into_c_string();
    unsafe {
        InitWindow(width, height, c_title.as_ptr());
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

pub fn is_window_ready() -> bool {
    unsafe { IsWindowReady() }
}

pub fn is_window_fullscreen() -> bool {
    unsafe { IsWindowFullscreen() }
}

pub fn is_window_hidden() -> bool {
    unsafe { IsWindowHidden() }
}

pub fn is_window_minimized() -> bool {
    unsafe { IsWindowMinimized() }
}

pub fn is_window_maximized() -> bool {
    unsafe { IsWindowMaximized() }
}

pub fn is_window_focused() -> bool {
    unsafe { IsWindowFocused() }
}

pub fn is_window_resized() -> bool {
    unsafe { IsWindowResized() }
}

pub fn set_window_title(title: impl IntoCStr) {
    let c_title = title.into_c_string();
    unsafe {
        SetWindowTitle(c_title.as_ptr());
    }
}

pub fn set_window_position(x: i32, y: i32) {
    unsafe {
        SetWindowPosition(x, y);
    }
}

pub fn set_window_min_size(width: i32, height: i32) {
    unsafe {
        SetWindowMinSize(width, height);
    }
}

pub fn set_window_size(width: i32, height: i32) {
    unsafe {
        SetWindowSize(width, height);
    }
}

pub fn maximize_window() {
    unsafe { MaximizeWindow() }
}

pub fn minimize_window() {
    unsafe { MinimizeWindow() }
}

pub fn restore_window() {
    unsafe { RestoreWindow() }
}

pub fn toggle_fullscreen() {
    unsafe { ToggleFullscreen() }
}

pub fn toggle_borderless_windowed() {
    unsafe { ToggleBorderlessWindowed() }
}

pub fn get_screen_width() -> i32 {
    unsafe { GetScreenWidth() }
}

pub fn get_screen_height() -> i32 {
    unsafe { GetScreenHeight() }
}

pub fn get_render_width() -> i32 {
    unsafe { GetRenderWidth() }
}

pub fn get_render_height() -> i32 {
    unsafe { GetRenderHeight() }
}

pub fn set_config_flags(flags: &[ConfigFlags]) {
    let flags = flags.iter().map(|f| *f as u32).fold(0, |acc, f| acc | f);
    unsafe {
        SetConfigFlags(flags);
    }
}

pub fn set_trace_log_level(log_level: TraceLogLevel) {
    unsafe {
        SetTraceLogLevel(log_level as i32);
    }
}

//------------------------------------------------------------------------------------
// Cursor Functions
//------------------------------------------------------------------------------------

pub fn show_cursor() {
    unsafe { ShowCursor() }
}

pub fn hide_cursor() {
    unsafe { HideCursor() }
}

pub fn is_cursor_hidden() -> bool {
    unsafe { IsCursorHidden() }
}

pub fn enable_cursor() {
    unsafe { EnableCursor() }
}

pub fn disable_cursor() {
    unsafe { DisableCursor() }
}

pub fn is_cursor_on_screen() -> bool {
    unsafe { IsCursorOnScreen() }
}

pub fn set_mouse_cursor(cursor: MouseCursor) {
    unsafe { SetMouseCursor(cursor as i32) }
}

//------------------------------------------------------------------------------------
// Drawing & Mode Functions
//------------------------------------------------------------------------------------

pub fn clear_background(color: Color) {
    unsafe { ClearBackground(color) }
}

pub fn begin_drawing() {
    unsafe { BeginDrawing() }
}

pub fn end_drawing() {
    unsafe { EndDrawing() }
}

pub fn begin_mode_2d(camera: Camera2D) {
    unsafe { BeginMode2D(camera) }
}

pub fn end_mode_2d() {
    unsafe { EndMode2D() }
}

pub fn begin_mode_3d(camera: Camera3D) {
    unsafe { BeginMode3D(camera) }
}

pub fn end_mode_3d() {
    unsafe { EndMode3D() }
}

pub fn begin_blend_mode(mode: BlendMode) {
    unsafe { BeginBlendMode(mode as i32) }
}

pub fn end_blend_mode() {
    unsafe { EndBlendMode() }
}

pub fn begin_scissor_mode(x: i32, y: i32, width: i32, height: i32) {
    unsafe { BeginScissorMode(x, y, width, height) }
}

pub fn end_scissor_mode() {
    unsafe { EndScissorMode() }
}

//------------------------------------------------------------------------------------
// Timing Functions
//------------------------------------------------------------------------------------

pub fn set_target_fps(fps: i32) {
    unsafe { SetTargetFPS(fps) }
}

pub fn get_frame_time() -> f32 {
    unsafe { GetFrameTime() }
}

pub fn get_time() -> f64 {
    unsafe { GetTime() }
}

pub fn get_fps() -> i32 {
    unsafe { GetFPS() }
}

//------------------------------------------------------------------------------------
// Input: Keyboard & Mouse Functions
//------------------------------------------------------------------------------------

pub fn is_key_pressed(key: KeyboardKey) -> bool {
    unsafe { IsKeyPressed(key as i32) }
}

pub fn is_key_pressed_repeat(key: KeyboardKey) -> bool {
    unsafe { IsKeyPressedRepeat(key as i32) }
}

pub fn is_key_down(key: KeyboardKey) -> bool {
    unsafe { IsKeyDown(key as i32) }
}

pub fn is_key_released(key: KeyboardKey) -> bool {
    unsafe { IsKeyReleased(key as i32) }
}

pub fn is_key_up(key: KeyboardKey) -> bool {
    unsafe { IsKeyUp(key as i32) }
}

pub fn get_key_pressed() -> i32 {
    unsafe { GetKeyPressed() }
}

pub fn get_char_pressed() -> i32 {
    unsafe { GetCharPressed() }
}

pub fn set_exit_key(key: KeyboardKey) {
    unsafe { SetExitKey(key as i32) }
}

pub fn is_mouse_button_pressed(button: MouseButton) -> bool {
    unsafe { IsMouseButtonPressed(button as i32) }
}

pub fn is_mouse_button_down(button: MouseButton) -> bool {
    unsafe { IsMouseButtonDown(button as i32) }
}

pub fn is_mouse_button_released(button: MouseButton) -> bool {
    unsafe { IsMouseButtonReleased(button as i32) }
}

pub fn is_mouse_button_up(button: MouseButton) -> bool {
    unsafe { IsMouseButtonUp(button as i32) }
}

pub fn get_mouse_x() -> i32 {
    unsafe { GetMouseX() }
}

pub fn get_mouse_y() -> i32 {
    unsafe { GetMouseY() }
}

pub fn get_mouse_position() -> Vector2 {
    unsafe { GetMousePosition() }
}

pub fn get_mouse_delta() -> Vector2 {
    unsafe { GetMouseDelta() }
}

pub fn set_mouse_position(x: i32, y: i32) {
    unsafe { SetMousePosition(x, y) }
}

pub fn get_mouse_wheel_move() -> f32 {
    unsafe { GetMouseWheelMove() }
}

pub fn get_mouse_wheel_move_v() -> Vector2 {
    unsafe { GetMouseWheelMoveV() }
}

//------------------------------------------------------------------------------------
// Screen-Space Conversion & Misc Functions
//------------------------------------------------------------------------------------

pub fn get_screen_to_world_2d(position: Vector2, camera: Camera2D) -> Vector2 {
    unsafe { GetScreenToWorld2D(position, camera) }
}

pub fn get_world_to_screen_2d(position: Vector2, camera: Camera2D) -> Vector2 {
    unsafe { GetWorldToScreen2D(position, camera) }
}

pub fn take_screenshot(file_name: impl IntoCStr) {
    let c_file = file_name.into_c_string();
    unsafe { TakeScreenshot(c_file.as_ptr()) }
}
