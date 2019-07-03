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

    pub fn get_left(&self) -> f32 {
        self.x
    }

    pub fn get_right(&self) -> f32 {
        self.x + self.w as f32
    }

    pub fn get_top(&self) -> f32 {
        self.y
    }

    pub fn get_bottom(&self) -> f32 {
        self.y + self.w as f32
    }

}