extern crate cgmath;

use self::cgmath::Vector2;
use color::Color;

#[derive(Debug, Copy, Clone)]
pub struct VertexPositionColorTexture {
    pub position: Vector2<f32>,
    pub color: Color,
    pub textureCoordinate: Vector2<f32>,
}

impl VertexPositionColorTexture {
    pub fn new() -> VertexPositionColorTexture {
        VertexPositionColorTexture {
            position: Vector2 {
                x: 0.0,
                y: 0.0,
            },
            color: Color::new(),
            textureCoordinate: Vector2 {
                x: 0.0,
                y: 0.0,
            }
        }
    }
}