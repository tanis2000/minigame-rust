use std::rc::Rc;
use std::collections::HashMap;
use std::path::Path;
use sdl2::render::TextureCreator;
use sdl2::video::WindowContext;
use sdl2::image::{LoadTexture, INIT_PNG, INIT_JPG};
use texture::Texture;

pub struct TextureManager<'tm> {
    texture_creator: &'tm TextureCreator<WindowContext>,
    items: HashMap<String, Rc<Texture<'tm>>>,
}

impl<'tm> TextureManager<'tm> {
    pub fn new(texture_creator: &'tm TextureCreator<WindowContext>) -> TextureManager<'tm> {
        TextureManager {
            texture_creator: texture_creator,
            items: HashMap::new(),
        }
    }

    pub fn load(&mut self, id: String, path: &Path) {
        let texture = self.texture_creator.load_texture(path).unwrap();
        let wabbit = Texture::new(texture);
        self.items.insert(id, Rc::new(wabbit));
    }

    pub fn get(&self, id: String) -> Rc<Texture<'tm>> {
        let entry = self.items.get(&id).unwrap();
        entry.clone()
    }

}