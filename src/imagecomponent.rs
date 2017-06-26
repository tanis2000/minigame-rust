extern crate cgmath;

use color::Color;
use rectangle::Rectangle;
use self::cgmath::Vector2;

struct ImageComponent {
    position: Vector2,
    origin: Vector2,
    scale: Vector2,
    zoom: f32,
    rotation: f32,
    color: Color,
}

impl ImageComponent {
    pub fn new() -> Self {
        ImageComponent {
            position: Vector2::new(0.0, 0.0),
            origin: Vector2::new(0.0, 0.0),
            scale: Vector2::new(1.0, 1.0),
            zoom: 1.0,
            rotation: 0.0,
            color: Color::white(),
            texture: Rc<Texture>,
            clip_rect: Rectangle::new(0.0, 0.0, 0, 0),
        }
    }
}