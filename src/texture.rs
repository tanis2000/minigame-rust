use sdl2::render::Texture as SdlTexture;
use std::cell::RefCell;

pub struct Texture<'t> {
    pub texture: RefCell<SdlTexture<'t>>,
}

impl<'t> Texture<'t> {
    pub fn new<'sdlt: 't>(t: SdlTexture<'sdlt>) -> Texture<'t> {
        Texture {
            texture: RefCell::new(t),
        }
    }
    pub fn get_width(&self) -> u32 {
        self.texture.borrow().query().width
    }

    pub fn get_height(&self) -> u32 {
        self.texture.borrow().query().height
    }

}