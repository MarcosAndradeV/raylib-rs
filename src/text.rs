use std::ffi::CString;

use crate::core::IntoCStr;
use crate::generated::*;

pub fn get_font_default() -> Font {
    unsafe { GetFontDefault() }
}

pub fn load_font(file_name: impl IntoCStr) -> Font {
    let c_path = file_name.into_c_string();
    unsafe { LoadFont(c_path.as_ptr()) }
}

pub fn is_font_valid(font: Font) -> bool {
    unsafe { IsFontValid(font) }
}

pub fn unload_font(font: Font) {
    unsafe { UnloadFont(font) }
}

pub fn draw_fps(pos_x: i32, pos_y: i32) {
    unsafe { DrawFPS(pos_x, pos_y) }
}

pub fn draw_text(text: impl IntoCStr, pos_x: i32, pos_y: i32, font_size: i32, color: Color) {
    let c_text = text.into_c_string();
    unsafe { DrawText(c_text.as_ptr(), pos_x, pos_y, font_size, color) }
}

pub fn measure_text(text: impl IntoCStr, font_size: i32) -> i32 {
    let c_text = text.into_c_string();
    unsafe { MeasureText(c_text.as_ptr(), font_size) }
}

pub fn measure_text_ex(font: Font, text: impl IntoCStr, fontSize: f32, spacing: f32) -> Vector2 {
    let c_text = text.into_c_string();
    unsafe { MeasureTextEx(font, c_text.as_ptr(), fontSize, spacing) }
}

pub fn measure_text_ex_str(
    font: Font,
    text: impl AsRef<str>,
    font_size: f32,
    spacing: f32,
) -> Vector2 {
    if let Ok(c_text) = CString::new(text.as_ref()) {
        unsafe { MeasureTextEx(font, c_text.as_ptr(), font_size, spacing) }
    } else {
        Vector2 { x: 0.0, y: 0.0 }
    }
}

pub fn draw_text_ex(
    font: Font,
    text: impl IntoCStr,
    position: Vector2,
    font_size: f32,
    spacing: f32,
    tint: Color,
) {
    let c_text = text.into_c_string();
    unsafe {
        DrawTextEx(font, c_text.as_ptr(), position, font_size, spacing, tint);
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
