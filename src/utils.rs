extern crate cgmath;

use self::cgmath::Vector2;
use self::cgmath::Matrix4;

use std::path::Path;
use std::fs::File;
use std::io::Read;
use log::Log;

pub trait Clamp {
    fn clamp(self, low: f32, high: f32) -> f32;
}

impl Clamp for f32 {
    fn clamp(self, low: f32, high: f32) -> f32 {
        if self > high {
            return high;
        } else {
            if self < low {
                return low;
            } else {
                return self;
            }
        }
    }
}

pub trait MinMax {
    fn min_of(a: f32, b: f32, c: f32, d: f32) -> f32;
    fn max_of(a: f32, b: f32, c: f32, d: f32) -> f32;
}

impl MinMax for f32 {
    fn min_of(a: f32, b: f32, c: f32, d: f32) -> f32 {
        a.min(b).min(c).min(d)
    }

    fn max_of(a: f32, b: f32, c: f32, d: f32) -> f32 {
        a.max(b).max(c).max(d)
    }

}

pub fn mtx_mul_v(m: Matrix4<f32>, v: Vector2<f32>) -> Vector2<f32> {
        let x = (v.x * m[0][0]) + (v.y * m[1][0]) + m[3][0];
        let y = (v.x * m[0][1]) + (v.y * m[1][1]) + m[3][1];
        Vector2::new(x, y)
}

/*
pub trait Mul {
    fn mul(self, v: Vector2<f32>) -> Vector2<f32>;
    fn mul(self, m: Matrix4<f32>) -> Vector2<f32>;
}

impl Mul for Matrix4<f32> {
    fn mul(self, v: Vector2<f32>) -> Vector2<f32> {
        let x = (v.x * self[0][0]) + (v.y * self[1][0]) + self[3][0];
        let y = (v.x * self[0][1]) + (v.y * self[1][1]) + self[3][1];
        Vector2::new(x, y)
    }
}

impl Mul for Vector2<f32> {
    fn mul(self, m: Matrix4<f32>) -> Vector2<f32> {
        let x = (self.x * m[0][0]) + (self.y * m[1][0]) + m[3][0];
        let y = (self.x * m[0][1]) + (self.y * m[1][1]) + m[3][1];
        Vector2::new(x, y)
    }
}
*/

pub fn load_string_from_file(path: &Path) -> Option<String> {
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
                            let src = String::from_utf8(data).unwrap();
                            return Some(src);
                        },
                        Err(e) => {
                            Log::error(&e.to_string());
                            return None;
                        }
                    }
                },
                Err(e) => {
                    Log::error(&e.to_string());
                    return None;
                }
            }
        },
        Err(e) => {
            Log::error(&e.to_string());
            return None;
        }
    }
}
