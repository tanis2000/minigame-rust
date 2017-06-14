use sdl2::render::Texture as SdlTexture;
use std::cell::RefCell;

pub struct Texture<'a> {
    pub texture: RefCell<SdlTexture<'a>>,
}