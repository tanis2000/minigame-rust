extern crate cgmath;

use component::Component;
use self::cgmath::Vector2;

pub struct TransformComponent {
    position: Vector2<f32>,
}

impl Component for TransformComponent {
    type Storage = Vec<Self>;
}


impl TransformComponent {
    pub fn new() -> Self {
        TransformComponent {
            position: Vector2::new(0.0, 0.0),
        }
    }

    pub fn get_position(&self) -> Vector2<f32> {
        return self.position;
    }

    pub fn set_position(&mut self, x: f32, y: f32) {
        self.position.x = x;
        self.position.y = y;
    }
}