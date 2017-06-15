use sdl2::render::Texture as SdlTexture;
use std::cell::RefCell;

pub struct Texture<'a> {
    pub texture: RefCell<SdlTexture<'a>>,
}

impl<'a> Texture<'a> {
    pub fn get_width(&self) -> u32 {
        self.texture.borrow().query().width
    }

    pub fn get_height(&self) -> u32 {
        self.texture.borrow().query().height
    }

}