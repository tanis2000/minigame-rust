extern crate cgmath;

use sdl2::video::Window;
use sdl2::render::Canvas;
use sdl2::rect::Rect;
use graphicsdevice::GraphicsDevice;
use renderstate::RenderState;
use spritebatcher::SpriteBatcher;
use rectangle::Rectangle;
use spritefont::SpriteFont;
use shader::Shader;
use self::cgmath::Matrix4;
use self::cgmath::Vector2;
use self::cgmath::One;
use std::option::Option;

#[derive(Copy, Clone)]
pub enum SpriteSortMode
{
    /// <summary>
    /// All sprites are drawing when <see cref="SpriteBatch.End"/> invokes, in order of draw call sequence. Depth is ignored.
    /// </summary>
    SpriteSortModeDeferred,
    /// <summary>
    /// Each sprite is drawing at individual draw call, instead of <see cref="SpriteBatch.End"/>. Depth is ignored.
    /// </summary>
    SpriteSortModeImmediate,
    /// <summary>
    /// Same as <see cref="SpriteSortMode.Deferred"/>, except sprites are sorted by texture prior to drawing. Depth is ignored.
    /// </summary>
    SpriteSortModeTexture,
    /// <summary>
    /// Same as <see cref="SpriteSortMode.Deferred"/>, except sprites are sorted by depth in back-to-front order prior to drawing.
    /// </summary>
    SpriteSortModeBackToFront,
    /// <summary>
    /// Same as <see cref="SpriteSortMode.Deferred"/>, except sprites are sorted by depth in front-to-back order prior to drawing.
    /// </summary>
    SpriteSortModeFrontToBack
}

pub struct SpriteBatch<'a> {
    renderer: &'a Canvas<Window>,
    render_state: RenderState<'a>,
    graphics_device: GraphicsDevice,
    batcher: SpriteBatcher<'a>,
    begin_called: bool,
    matrix: Matrix4<f32>,
    temp_rect: Rectangle,
    texCoordTL: Vector2<f32>,
    texCoordBR: Vector2<f32>,
    scaled_origin: Vector2<f32>,
    origin_rect: Rectangle,
    sprite_font: SpriteFont,
    sort_mode: SpriteSortMode,
    // Culling stuff
    cull_rect: Rectangle,
    vertexToCullTL: Vector2<f32>,
    vertexToCullTR: Vector2<f32>,
    vertexToCullBL: Vector2<f32>,
    vertexToCullBR: Vector2<f32>,
}

impl <'a>SpriteBatch<'a> {
    pub fn new(renderer: &'a Canvas<Window>) -> SpriteBatch<'a> {
        let mut gd = GraphicsDevice::new();
        gd.initialize();
        SpriteBatch {
            renderer: renderer,
            render_state: RenderState::new(None, None),
            graphics_device: gd,
            batcher: SpriteBatcher::new(renderer),
            begin_called: false,
            matrix: Matrix4::one(),
            temp_rect: Rectangle::new(0.0, 0.0, 0, 0),
            texCoordTL: Vector2 {
                x: 0.0,
                y: 0.0,
            },
            texCoordBR: Vector2 {
                x: 0.0,
                y: 0.0,
            },
            scaled_origin: Vector2 {
                x: 0.0,
                y: 0.0,
            },
            origin_rect: Rectangle::new(0.0, 0.0, 0, 0),
            sprite_font: SpriteFont {

            },
            sort_mode: SpriteSortMode::SpriteSortModeImmediate,
            cull_rect: Rectangle::new(0.0, 0.0, 0, 0),
            vertexToCullTL: Vector2 {
                x: 0.0,
                y: 0.0,
            },
            vertexToCullTR: Vector2 {
                x: 0.0,
                y: 0.0,
            },
            vertexToCullBL: Vector2 {
                x: 0.0,
                y: 0.0,
            },
            vertexToCullBR: Vector2 {
                x: 0.0,
                y: 0.0,
            },
        }
    }

    pub fn compute_cull_rectangle(&mut self) {
        let vp = self.renderer.viewport();
        //SDL_Rect vp;
        //SDL_RenderGetViewport(renderer, &vp);
        
        self.cull_rect.x = vp.x as f32;
        self.cull_rect.y = vp.y as f32;
        self.cull_rect.w = vp.w;
        self.cull_rect.h = vp.h;
    }

    pub fn begin(&mut self, sortMode: SpriteSortMode/*, BlendState *blendState = NULL, SamplerState *samplerState = NULL, DepthStencilState *depthStencilState = NULL, RasterizerState *rasterizerState = NULL, Effect *effect = NULL*/, shader: Option<&'a Shader>, transformMatrix: Option<Matrix4<f32>>) {
        self.render_state.shader = shader;
        if transformMatrix.is_some() {
            self.matrix = transformMatrix.unwrap();
        } else {
            self.matrix = Matrix4::one();
        }
        self.render_state.transform = self.matrix;
        self.sort_mode = sortMode;
        self.compute_cull_rectangle();
        match self.sort_mode
        {
            SpriteSortMode::SpriteSortModeImmediate => {
                self.setup();
            },
            _ => {},
        }

        self.begin_called = true;
    }

    pub fn end(&'a mut self) {
        self.begin_called = false;
        match self.sort_mode {
            SpriteSortMode::SpriteSortModeImmediate => {},
            _ => {
                self.setup();
            },
        }
        self.batcher.draw_batch(self.sort_mode, &mut self.render_state, &mut self.graphics_device);
    }

    pub fn setup(&mut self) {

    }

}