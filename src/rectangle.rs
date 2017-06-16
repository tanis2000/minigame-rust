#[derive(Debug, Copy, Clone)]
pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub w: i32,
    pub h: i32,
}

impl Rectangle {
    pub fn new(x: f32, y: f32, w: i32, h: i32) -> Rectangle {
        Rectangle {
            x: x,
            y: y,
            w: w,
            h: h,
        }
    }
}