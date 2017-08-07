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
    pub vertexTL: VertexPositionColorTexture,
    pub vertexTR: VertexPositionColorTexture,
    pub vertexBL: VertexPositionColorTexture,
    pub vertexBR: VertexPositionColorTexture,
    pub sortKey: f32,
}

impl SpriteBatchItem {
    pub fn new() -> Self {
        SpriteBatchItem {
            vertexTL: VertexPositionColorTexture::new(),
            vertexTR: VertexPositionColorTexture::new(),
            vertexBL: VertexPositionColorTexture::new(),
            vertexBR: VertexPositionColorTexture::new(),
            texture: None,
            sortKey: 0.0,
        }
    }

    pub fn with_position(x: f32, y: f32, w: f32, h: f32, color: Color, texCoordTL: Vector2<f32>, texCoordBR: Vector2<f32>, depth: f32, texture: Rc<Texture>) -> Self {
        SpriteBatchItem {
            vertexTL: VertexPositionColorTexture {
                position: Vector2 {
                    x: x,
                    y: y,
                },
                color: color,
                textureCoordinate: Vector2 {
                    x: texCoordTL.x,
                    y: texCoordTL.y,
                },
            },
            vertexTR: VertexPositionColorTexture {
                position: Vector2 {
                    x: x + w,
                    y: y,
                },
                color: color,
                textureCoordinate: Vector2 {
                    x: texCoordBR.x,
                    y: texCoordTL.y,
                },
            },
            vertexBL: VertexPositionColorTexture {
                position: Vector2 {
                    x: x,
                    y: y + h,
                },
                color: color,
                textureCoordinate: Vector2 {
                    x: texCoordTL.x,
                    y: texCoordBR.y,
                },
            },
            vertexBR: VertexPositionColorTexture {
                position: Vector2 {
                    x: x + w,
                    y: y + h,
                },
                color: color,
                textureCoordinate: Vector2 {
                    x: texCoordBR.x,
                    y: texCoordBR.y,
                },
            },
            sortKey: 0.0,
            texture: Some(texture),
        }
    }

    pub fn with_rotation(x: f32, y: f32, dx: f32, dy: f32, w: f32, h: f32, sin: f32, cos: f32, color: Color, texCoordTL: Vector2<f32>, texCoordBR: Vector2<f32>, depth: f32, texture: Rc<Texture>) -> Self {
        SpriteBatchItem {
            vertexTL: VertexPositionColorTexture {
                position: Vector2 {
                    x: x + dx * cos - dy * sin,
                    y: y + dx * sin + dy * cos,
                },
                color: color,
                textureCoordinate: Vector2 {
                    x: texCoordTL.x,
                    y: texCoordTL.y,
                },
            },
            vertexTR: VertexPositionColorTexture {
                position: Vector2 {
                    x: x + (dx + w) * cos - dy * sin,
                    y: y + (dx + w) * sin + dy * cos,
                },
                color: color,
                textureCoordinate: Vector2 {
                    x: texCoordBR.x,
                    y: texCoordTL.y,
                },
            },
            vertexBL: VertexPositionColorTexture {
                position: Vector2 {
                    x: x + dx * cos - (dy + h) * sin,
                    y: y + dx * sin + (dy + h) * cos,
                },
                color: color,
                textureCoordinate: Vector2 {
                    x: texCoordTL.x,
                    y: texCoordBR.y,
                },
            },
            vertexBR: VertexPositionColorTexture {
                position: Vector2 {
                    x: x + (dx + w) * cos - (dy + h) * sin,
                    y: y + (dx + w) * sin + (dy + h) * cos,
                },
                color: color,
                textureCoordinate: Vector2 {
                    x: texCoordBR.x,
                    y: texCoordBR.y,
                },
            },
            sortKey: 0.0,
            texture: Some(texture),
        }
    }

    pub fn set_with_rotation(&mut self, x: f32, y: f32, dx: f32, dy: f32, w: f32, h: f32, sin: f32, cos: f32, color: Color, texCoordTL: Vector2<f32>, texCoordBR: Vector2<f32>, depth: f32, texture: Rc<Texture>) {
        Log::debug("SpriteBatchItem::set_with_rotation");
        Log::debug(&texture.get_height().to_string());
        self.vertexTL = VertexPositionColorTexture {
                position: Vector2 {
                    x: x + dx * cos - dy * sin,
                    y: y + dx * sin + dy * cos,
                },
                color: color,
                textureCoordinate: Vector2 {
                    x: texCoordTL.x,
                    y: texCoordTL.y,
                },
            };
        self.vertexTR = VertexPositionColorTexture {
                position: Vector2 {
                    x: x + (dx + w) * cos - dy * sin,
                    y: y + (dx + w) * sin + dy * cos,
                },
                color: color,
                textureCoordinate: Vector2 {
                    x: texCoordBR.x,
                    y: texCoordTL.y,
                },
            };
        self.vertexBL = VertexPositionColorTexture {
                position: Vector2 {
                    x: x + dx * cos - (dy + h) * sin,
                    y: y + dx * sin + (dy + h) * cos,
                },
                color: color,
                textureCoordinate: Vector2 {
                    x: texCoordTL.x,
                    y: texCoordBR.y,
                },
            };
        self.vertexBR = VertexPositionColorTexture {
                position: Vector2 {
                    x: x + (dx + w) * cos - (dy + h) * sin,
                    y: y + (dx + w) * sin + (dy + h) * cos,
                },
                color: color,
                textureCoordinate: Vector2 {
                    x: texCoordBR.x,
                    y: texCoordBR.y,
                },
            };
        self.texture = Some(texture);
    }

    pub fn cmp(&self, other: &SpriteBatchItem) -> Ordering { 
        if self.sortKey < other.sortKey {
            return Ordering::Less;
        } else if self.sortKey > other.sortKey {
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