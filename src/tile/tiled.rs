use std::path::Path;
use crate::utils;

pub fn load_map_from_file(path: String) -> Option<tiled_json_rs::Map> {
    match utils::load_string_from_file(Path::new(&path)) {
        Some(data) => {
            let map = tiled_json_rs::Map::load_from_str(data.as_str()).unwrap();
            return Some(map);
        },
        None => {
            return None;
        }
    }
}