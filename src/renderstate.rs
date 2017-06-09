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
    blendMode: BlendMode,
    transform: Matrix4<f32>,
    texture: Rc<Texture>,
    shader: Rc<Shader>,
    viewport: Rectangle,
}

impl RenderState {
    fn new(texture: Rc<Texture>, shader: Rc<Shader>) -> RenderState {
        RenderState {
            blendMode: BlendAlpha,
            transform: Matrix4::one(),
            texture: texture.clone(),
            shader: shader.clone(),
            viewport: Rectangle{},
        }
    }
}
