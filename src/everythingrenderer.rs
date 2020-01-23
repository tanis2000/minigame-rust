use renderer::Renderer;
use scene::Scene;
use spritebatch::SpriteBatch;
use sdl2::video::Window;
use sdl2::render::Canvas;
use rectangle::Rectangle;

pub struct EverythingRenderer {

}

impl EverythingRenderer {
    pub fn new() -> Self {
        EverythingRenderer {}
    }
}

impl<T> Renderer<T> for EverythingRenderer {
    fn render_end <'sb>(&self, scene: &Scene<T>, viewport: Rectangle, spritebatch: &'sb mut SpriteBatch) {
        scene.render_entities();
        spritebatch.end(viewport);
    }
}