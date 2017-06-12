extern crate cgmath;

use blendmode::BlendMode;
use blendmode::BlendAlpha;
use shader::Shader;
use texture::Texture;
use rectangle::Rectangle;
use self::cgmath::Matrix4;
use self::cgmath::One;
use std::ptr;

pub struct RenderState<'a> {
    pub blendMode: BlendMode,
    pub transform: Matrix4<f32>,
    pub texture: &'a mut Texture<'a>,
    pub shader: &'a Shader,
    pub viewport: Rectangle,
}

impl<'a> RenderState<'a> {
    fn new(texture: &'a mut Texture<'a>, shader: &'a Shader) -> RenderState<'a> {
        RenderState {
            blendMode: BlendAlpha,
            transform: Matrix4::one(),
            texture: texture,
            shader: shader,
            viewport: Rectangle::new(0.0, 0.0, 0, 0),
        }
    }
}
