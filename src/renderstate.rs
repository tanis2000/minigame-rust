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

pub struct RenderState {
    pub blendMode: BlendMode,
    pub transform: Matrix4<f32>,
    pub texture: Option<Rc<Texture>>,
    pub shader: Option<Shader>,
    pub viewport: Rectangle,
}

impl RenderState {
    pub fn new(texture: Option<Rc<Texture>>, shader: Option<Shader>) -> RenderState {
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
