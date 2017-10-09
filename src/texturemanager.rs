use std::rc::Rc;
use std::collections::HashMap;
use std::path::Path;
use std::string::String;
use sdl2::render::TextureCreator;
use sdl2::video::WindowContext;
//use sdl2::image::{LoadTexture, INIT_PNG, INIT_JPG};
use stb_image::image;
use stb_image::image::LoadResult::ImageU8;
use stb_image::image::LoadResult::ImageF32;
use texture::Texture;

pub struct TextureManager<'tm> {
    texture_creator: &'tm TextureCreator<WindowContext>,
    items: HashMap<String, Rc<Texture>>,
}

impl <'tm>TextureManager<'tm> {
    pub fn new(texture_creator: &'tm TextureCreator<WindowContext>) -> TextureManager<'tm> {
        TextureManager {
            texture_creator: texture_creator,
            items: HashMap::new(),
        }
    }

    pub fn load(&mut self, id: String, path: &Path) {
        let stbimg = image::load(path);
        match stbimg {
            ImageU8(img) => {
                let sdltex = img;
                let mut tex = Texture::new();
                tex.from_image_u8(sdltex);
                self.items.insert(id, Rc::new(tex));
            },
            ImageF32(img) => {
                let sdltex = img;
                let mut tex = Texture::new();
                tex.from_image_f32(sdltex);
                self.items.insert(id, Rc::new(tex));
            },
            Error => {},
        }
        //let sdltex = self.texture_creator.load_texture(path).unwrap();
    }

    pub fn get(&self, id: &String) -> Rc<Texture> {
        let entry = self.items.get(id).unwrap();
        entry.clone()
    }

}