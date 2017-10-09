use engine::gl::types::*;
use engine::gl as gl;
use std::cell::RefCell;
use stb_image::image;
use stb_image::image::*;
use std::mem;
use std::ptr;

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

    pub fn from_image_u8(&mut self, image: image::Image<u8>) {
        self.width = image.width as u32;
        self.height = image.height as u32;
        unsafe {
            let mut tex_id: u32 = 0;
            gl::GenTextures(1, &mut tex_id);
            self.tex_id = tex_id;
            gl::BindTexture(gl::TEXTURE_2D, self.tex_id);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as i32, self.width as i32, self.height as i32, 0, gl::RGBA, gl::UNSIGNED_BYTE, mem::transmute(&image.data[0]));
        }
    }

    pub fn from_image_f32(&mut self, image: image::Image<f32>) {
        self.width = image.width as u32;
        self.height = image.height as u32;
    }

}