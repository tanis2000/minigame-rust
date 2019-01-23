extern crate cgmath;

use texture::Texture;
use vertexpositioncolortexture::VertexPositionColorTexture;
use color::Color;
use log::Log;
use self::cgmath::Vector2;
use std::cmp::Ordering;
use std::rc::Rc;

#[derive(Clone)]
pub struct SpriteBatchItem {
    pub texture: Option<Rc<Texture>>,
    pub vertex_tl: VertexPositionColorTexture,
    pub vertex_tr: VertexPositionColorTexture,
    pub vertex_bl: VertexPositionColorTexture,
    pub vertex_br: VertexPositionColorTexture,
    pub sort_key: f32,
}

impl SpriteBatchItem {
    pub fn new() -> Self {
        SpriteBatchItem {
            vertex_tl: VertexPositionColorTexture::new(),
            vertex_tr: VertexPositionColorTexture::new(),
            vertex_bl: VertexPositionColorTexture::new(),
            vertex_br: VertexPositionColorTexture::new(),
            texture: None,
            sort_key: 0.0,
        }
    }

    pub fn with_position(x: f32, y: f32, w: f32, h: f32, color: Color, tex_coord_tl: Vector2<f32>, tex_coord_br: Vector2<f32>, depth: f32, texture: Rc<Texture>) -> Self {
        SpriteBatchItem {
            vertex_tl: VertexPositionColorTexture {
                position: Vector2 {
                    x: x,
                    y: y,
                },
                color: color,
                texture_coordinate: Vector2 {
                    x: tex_coord_tl.x,
                    y: tex_coord_tl.y,
                },
            },
            vertex_tr: VertexPositionColorTexture {
                position: Vector2 {
                    x: x + w,
                    y: y,
                },
                color: color,
                texture_coordinate: Vector2 {
                    x: tex_coord_br.x,
                    y: tex_coord_tl.y,
                },
            },
            vertex_bl: VertexPositionColorTexture {
                position: Vector2 {
                    x: x,
                    y: y + h,
                },
                color: color,
                texture_coordinate: Vector2 {
                    x: tex_coord_tl.x,
                    y: tex_coord_br.y,
                },
            },
            vertex_br: VertexPositionColorTexture {
                position: Vector2 {
                    x: x + w,
                    y: y + h,
                },
                color: color,
                texture_coordinate: Vector2 {
                    x: tex_coord_br.x,
                    y: tex_coord_br.y,
                },
            },
            sort_key: depth,
            texture: Some(texture),
        }
    }

    pub fn with_rotation(x: f32, y: f32, dx: f32, dy: f32, w: f32, h: f32, sin: f32, cos: f32, color: Color, tex_coord_tl: Vector2<f32>, tex_coord_br: Vector2<f32>, depth: f32, texture: Rc<Texture>) -> Self {
        SpriteBatchItem {
            vertex_tl: VertexPositionColorTexture {
                position: Vector2 {
                    x: x + dx * cos - dy * sin,
                    y: y + dx * sin + dy * cos,
                },
                color: color,
                texture_coordinate: Vector2 {
                    x: tex_coord_tl.x,
                    y: tex_coord_tl.y,
                },
            },
            vertex_tr: VertexPositionColorTexture {
                position: Vector2 {
                    x: x + (dx + w) * cos - dy * sin,
                    y: y + (dx + w) * sin + dy * cos,
                },
                color: color,
                texture_coordinate: Vector2 {
                    x: tex_coord_br.x,
                    y: tex_coord_tl.y,
                },
            },
            vertex_bl: VertexPositionColorTexture {
                position: Vector2 {
                    x: x + dx * cos - (dy + h) * sin,
                    y: y + dx * sin + (dy + h) * cos,
                },
                color: color,
                texture_coordinate: Vector2 {
                    x: tex_coord_tl.x,
                    y: tex_coord_br.y,
                },
            },
            vertex_br: VertexPositionColorTexture {
                position: Vector2 {
                    x: x + (dx + w) * cos - (dy + h) * sin,
                    y: y + (dx + w) * sin + (dy + h) * cos,
                },
                color: color,
                texture_coordinate: Vector2 {
                    x: tex_coord_br.x,
                    y: tex_coord_br.y,
                },
            },
            sort_key: depth,
            texture: Some(texture),
        }
    }

    pub fn set_with_rotation(&mut self, x: f32, y: f32, dx: f32, dy: f32, w: f32, h: f32, sin: f32, cos: f32, color: Color, tex_coord_tl: Vector2<f32>, tex_coord_br: Vector2<f32>, depth: f32, texture: Rc<Texture>) {
        Log::debug("SpriteBatchItem::set_with_rotation");
        Log::debug(&texture.get_height().to_string());
        self.vertex_tl = VertexPositionColorTexture {
                position: Vector2 {
                    x: x + dx * cos - dy * sin,
                    y: y + dx * sin + dy * cos,
                },
                color: color,
                texture_coordinate: Vector2 {
                    x: tex_coord_tl.x,
                    y: tex_coord_tl.y,
                },
            };
        self.vertex_tr = VertexPositionColorTexture {
                position: Vector2 {
                    x: x + (dx + w) * cos - dy * sin,
                    y: y + (dx + w) * sin + dy * cos,
                },
                color: color,
                texture_coordinate: Vector2 {
                    x: tex_coord_br.x,
                    y: tex_coord_tl.y,
                },
            };
        self.vertex_bl = VertexPositionColorTexture {
                position: Vector2 {
                    x: x + dx * cos - (dy + h) * sin,
                    y: y + dx * sin + (dy + h) * cos,
                },
                color: color,
                texture_coordinate: Vector2 {
                    x: tex_coord_tl.x,
                    y: tex_coord_br.y,
                },
            };
        self.vertex_br = VertexPositionColorTexture {
                position: Vector2 {
                    x: x + (dx + w) * cos - (dy + h) * sin,
                    y: y + (dx + w) * sin + (dy + h) * cos,
                },
                color: color,
                texture_coordinate: Vector2 {
                    x: tex_coord_br.x,
                    y: tex_coord_br.y,
                },
            };
        self.sort_key = depth;
        self.texture = Some(texture);
    }

    pub fn cmp(&self, other: &SpriteBatchItem) -> Ordering { 
        if self.sort_key < other.sort_key {
            return Ordering::Less;
        } else if self.sort_key > other.sort_key {
            return Ordering::Greater;
        } else {
            return Ordering::Equal;
        }
    }

    pub fn set_texture(&mut self, texture: Option<Rc<Texture>>) {
        Log::debug("Setting the texture of the SpriteBatchItem");
        if texture.is_none() {
            Log::debug("Texture is None");
        }
        self.texture = texture;
    }
}

impl Default for SpriteBatchItem {
    fn default() -> SpriteBatchItem {
        SpriteBatchItem::new()
    }
}