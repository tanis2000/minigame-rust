use std::path::Path;
use crate::utils;

#[derive(Serialize, Deserialize)]
pub struct FrameData {
    pub x: usize,
    pub y: usize,
    pub w: usize,
    pub h: usize,
}

#[derive(Serialize, Deserialize)]
pub struct Size {
    pub w: usize,
    pub h: usize,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Frame {
    pub filename: String,
    pub frame: FrameData,
    pub rotated: bool,
    pub trimmed: bool,
    pub sprite_source_size: FrameData,
    pub source_size: Size,
}

#[derive(Serialize, Deserialize)]
pub struct Meta {
    pub size: Size,
}

#[derive(Serialize, Deserialize)]
pub struct Spritesheet {
    pub frames: Vec<Frame>,
    pub meta: Meta,
}

pub fn load_atlas_from_file(path: String) -> Option<Spritesheet> {
    match utils::load_string_from_file(Path::new(&path)) {
        Some(data) => {
            let spritesheet = serde_json::from_str(data.as_ref()).unwrap();
            return Some(spritesheet);
        },
        None => {
            return None;
        }
    }
}