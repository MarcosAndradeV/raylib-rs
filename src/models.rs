use crate::generated::*;

pub fn draw_grid(slices: i32, spacing: f32) {
    unsafe { DrawGrid(slices, spacing) }
}

pub fn draw_cube(position: Vector3, width: f32, height: f32, length: f32, color: Color) {
    unsafe { DrawCube(position, width, height, length, color) }
}

pub fn draw_cube_v(position: Vector3, size: Vector3, color: Color) {
    unsafe { DrawCubeV(position, size, color) }
}

pub fn draw_cube_wires(position: Vector3, width: f32, height: f32, length: f32, color: Color) {
    unsafe { DrawCubeWires(position, width, height, length, color) }
}

pub fn draw_cube_wires_v(position: Vector3, size: Vector3, color: Color) {
    unsafe { DrawCubeWiresV(position, size, color) }
}

pub fn draw_sphere(center_pos: Vector3, radius: f32, color: Color) {
    unsafe { DrawSphere(center_pos, radius, color) }
}

pub fn draw_sphere_wires(
    center_pos: Vector3,
    radius: f32,
    rings: i32,
    slices: i32,
    color: Color,
) {
    unsafe { DrawSphereWires(center_pos, radius, rings, slices, color) }
}

pub fn draw_plane(center_pos: Vector3, size: Vector2, color: Color) {
    unsafe { DrawPlane(center_pos, size, color) }
}

pub fn draw_ray(ray: Ray, color: Color) {
    unsafe { DrawRay(ray, color) }
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

pub fn get_world_to_screen(position: Vector3, camera: Camera3D) -> Vector2 {
    unsafe { GetWorldToScreen(position, camera) }
}

/// Converts a 3D world position to 2D screen coordinates with custom viewport width and height.
pub fn get_world_to_screen_ex(
    position: Vector3,
    camera: Camera3D,
    width: i32,
    height: i32,
) -> Vector2 {
    unsafe { GetWorldToScreenEx(position, camera, width, height) }
}

pub fn get_screen_to_world_ray(position: Vector2, camera: Camera3D) -> Ray {
    unsafe { GetScreenToWorldRay(position, camera) }
}
