use renderer::Renderer;
use scene::Scene;
use spritebatch::SpriteBatch;
use sdl2::video::Window;
use sdl2::render::Canvas;

pub struct EverythingRenderer {

}

impl EverythingRenderer {
    pub fn new() -> Self {
        EverythingRenderer {}
    }
}

impl Renderer for EverythingRenderer {
    fn render_end <'sb>(&self, scene: &Scene, renderer: &'sb mut Canvas<Window>, spritebatch: &'sb mut SpriteBatch) {
        scene.render_entities();
        spritebatch.end(renderer);
    }
}