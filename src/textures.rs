use crate::core::IntoCStr;
use crate::enums::*;
use crate::generated::*;

//------------------------------------------------------------------------------------
// Image Loading, Generation, and Manipulation Functions
//------------------------------------------------------------------------------------

pub fn load_image(path: impl IntoCStr) -> Image {
    let c_path = path.into_c_string();
    unsafe { LoadImage(c_path.as_ptr()) }
}

pub fn load_image_from_memory(file_type: impl IntoCStr, file_data: &[u8]) -> Image {
    let c_type = file_type.into_c_string();
    unsafe {
        LoadImageFromMemory(
            c_type.as_ptr(),
            file_data.as_ptr(),
            file_data.len() as i32,
        )
    }
}

pub fn load_image_from_texture(texture: Texture2D) -> Image {
    unsafe { LoadImageFromTexture(texture) }
}

pub fn load_image_from_screen() -> Image {
    unsafe { LoadImageFromScreen() }
}

pub fn is_image_valid(image: Image) -> bool {
    unsafe { IsImageValid(image) }
}

pub fn unload_image(image: Image) {
    unsafe {
        UnloadImage(image);
    }
}

pub fn export_image(image: Image, path: impl IntoCStr) -> bool {
    let c_path = path.into_c_string();
    unsafe { ExportImage(image, c_path.as_ptr()) }
}

pub fn export_image_to_memory(image: Image, file_type: impl IntoCStr) -> Option<Vec<u8>> {
    let c_type = file_type.into_c_string();
    let mut file_size: i32 = 0;
    unsafe {
        let ptr = ExportImageToMemory(image, c_type.as_ptr(), &mut file_size);
        if ptr.is_null() || file_size <= 0 {
            None
        } else {
            let slice = std::slice::from_raw_parts(ptr, file_size as usize);
            let vec = slice.to_vec();
            MemFree(ptr as *mut ::std::os::raw::c_void);
            Some(vec)
        }
    }
}

pub fn gen_image_color(width: i32, height: i32, color: Color) -> Image {
    unsafe { GenImageColor(width, height, color) }
}

pub fn gen_image_checked(
    width: i32,
    height: i32,
    checks_x: i32,
    checks_y: i32,
    col1: Color,
    col2: Color,
) -> Image {
    unsafe { GenImageChecked(width, height, checks_x, checks_y, col1, col2) }
}

pub fn get_image_color(image: Image, x: i32, y: i32) -> Color {
    unsafe { GetImageColor(image, x, y) }
}

pub fn image_resize(image: &mut Image, new_width: i32, new_height: i32) {
    unsafe { ImageResize(image, new_width, new_height) }
}

pub fn image_flip_vertical(image: &mut Image) {
    unsafe { ImageFlipVertical(image) }
}

pub fn image_flip_horizontal(image: &mut Image) {
    unsafe { ImageFlipHorizontal(image) }
}

//------------------------------------------------------------------------------------
// Texture Loading and Drawing Functions
//------------------------------------------------------------------------------------

pub fn load_texture(path: impl IntoCStr) -> Texture {
    let c_path = path.into_c_string();
    unsafe { LoadTexture(c_path.as_ptr()) }
}

pub fn load_texture_from_image(image: Image) -> Texture2D {
    unsafe { LoadTextureFromImage(image) }
}

pub fn is_texture_valid(texture: Texture2D) -> bool {
    unsafe { IsTextureValid(texture) }
}

pub fn unload_texture(texture: Texture2D) {
    unsafe {
        UnloadTexture(texture);
    }
}

pub fn set_texture_filter(texture: Texture2D, filter: TextureFilter) {
    unsafe {
        SetTextureFilter(texture, filter as i32);
    }
}

pub fn set_texture_wrap(texture: Texture2D, wrap: TextureWrap) {
    unsafe {
        SetTextureWrap(texture, wrap as i32);
    }
}

pub fn draw_texture(texture: Texture2D, pos_x: i32, pos_y: i32, tint: Color) {
    unsafe {
        DrawTexture(texture, pos_x, pos_y, tint);
    }
}

pub fn draw_texture_v(texture: Texture2D, position: Vector2, tint: Color) {
    unsafe {
        DrawTextureV(texture, position, tint);
    }
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

pub fn draw_texture_rec(texture: Texture2D, source: Rectangle, position: Vector2, tint: Color) {
    unsafe {
        DrawTextureRec(texture, source, position, tint);
    }
}

pub fn draw_texture_pro(
    texture: Texture2D,
    source: Rectangle,
    dest: Rectangle,
    origin: Vector2,
    rotation: f32,
    tint: Color,
) {
    unsafe {
        DrawTexturePro(texture, source, dest, origin, rotation, tint);
    }
}

//------------------------------------------------------------------------------------
// RenderTexture Functions
//------------------------------------------------------------------------------------

pub fn load_render_texture(width: i32, height: i32) -> RenderTexture2D {
    unsafe { LoadRenderTexture(width, height) }
}

pub fn is_render_texture_valid(target: RenderTexture2D) -> bool {
    unsafe { IsRenderTextureValid(target) }
}

pub fn unload_render_texture(target: RenderTexture2D) {
    unsafe { UnloadRenderTexture(target) }
}

pub fn begin_texture_mode(target: RenderTexture2D) {
    unsafe { BeginTextureMode(target) }
}

pub fn end_texture_mode() {
    unsafe { EndTextureMode() }
}
