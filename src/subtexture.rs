extern crate cgmath;

use rectangle::Rectangle;
use texture::Texture;
use self::cgmath::Vector2;
use std::rc::Rc;
use std::option::Option;

pub struct Subtexture {
    pub texture: Option<Rc<Texture>>,
    pub rect: Rectangle,
}

impl Subtexture {
    pub fn new() -> Self {
        Subtexture {
            texture: None,
            rect: Rectangle::new(0.0, 0.0, 0, 0),
        }
    }

    pub fn with_texture(texture: Option<Rc<Texture>>, x: i32, y: i32, width: i32, height: i32) -> Self {
        Subtexture {
            texture: texture,
            rect: Rectangle::new(x as f32, y as f32, width, height),
        }
    }

    pub fn with_subtexture(sub: Subtexture, x: i32, y: i32, width: i32, height: i32) -> Self {
        Subtexture {
            texture: sub.texture,
            rect: Rectangle::new(x as f32, y as f32, width, height),
        }
    }

    pub fn get_x(&self) -> i32 {
        self.rect.x as i32
    }

    pub fn get_y(&self) -> i32 {
        self.rect.y as i32
    }

    pub fn get_width(&self) -> i32 {
        self.rect.w
    }

    pub fn get_height(&self) -> i32 {
        self.rect.h
    }

    pub fn get_size(&self) -> Vector2<f32> {
        Vector2::new(self.get_width() as f32, self.get_height() as f32)
    }
    
    pub fn get_center(&self) -> Vector2<f32> {
        self.get_size() * 0.5
    }

    pub fn get_frame(&self, index: i32, frame_width: i32, frame_height: i32) -> Rectangle {
        let mut x = index * frame_width;
        let y = (x / self.rect.w) * frame_height;
        x %= self.rect.w;
        Rectangle::new(self.get_x() as f32 + x as f32, self.get_y() as f32 + y as f32, frame_width, frame_height)
    }

    pub fn get_absolute_clip_rect(&self, relative_clip_rect: Rectangle) -> Rectangle {
        Rectangle::new(relative_clip_rect.x + self.rect.x, relative_clip_rect.y + self.rect.y,
        relative_clip_rect.w, relative_clip_rect.h)
    }
}