use sdl2::render::Texture as SdlTexture;
use std::cell::RefCell;

pub struct Texture {
    pub texture: RefCell<SdlTexture>,
}

impl Texture {
    pub fn new(t: SdlTexture) -> Texture {
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