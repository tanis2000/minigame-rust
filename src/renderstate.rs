extern crate cgmath;

use blendmode::BlendMode;
use blendmode::BlendAlpha;
use shader::Shader;
use texture::Texture;
use rectangle::Rectangle;
use self::cgmath::Matrix4;
use self::cgmath::One;
use std::ptr;
use std::rc::Rc;

pub struct RenderState<'a> {
    pub blendMode: BlendMode,
    pub transform: Matrix4<f32>,
    pub texture: Option<Rc<Texture>>,
    pub shader: Option<&'a Shader>,
    pub viewport: Rectangle,
}

impl<'a> RenderState<'a> {
    pub fn new(texture: Option<Rc<Texture>>, shader: Option<&'a Shader>) -> RenderState<'a> {
        RenderState {
            blendMode: BlendAlpha,
            transform: Matrix4::one(),
            texture: texture,
            shader: shader,
            viewport: Rectangle::new(0.0, 0.0, 0, 0),
        }
    }

    pub fn set_texture(&mut self, texture: Option<Rc<Texture>>) {
        self.texture = texture;
    }
}
