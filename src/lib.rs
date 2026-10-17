#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

pub mod audio;
pub mod color;
pub mod core;
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
pub use enums::*;
pub use generated::*;
pub use math::*;
pub use models::*;
pub use shapes::*;
pub use text::*;
pub use textures::*;

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
