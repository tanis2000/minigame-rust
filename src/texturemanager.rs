use std::rc::Rc;
use std::collections::HashMap;
use std::path::Path;
use std::string::String;
use std::io::Read;
use sdl2::rwops::RWops;
use stb_image::image;
use stb_image::image::LoadResult::{ImageU8, ImageF32, Error};
use texture::Texture;
use log::Log;

pub struct TextureManager {
    items: HashMap<String, Rc<Texture>>,
}

impl TextureManager {
    pub fn new() -> TextureManager {
        TextureManager {
            items: HashMap::new(),
        }
    }

    pub fn load(&mut self, id: String, path: &Path) {
        let fs = RWops::from_file(path, "rb");
        match fs {
            Ok(mut r) => {
                let mut data : Vec<u8>;
                match r.len() {
                    Some(size) => {
                        data = vec![0; size];
                        match r.read(&mut data) {
                            Ok(_read_size) => {
                                //let stbimg = image::load(path);
                                let stbimg = image::load_from_memory(&data);
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
                                    Error(error) => {
                                        let e: &str = &error[..];
                                        let er: &str = &format!("Error loading texture: {}", e)[..];
                                        Log::error(er);
                                        return;
                                    },
                                }
                            },
                            Err(e) => {
                                Log::error(&e.to_string());
                                return;
                            }
                        }
                    },
                    None => {
                        Log::error("Cannot read size of stream");
                        return;
                    }
                }
            },
            Err(s) => {
                Log::error(&s);
                return;
            }
        }
    }

    pub fn load_from_memory(&mut self, id: String, data: &[u8]) {
        let stbimg = image::load_from_memory(data);
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
            Error(error) => {
                let e: &str = &error[..];
                let er: &str = &format!("Error loading texture: {}", e)[..];
                Log::error(er);
                return;
            },
        }
    }

    pub fn get(&self, id: &String) -> Rc<Texture> {
        let entry = self.items.get(id).unwrap();
        entry.clone()
    }

}