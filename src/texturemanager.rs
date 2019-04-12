use std::rc::Rc;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::string::String;
use std::io::Read;
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
        let fs = File::open(path);
        match fs {
            Ok(mut fs) => {
                let mut data : Vec<u8>;
                let metadata = fs.metadata();
                match metadata {
                    Ok(metadata) => {
                        let file_size = metadata.len();
                        data = vec![0; file_size as usize];
                        match fs.read(&mut data) {
                            Ok(_read_size) => {
                                let img = image::load_from_memory(&data);
                                match img {
                                    Ok(img) => {
                                        let mut tex = Texture::new();
                                        tex.from_image_u8(img);
                                        self.items.insert(id, Rc::new(tex));
                                    },
                                    Err(err) => {
                                        Log::error(err.description());
                                        return;
                                    }
                                }
                            },
                            Err(err) => {
                                Log::error(err.description());
                                return;
                            }
                        }
                    },
                    Err(err) => {
                        Log::error(err.description());
                        return;
                    }
                }
            },
            Err(err) => {
                Log::error(err.description());
                return;
            }
        }
    }

    pub fn get(&self, id: &String) -> Rc<Texture> {
        let entry = self.items.get(id).unwrap();
        entry.clone()
    }

}