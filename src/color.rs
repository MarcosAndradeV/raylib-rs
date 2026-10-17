use crate::generated::*;

// Some Basic Colors
// NOTE: Custom raylib color palette for amazing visuals on WHITE background
pub const LIGHTGRAY: Color = Color { r: 200, g: 200, b: 200, a: 255 }; // Light Gray
pub const GRAY: Color = Color { r: 130, g: 130, b: 130, a: 255 }; // Gray
pub const DARKGRAY: Color = Color { r: 80, g: 80, b: 80, a: 255 }; // Dark Gray
pub const YELLOW: Color = Color { r: 253, g: 249, b: 0, a: 255 }; // Yellow
pub const GOLD: Color = Color { r: 255, g: 203, b: 0, a: 255 }; // Gold
pub const ORANGE: Color = Color { r: 255, g: 161, b: 0, a: 255 }; // Orange
pub const PINK: Color = Color { r: 255, g: 109, b: 194, a: 255 }; // Pink
pub const RED: Color = Color { r: 230, g: 41, b: 55, a: 255 }; // Red
pub const MAROON: Color = Color { r: 190, g: 33, b: 55, a: 255 }; // Maroon
pub const GREEN: Color = Color { r: 0, g: 228, b: 48, a: 255 }; // Green
pub const LIME: Color = Color { r: 0, g: 158, b: 47, a: 255 }; // Lime
pub const DARKGREEN: Color = Color { r: 0, g: 117, b: 44, a: 255 }; // Dark Green
pub const SKYBLUE: Color = Color { r: 102, g: 191, b: 255, a: 255 }; // Sky Blue
pub const BLUE: Color = Color { r: 0, g: 121, b: 241, a: 255 }; // Blue
pub const DARKBLUE: Color = Color { r: 0, g: 82, b: 172, a: 255 }; // Dark Blue
pub const PURPLE: Color = Color { r: 200, g: 122, b: 255, a: 255 }; // Purple
pub const VIOLET: Color = Color { r: 135, g: 60, b: 190, a: 255 }; // Violet
pub const DARKPURPLE: Color = Color { r: 112, g: 31, b: 126, a: 255 }; // Dark Purple
pub const BEIGE: Color = Color { r: 211, g: 176, b: 131, a: 255 }; // Beige
pub const BROWN: Color = Color { r: 127, g: 106, b: 79, a: 255 }; // Brown
pub const DARKBROWN: Color = Color { r: 76, g: 63, b: 47, a: 255 }; // Dark Brown
pub const WHITE: Color = Color { r: 255, g: 255, b: 255, a: 255 }; // White
pub const BLACK: Color = Color { r: 0, g: 0, b: 0, a: 255 }; // Black
pub const BLANK: Color = Color { r: 0, g: 0, b: 0, a: 0 }; // Blank (Transparent)
pub const MAGENTA: Color = Color { r: 255, g: 0, b: 255, a: 255 }; // Magenta
/// (raylib WHITE)
pub const RAYWHITE: Color = Color { r: 245, g: 245, b: 245, a: 255 };

pub fn get_color(hex: u32) -> Color {
    unsafe { GetColor(hex) }
}

pub fn fade(color: Color, alpha: f32) -> Color {
    unsafe { Fade(color, alpha) }
}

impl Eq for Color {}

impl Color {
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    pub fn from_hex(hex: u32) -> Self {
        get_color(hex)
    }

    pub fn fade(&self, alpha: f32) -> Self {
        fade(*self, alpha)
    }

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
