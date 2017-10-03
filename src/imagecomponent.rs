extern crate cgmath;

use color::Color;
use rectangle::Rectangle;
use texture::Texture;
use spritebatch::SpriteBatch;
use subtexture::Subtexture;
use entity::Entity;
use self::cgmath::Vector2;
use std::rc::Rc;
use std::option::Option;
use log::Log;

pub struct ImageComponent {
    position: Vector2<f32>,
    origin: Vector2<f32>,
    scale: Vector2<f32>,
    zoom: f32,
    rotation: f32,
    color: Color,
    pub texture: Option<Rc<Texture>>,
    clip_rect: Rectangle,
}

impl ImageComponent {
    pub fn new() -> Self {
        ImageComponent {
            position: Vector2::new(0.0, 0.0),
            origin: Vector2::new(0.0, 0.0),
            scale: Vector2::new(1.0, 1.0),
            zoom: 1.0,
            rotation: 0.0,
            color: Color::white(),
            texture: None,
            clip_rect: Rectangle::new(0.0, 0.0, 0, 0),
        }
    }

    pub fn with_texture(texture: Rc<Texture>) -> Self {
        let mut ic = ImageComponent::new();
        ic.texture = Some(texture);
        ic.clip_rect = Rectangle::new(0.0, 0.0, ic.texture.as_ref().unwrap().get_width() as i32, ic.texture.as_ref().unwrap().get_height() as i32);
        ic
    }

    pub fn render(&self, entity: Option<&Entity>, spritebatch: &mut SpriteBatch) {
        Log::info("Render called");
        let render_pos = self.render_position(entity);
        let scaled_vec = self.scale * self.zoom;
        let t = self.texture.as_ref().unwrap().clone();
        spritebatch.draw_vector_scale(t, Some(render_pos), Some(self.clip_rect), self.color, self.rotation,
        self.origin, scaled_vec, 0.0);
    }

    pub fn width(&self) -> f32 {
        self.clip_rect.w as f32
    }

    pub fn height(&self) -> f32 {
        self.clip_rect.h as f32
    }

    pub fn center_origin(&mut self) {
        self.origin.x = self.width() / 2.0;
        self.origin.y = self.height() / 2.0;
    }

    pub fn justify_origin_vec(&mut self, at: Vector2<f32>) {
        self.origin.x = self.width() * at.x;
        self.origin.y = self.height() * at.y;
    }

    pub fn justify_origin(&mut self, x: f32, y: f32) {
        self.origin.x = self.width() * x;
        self.origin.y = self.height() * y;
    }

    pub fn swap_subtexture(&mut self, subtexture: Option<Rc<Subtexture>>, clip_rect: Option<Rectangle>) {
        self.texture = subtexture.as_ref().unwrap().clone().texture.clone();
        if clip_rect.is_some() {
            self.clip_rect = clip_rect.unwrap();
        } else {
            self.clip_rect = subtexture.unwrap().rect;
        }
    }

    pub fn render_position(&self, entity: Option<&Entity>) -> Vector2<f32> {
        match entity {
            Some(_) => {
                return entity.unwrap().position + self.position;
            },
            None => {
                return Vector2::new(0.0, 0.0);
            }
        }
    }

}