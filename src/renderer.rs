extern crate cgmath;

use camera::Camera;
use spritebatch::SpriteBatch;
use spritebatch::SpriteSortMode;
use shader::Shader;
use viewportadapter::ViewportAdapter;
use viewportadapter::ViewportAdapterTrait;
use scene::Scene;
use engine::gl as gl;

pub trait Renderer {
    fn before_render(&self, scene: &Scene) {

    }

    fn render_begin <'sb>(&self, camera: &'sb mut Camera<ViewportAdapter>, spritebatch: &'sb mut SpriteBatch, shader: Shader) {
        // Sets the current camera viewport if the camera has one
        let mut vp = cgmath::Vector4::new(0, 0, 0, 0);
        match camera.get_viewport_adapter() {
            &Some(va) => {
                let r = va;
                let rr = r.get_viewport();
                //let vp = Rect::new(rr.x as i32, rr.y as i32, rr.w as u32, rr.h as u32);
                //renderer.set_viewport(vp);
                vp = cgmath::Vector4::new(rr.x as i32, rr.y as i32, rr.w as i32, rr.h as i32);
                unsafe {
                    gl::Viewport(rr.x as i32, rr.y as i32, rr.w as i32, rr.h as i32);
                }
            },
            _ => {}
        }

        // MonoGame resets the Viewport to the RT size without asking so we have to let the Camera know to update itself
        camera.force_matrix_update();

        let m = camera.get_transform_matrix();
        spritebatch.begin(vp, SpriteSortMode::SpriteSortModeDeferred, Some(shader), Some(m));
    }
    
    fn render_end <'sb>(&self, scene: &Scene, viewport: cgmath::Vector4<i32>, spritebatch: &'sb mut SpriteBatch);
    
    fn after_render(&self, scene: &Scene) {

    }

    fn render(&self, scene: &Scene) {
        //self.camera.force_matrix_update();
        //let m = self.camera.get_transform_matrix();
        //spritebatch.begin(renderer, SpriteSortMode::SpriteSortModeDeferred, Some(shader), Some(m));
        //Game::GetSpriteBatch()->Begin(
        //  SpriteBatch::SpriteSortModeDeferred, shader, &m, camera->GetViewportAdapter()->GetViewport());
    }
}