extern crate cgmath;

use self::cgmath::Vector2;
use color::Color;

pub struct VertexPositionColorTexture {
    pub position: Vector2<f32>,
    pub color: Color,
    pub textureCoordinate: Vector2<f32>,
}