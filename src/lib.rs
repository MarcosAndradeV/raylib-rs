#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

pub mod audio;
pub mod color;
pub mod core;
pub mod d3;
pub mod enums;
pub mod generated;
pub mod math;
pub mod models;
pub mod rlgl;
pub mod shapes;
pub mod text;
pub mod textures;

pub use audio::*;
pub use color::*;
pub use core::*;
pub use d3::*;
pub use enums::*;
pub use generated::*;
pub use math::*;
pub use models::*;
pub use shapes::*;
pub use text::*;
pub use textures::*;

/// Hardware cursor visibility and capture modes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CursorMode {
    /// Standard visible OS cursor (default).
    #[default]
    Normal,
    /// Cursor is hidden while hovering over the window, but not locked.
    Hidden,
    /// Cursor is hidden and locked to the window for continuous 3D mouselook.
    Captured,
}

/// Safely applies a [`CursorMode`] (`Normal`, `Hidden`, or `Captured`) when the window is initialized.
pub fn set_cursor_mode(mode: CursorMode) {
    if is_window_ready() {
        match mode {
            CursorMode::Normal => {
                enable_cursor();
                show_cursor();
            }
            CursorMode::Hidden => {
                enable_cursor();
                hide_cursor();
            }
            CursorMode::Captured => {
                disable_cursor();
            }
        }
    }
}

/// Safely disables the default `KEY_ESCAPE` window-close behavior so `KEY_ESCAPE` can be used for in-game pause menus.
pub fn disable_exit_key() {
    set_exit_key(KeyboardKey::KEY_NULL);
}

/// Safe wrapper around Raylib's `TraceLog`.
pub fn trace_log(level: TraceLogLevel, message: &str) {
    let c_msg = std::ffi::CString::new(message).unwrap_or_default();
    unsafe {
        TraceLog(
            level as std::os::raw::c_int,
            c"%s".as_ptr(),
            c_msg.as_ptr(),
        );
    }
}

#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {
        $crate::trace_log($crate::LOG_INFO, &format!($($arg)*))
    };
}

#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => {
        $crate::trace_log($crate::LOG_WARNING, &format!($($arg)*))
    };
}

#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {
        $crate::trace_log($crate::LOG_ERROR, &format!($($arg)*))
    };
}

#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => {
        $crate::trace_log($crate::LOG_DEBUG, &format!($($arg)*))
    };
}

#[macro_export]
macro_rules! log_trace {
    ($($arg:tt)*) => {
        $crate::trace_log($crate::LOG_TRACE, &format!($($arg)*))
    };
}

/// Convenience prelude module re-exporting all safe Raylib wrappers, types, enums, and colors.
pub mod prelude {
    pub use crate::audio::*;
    pub use crate::color::*;
    pub use crate::core::*;
    pub use crate::enums::*;
    pub use crate::math::*;
    pub use crate::models::*;
    pub use crate::shapes::*;
    pub use crate::text::*;
    pub use crate::textures::*;
    pub use crate::{
        AudioStream, AutomationEvent, AutomationEventList, BoneInfo, BoundingBox, Camera,
        Camera2D, Camera3D, Color, FilePathList, Font, GlyphInfo, Image, Material, MaterialMap,
        Matrix, Mesh, Model, ModelAnimation, ModelSkeleton, Music, NPatchInfo, Quaternion, Ray,
        RayCollision, Rectangle, RenderTexture, RenderTexture2D, Shader, Sound, Texture,
        Texture2D, TextureCubemap, Transform, Vector2, Vector3, Vector4, VrDeviceInfo,
        VrStereoConfig, Wave, cstr, screen_size,
    };
}

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
        $crate::Vector2 {
            x: $crate::get_screen_width() as f32,
            y: $crate::get_screen_height() as f32,
        }
    }};
}
