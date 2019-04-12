use renderer::Renderer;
use scene::Scene;
use spritebatch::SpriteBatch;

pub struct EverythingRenderer {

}

impl EverythingRenderer {
    pub fn new() -> Self {
        EverythingRenderer {}
    }
}

impl Renderer for EverythingRenderer {
    fn render_end <'sb>(&self, scene: &Scene, viewport: cgmath::Vector4<i32>, spritebatch: &'sb mut SpriteBatch) {
        scene.render_entities();
        spritebatch.end(viewport);
    }
}