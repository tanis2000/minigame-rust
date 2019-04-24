//! Window and input (OS specific stuff) is in here

/*
#[cfg(not(target_arch="wasm32"))]
extern crate glutin;
#[cfg(target_arch="wasm32")]
extern crate stdweb;
*/

#[cfg(not(target_arch="wasm32"))]
pub use self::gl_backend::GLBackend;
#[cfg(target_arch="wasm32")]
pub use self::webgl_backend::WebGLBackend;
pub use self::backend::Backend;

#[cfg(not(target_arch="wasm32"))]
mod gl_backend;
#[cfg(target_arch="wasm32")]
mod webgl_backend;
mod backend;