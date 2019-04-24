#[cfg(not(target_arch="wasm32"))]
use backend::GLBackend as BackendImpl;
#[cfg(target_arch="wasm32")]
use backend::WebGLBackend as BackendImpl;


pub struct Backend {
  backend: BackendImpl,
}

impl Backend {
  pub fn build() -> Self {
    Backend {
      backend: BackendImpl::build(),
    }
  }

  pub fn swap_buffers(&mut self) {
    self.backend.swap_buffers()
  }

  pub fn poll_events(&mut self) {
    self.backend.poll_events()
  }
  
  pub fn size(&self) -> (u32, u32) {
    self.backend.size()
  }
}

pub trait AbstractBackend {
  fn build() -> Self;
  fn swap_buffers(&mut self);
  fn poll_events(&mut self);
  fn size(&self) -> (u32, u32);
}