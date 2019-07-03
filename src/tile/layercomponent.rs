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

pub struct LayerComponent {
    position: Vector2<f32>,
    origin: Vector2<f32>,
    scale: Vector2<f32>,
    zoom: f32,
    rotation: f32,
    color: Color,
    texture: Option<Rc<Texture>>,
    layer: Option<tiled_json_rs::Layer>,
}

impl Component for LayerComponent {
    type Storage = HashMap<usize, Self>;
}

impl LayerComponent {
    pub fn new() -> Self {
        LayerComponent {
            position: Vector2::new(0.0, 0.0),
            origin: Vector2::new(0.0, 0.0),
            scale: Vector2::new(1.0, 1.0),
            zoom: 1.0,
            rotation: 0.0,
            color: Color::white(),
            texture: None,
            layer: None,
        }
    }

    pub fn with_texture(texture: Rc<Texture>) -> Self {
        let mut ic = LayerComponent::new();
        ic.texture = Some(texture);
        ic
    }

    pub fn get_texture(&self) -> Option<Rc<Texture>> {
        match self.texture {
            Some(ref texture) => {
                return Some(texture.clone());
            },
            None => {
                return None;
            }
        }
    }

    pub fn set_texture(&mut self, texture: Option<Rc<Texture>>) {
        self.texture = texture;
    }

    pub fn set_layer(&mut self, layer: Option<tiled_json_rs::Layer>) {
        self.layer = layer;
    }

    pub fn get_layer(&self) -> &Option<tiled_json_rs::Layer> {
        &self.layer
    }

}