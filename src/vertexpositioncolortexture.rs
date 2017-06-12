extern crate cgmath;

use self::cgmath::Vector2;
use color::Color;

pub struct VertexPositionColorTexture {
    position: Vector2<f32>,
    color: Color,
    textureCoordinate: Vector2<f32>,
}