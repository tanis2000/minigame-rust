extern crate cgmath;

use color::Color;
use component::{Component};
use rectangle::Rectangle;
use texture::Texture;
use spritebatch::SpriteBatch;
use subtexture::Subtexture;
use entity::Entity;
use self::cgmath::Vector2;
use std::collections::HashMap;
use std::rc::Rc;
use std::option::Option;
use log::Log;

pub struct SpriteFrame {
    subtexture: Subtexture,
    origin: Vector2<f32>,
}

impl SpriteFrame {
    pub fn new() -> Self {
        SpriteFrame {
            subtexture: Subtexture::new(),
            origin: Vector2::new(0.0, 0.0),
        }
    }

    pub fn with_subtexture_and_origin(subtexture: Subtexture, origin: Vector2<f32>) -> Self {
        let mut sf = SpriteFrame::new();
        sf.origin = origin;
        sf.subtexture = subtexture;
        sf
    }

    pub fn with_subtexture(subtexture: Subtexture) -> Self {
        let mut sf = SpriteFrame::new();
        sf.subtexture = subtexture;
        sf
    }

    pub fn get_texture(&self) -> Option<Rc<Texture>> {
        match self.subtexture.texture {
            Some(ref texture) => {
                return Some(texture.clone());
            },
            None => {
                return None;
            }
        }
    }
}

pub struct SpriteAnimation {
    frames: Vec<usize>,
    delay: f32,
    looping: bool,
}

impl SpriteAnimation {
    pub fn new() -> Self {
        SpriteAnimation {
            frames: Vec::new(),
            delay: 0.0,
            looping: false,
        }
    }
}
pub struct SpriteComponent {
    position: Vector2<f32>,
    origin: Vector2<f32>,
    scale: Vector2<f32>,
    zoom: f32,
    rotation: f32,
    color: Color,

    frames: Vec<SpriteFrame>,
    playing: bool,
    finished: bool,
    rate: f32,
    current_frame: usize,
    animations: HashMap<usize, SpriteAnimation>,
    current_animation: Option<Rc<SpriteAnimation>>,
    current_animation_id: usize,
    current_animation_frame: usize,
    timer: f32,
}

impl Component for SpriteComponent {
    type Storage = HashMap<usize, Self>;
}

impl SpriteComponent {
    pub fn new() -> Self {
        SpriteComponent {
            position: Vector2::new(0.0, 0.0),
            origin: Vector2::new(0.0, 0.0),
            scale: Vector2::new(1.0, 1.0),
            zoom: 1.0,
            rotation: 0.0,
            color: Color::white(),

            frames: Vec::new(),
            playing: false,
            finished: false,
            rate: 1.0,
            current_frame: 0,
            animations: HashMap::new(),
            current_animation: None,
            current_animation_id: 0,
            current_animation_frame: 0,
            timer: 0.0,
        }
    }

    pub fn add_frame(&mut self, frame: SpriteFrame) {
        self.frames.push(frame);
    }

    pub fn add_frame_with_subtexture_and_origin(&mut self, subtexture: Subtexture, origin: Vector2<f32>) {
        self.add_frame(SpriteFrame::with_subtexture_and_origin(subtexture, origin));
    }

    pub fn add_frame_with_subtexture(&mut self, subtexture: Subtexture) {
        self.add_frame(SpriteFrame::with_subtexture(subtexture));
    }

    pub fn render_position(&self, entity: Option<&Entity>) -> Vector2<f32> {
        match entity {
            Some(_) => {
                //return entity.unwrap().position + self.position;
                // TODO: grab the position from the TransformComponent?
                return Vector2::new(0.0, 0.0);
            },
            None => {
                return Vector2::new(0.0, 0.0);
            }
        }
    }

    pub fn render(&self, entity: Option<&Entity>, spritebatch: &mut SpriteBatch) {
        Log::info("Render called");
        let render_pos = self.render_position(entity);
        let scaled_vec = self.scale * self.zoom;
        let t = self.frames.get(self.current_frame).unwrap().subtexture.texture.as_ref().unwrap().clone();
        let clip_rect = self.frames.get(self.current_frame).unwrap().subtexture.get_rect();
        spritebatch.draw_vector_scale(t, Some(render_pos), Some(*clip_rect), self.color, self.rotation,
        self.origin, scaled_vec, 0.0);
    }

    pub fn get_current_frame(&self) -> usize {
        self.current_frame
    }

    pub fn get_frame(&self, index: usize) -> Option<&SpriteFrame> {
        self.frames.get(index)
    }

    pub fn get_source_rect(&self) -> &Rectangle {
        let clip_rect = self.frames.get(self.current_frame).unwrap().subtexture.get_rect();
        return clip_rect;
    }
}