
unsafe extern "C" {
    #[link_name = "rlPushMatrix"]
    fn rlPushMatrix();
}
pub fn rl_push_matrix() {
    unsafe {
        rlPushMatrix();
    }
}

unsafe extern "C" {
    #[link_name = "rlPopMatrix"]
    fn rlPopMatrix();
}
pub fn rl_pop_matrix() {
    unsafe {
        rlPopMatrix();
    }
}

unsafe extern "C" {
    #[link_name = "rlTranslatef"]
    fn rlTranslatef(x: f32, y: f32, z: f32);
}
pub fn rl_translatef(x: f32, y: f32, z: f32) {
    unsafe {
        rlTranslatef(x, y, z);
    }
}

unsafe extern "C" {
    #[link_name = "rlRotatef"]
    fn rlRotatef(angle: f32, x: f32, y: f32, z: f32);
}
pub fn rl_rotatef(angle: f32, x: f32, y: f32, z: f32) {
    unsafe {
        rlRotatef(angle, x, y, z);
    }
}
