use engine::gl::types::*;
use engine::gl as gl;
use image::{GenericImage, ImageBuffer, RgbaImage, GenericImageView};
use std::mem;

pub struct Texture {
    pub tex_id: GLuint,
    pub width: u32,
    pub height: u32,
}

impl Texture {
    pub fn new() -> Texture {
        Texture {
            tex_id: 0,
            width: 0, 
            height: 0,
        }
    }
    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    pub fn from_image_u8(&mut self, image: image::DynamicImage) {
        self.width = image.width();
        self.height = image.height();
        unsafe {
            let mut tex_id: u32 = 0;
            gl::GenTextures(1, &mut tex_id);
            self.tex_id = tex_id;
            gl::BindTexture(gl::TEXTURE_2D, self.tex_id);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as i32, self.width as i32, self.height as i32, 0, gl::RGBA, gl::UNSIGNED_BYTE, mem::transmute(&image.to_rgba().into_raw()[0]));
        }
    }

}