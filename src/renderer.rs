use camera::Camera;
use spritebatch::SpriteBatch;
use spritebatch::SpriteSortMode;
use shader::Shader;
use viewportadapter::ViewportAdapter;
use viewportadapter::ViewportAdapterTrait;
use sdl2::video::Window;
use sdl2::render::Canvas;
use sdl2::rect::Rect;

pub trait Renderer {
    fn before_render();
    fn render <'sb>(renderer: &'sb mut Canvas<Window>, camera: &'sb mut Camera<ViewportAdapter>, spritebatch: &'sb mut SpriteBatch, shader: Shader) {
        // Sets the current camera viewport if the camera has one
        match camera.get_viewport_adapter() {
            &Some(ViewportAdapter) => {
                let r = camera.get_viewport_adapter();
                let rr = r.unwrap().get_viewport();
                let vp = Rect::new(rr.x as i32, rr.y as i32, rr.w as u32, rr.h as u32);
                renderer.set_viewport(vp);
            },
            _ => {}
        }

        // MonoGame resets the Viewport to the RT size without asking so we have to let the Camera know to update itself
        camera.force_matrix_update();

        let m = camera.get_transform_matrix();
        spritebatch.begin(renderer, SpriteSortMode::SpriteSortModeDeferred, Some(shader), Some(m));
    }
    fn after_render();
}