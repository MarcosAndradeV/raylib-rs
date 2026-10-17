use crate::generated::*;

//------------------------------------------------------------------------------------
// Basic Shapes Drawing Functions
//------------------------------------------------------------------------------------

pub fn draw_pixel(pos_x: i32, pos_y: i32, color: Color) {
    unsafe { DrawPixel(pos_x, pos_y, color) }
}

pub fn draw_pixel_v(position: Vector2, color: Color) {
    unsafe { DrawPixelV(position, color) }
}

pub fn draw_line(start: Vector2, end: Vector2, color: Color) {
    unsafe { DrawLineV(start, end, color) }
}

pub fn draw_line_ex(start_pos: Vector2, end_pos: Vector2, thick: f32, color: Color) {
    unsafe { DrawLineEx(start_pos, end_pos, thick, color) }
}

pub fn draw_circle(center_x: i32, center_y: i32, radius: f32, color: Color) {
    unsafe { DrawCircle(center_x, center_y, radius, color) }
}

pub fn draw_circle_v(position: Vector2, radius: f32, color: Color) {
    unsafe { DrawCircleV(position, radius, color) }
}

pub fn draw_circle_lines(center_x: i32, center_y: i32, radius: f32, color: Color) {
    unsafe { DrawCircleLines(center_x, center_y, radius, color) }
}

pub fn draw_circle_lines_v(center: Vector2, radius: f32, color: Color) {
    unsafe { DrawCircleLinesV(center, radius, color) }
}

pub fn draw_rectangle(pos_x: i32, pos_y: i32, width: i32, height: i32, color: Color) {
    unsafe { DrawRectangle(pos_x, pos_y, width, height, color) }
}

pub fn draw_rectangle_v(position: Vector2, size: Vector2, color: Color) {
    unsafe { DrawRectangleV(position, size, color) }
}

pub fn draw_rectangle_rec(rec: Rectangle, color: Color) {
    unsafe { DrawRectangleRec(rec, color) }
}

pub fn draw_rectangle_pro(rec: Rectangle, origin: Vector2, rotation: f32, color: Color) {
    unsafe { DrawRectanglePro(rec, origin, rotation, color) }
}

pub fn draw_rectangle_lines(pos_x: i32, pos_y: i32, width: i32, height: i32, color: Color) {
    unsafe { DrawRectangleLines(pos_x, pos_y, width, height, color) }
}

pub fn draw_rectangle_lines_ex(rec: Rectangle, line_thick: f32, color: Color) {
    unsafe { DrawRectangleLinesEx(rec, line_thick, color) }
}

pub fn draw_rectangle_rounded(rec: Rectangle, roundness: f32, segments: i32, color: Color) {
    unsafe { DrawRectangleRounded(rec, roundness, segments, color) }
}

pub fn draw_triangle(v1: Vector2, v2: Vector2, v3: Vector2, color: Color) {
    unsafe { DrawTriangle(v1, v2, v3, color) }
}

pub fn draw_triangle_lines(v1: Vector2, v2: Vector2, v3: Vector2, color: Color) {
    unsafe { DrawTriangleLines(v1, v2, v3, color) }
}

//------------------------------------------------------------------------------------
// Basic Shapes Collision Detection Functions
//------------------------------------------------------------------------------------

pub fn check_collision_recs(rec1: Rectangle, rec2: Rectangle) -> bool {
    unsafe { CheckCollisionRecs(rec1, rec2) }
}

pub fn check_collision_circles(
    center1: Vector2,
    radius1: f32,
    center2: Vector2,
    radius2: f32,
) -> bool {
    unsafe { CheckCollisionCircles(center1, radius1, center2, radius2) }
}

pub fn check_collision_circle_rec(center: Vector2, radius: f32, rec: Rectangle) -> bool {
    unsafe { CheckCollisionCircleRec(center, radius, rec) }
}

pub fn check_collision_point_rec(point: Vector2, rec: Rectangle) -> bool {
    unsafe { CheckCollisionPointRec(point, rec) }
}

pub fn check_collision_point_circle(point: Vector2, center: Vector2, radius: f32) -> bool {
    unsafe { CheckCollisionPointCircle(point, center, radius) }
}

pub fn get_collision_rec(rec1: Rectangle, rec2: Rectangle) -> Rectangle {
    unsafe { GetCollisionRec(rec1, rec2) }
}
