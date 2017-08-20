extern crate cgmath;

use sdl2::video::Window;
use sdl2::render::Canvas;
use sdl2::rect::Rect;
use sdl2::render::Texture as SdlTexture;
use graphicsdevice::GraphicsDevice;
use renderstate::RenderState;
use spritebatcher::SpriteBatcher;
use spritebatchitem::SpriteBatchItem;
use rectangle::Rectangle;
use spritefont::SpriteFont;
use shader::Shader;
use texture::Texture;
use log::Log;
use color::Color;
use self::cgmath::Matrix4;
use self::cgmath::Vector2;
use self::cgmath::One;
use std::option::Option;
use std::f32;
use std::rc::Rc;
use std::ops::Mul;
use std::cell::RefCell;

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

pub struct SpriteBatch {
    render_state: RenderState,
    graphics_device: GraphicsDevice,
    batcher: SpriteBatcher,
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

impl SpriteBatch {
    pub fn new() -> SpriteBatch {
        let mut gd = GraphicsDevice::new();
        gd.initialize();
        SpriteBatch {
            render_state: RenderState::new(None, None),
            graphics_device: gd,
            batcher: SpriteBatcher::new(),
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

    pub fn compute_cull_rectangle<'c>(&mut self, renderer: &'c Canvas<Window>) {
        let vp = renderer.viewport();
        
        self.cull_rect.x = vp.x as f32;
        self.cull_rect.y = vp.y as f32;
        self.cull_rect.w = vp.w;
        self.cull_rect.h = vp.h;
    }

    pub fn begin<'c>(&mut self, renderer: &'c Canvas<Window>, sortMode: SpriteSortMode/*, BlendState *blendState = NULL, SamplerState *samplerState = NULL, DepthStencilState *depthStencilState = NULL, RasterizerState *rasterizerState = NULL, Effect *effect = NULL*/, shader: Option<Shader>, transformMatrix: Option<Matrix4<f32>>) {
        self.render_state.shader = shader;
        if transformMatrix.is_some() {
            self.matrix = transformMatrix.unwrap();
        } else {
            self.matrix = Matrix4::one();
        }
        self.render_state.transform = self.matrix;
        self.sort_mode = sortMode;
        self.compute_cull_rectangle(renderer);
        match self.sort_mode
        {
            SpriteSortMode::SpriteSortModeImmediate => {
                self.setup(renderer);
            },
            _ => {},
        }

        self.begin_called = true;
    }

    pub fn end<'c>(&mut self, renderer: &'c Canvas<Window>) {
        self.begin_called = false;
        match self.sort_mode {
            SpriteSortMode::SpriteSortModeImmediate => {},
            _ => {
                self.setup(renderer);
            },
        }
        self.batcher.draw_batch(self.sort_mode, &mut self.render_state, &mut self.graphics_device);
    }

    pub fn setup<'c>(&mut self, renderer: &'c Canvas<Window>) {
        let vp = renderer.viewport();

        //In OpenGL the viewport is bottom left origin, so we flip the y
        //when submitting our top left based coordinates.
        //We use the target size property of the renderer, which
        //when rendering to the screen matches the window and when
        //rendering to a texture/render target, matches the target.
        let mut _y: f32 = 0.0;
        let (rendererWidth, rendererHeight) = renderer.output_size().unwrap();
        _y = (rendererHeight - (vp.y as u32 + vp.h as u32)) as f32;

        self.render_state.viewport = Rectangle::new(vp.x as f32, _y, vp.w as i32, vp.h as i32);
        
        
        // Normal 3D cameras look into the -z direction (z = 1 is in font of z = 0). The
        // sprite batch layer depth is the opposite (z = 0 is in front of z = 1).
        // --> We get the correct matrix with near plane 0 and far plane -1.
        let mut projection: Matrix4<f32> = GraphicsDevice::createOrthographicMatrixOffCenter(0.0, vp.w as f32, vp.h as f32, 0.0, 0.0, -1.0);
        projection = Matrix4::mul(self.matrix, projection);
    }

    pub fn draw_internal(&mut self, texture: Rc<Texture>,
                            /* destinationRectangle: Rectangle, */
                               sourceRectangle: Option<Rectangle>, color: Color,
                               rotation: f32, /* origin: Vector2<f32>, */
                               /*SpriteEffects *effects, */ depth: f32,
                               autoFlush: bool) {
        
        // Cull geometry outside the viewport
        self.vertexToCullTL.x = self.origin_rect.x + -self.scaled_origin.x * rotation.cos() - -self.scaled_origin.y * rotation.sin();
        self.vertexToCullTL.y = self.origin_rect.y + -self.scaled_origin.x * rotation.sin() + -self.scaled_origin.y * rotation.cos();
        
        self.vertexToCullTR.x = self.origin_rect.x + (-self.scaled_origin.x + self.origin_rect.w as f32) * rotation.cos() - -self.scaled_origin.y * rotation.sin();
        self.vertexToCullTR.y = self.origin_rect.y + (-self.scaled_origin.x + self.origin_rect.w as f32) * rotation.sin() + -self.scaled_origin.y * rotation.cos();
        
        self.vertexToCullBL.x = self.origin_rect.x + -self.scaled_origin.x * rotation.cos() - (-self.scaled_origin.y + self.origin_rect.h as f32) * rotation.sin();
        self.vertexToCullBL.y = self.origin_rect.y + -self.scaled_origin.x * rotation.sin() + (-self.scaled_origin.y + self.origin_rect.h as f32) * rotation.cos();

        self.vertexToCullBR.x = self.origin_rect.x + (-self.scaled_origin.x + self.origin_rect.w as f32) * rotation.cos() - (-self.scaled_origin.y + self.origin_rect.h as f32) * rotation.sin();
        self.vertexToCullBR.y = self.origin_rect.y + (-self.scaled_origin.x + self.origin_rect.w as f32) * rotation.sin() + (-self.scaled_origin.y + self.origin_rect.h as f32) * rotation.cos();
        
        
        /*if (!cullRect.containsAnyPoint(vertexToCullTL, vertexToCullTR, vertexToCullBL, vertexToCullBR)) {
            return;
        }*/

        if sourceRectangle.is_some() {
            let src = sourceRectangle.unwrap();
            self.temp_rect.x = src.x;
            self.temp_rect.y = src.y;
            self.temp_rect.w = src.w;
            self.temp_rect.h = src.h;
        } else {
            self.temp_rect.x = 0.0;
            self.temp_rect.y = 0.0;
            self.temp_rect.w = texture.get_width() as i32;
            self.temp_rect.h = texture.get_height() as i32;
        }

        self.texCoordTL.x = self.temp_rect.x / texture.get_width() as f32;
        self.texCoordTL.y = self.temp_rect.y / texture.get_height() as f32;
        self.texCoordBR.x = (self.temp_rect.x + self.temp_rect.w as f32) / texture.get_width() as f32;
        self.texCoordBR.y = (self.temp_rect.y + self.temp_rect.h as f32) / texture.get_height() as f32;

        /*if ((effect & SpriteEffects.FlipVertically) != 0) {
            var temp = _texCoordBR.Y;
            _texCoordBR.Y = _texCoordTL.Y;
            _texCoordTL.Y = temp;
        }
        if ((effect & SpriteEffects.FlipHorizontally) != 0) {
            var temp = _texCoordBR.X;
            _texCoordBR.X = _texCoordTL.X;
            _texCoordTL.X = temp;
        }*/

        //Log::debug(&texture.get_height().to_string());
        {
            let mut item = self.batcher.create_batch_item();
            item.set_with_rotation(self.origin_rect.x, self.origin_rect.y, 
                        -self.scaled_origin.x, -self.scaled_origin.y, self.origin_rect.w as f32, self.origin_rect.h as f32,
                        rotation.sin(), rotation.cos(), color, self.texCoordTL,
                        self.texCoordBR, depth, texture);
            //Log::debug("{:?}", item.vertexTL.position);
            //Log::debug("{:?}", item.vertexTR.position);
            //Log::debug("{:?}", item.vertexBL.position);
            //Log::debug("{:?}", item.vertexBR.position);

            // set SortKey based on SpriteSortMode.
            match self.sort_mode {
                    // Comparison of Texture objects.
                SpriteSortMode::SpriteSortModeTexture => {
                    //item->sortKey = texture->sortingKey;
                },
                    // Comparison of Depth
                SpriteSortMode::SpriteSortModeFrontToBack => {
                    item.sortKey = depth;
                },
                    // Comparison of Depth in reverse
                SpriteSortMode::SpriteSortModeBackToFront => {
                    item.sortKey = -depth;
                },
                _ => {},
            }
        }

        if autoFlush {
            self.flush_if_needed();
        }
    }

    // Mark the end of a draw operation for Immediate SpriteSortMode.
    pub fn flush_if_needed(&mut self) {
        match self.sort_mode {
            SpriteSortMode::SpriteSortModeImmediate => {
                self.batcher.draw_batch(self.sort_mode/*, _effect*/, &mut self.render_state, &mut self.graphics_device);
            },
            _ => {}
        }
    }

    pub fn draw(&mut self, texture: Rc<Texture>, position: Option<Vector2<f32>>,
               destinationRectangle: Option<Rectangle>,
               sourceRectangle: Option<Rectangle>, origin: Option<Vector2<f32>>,
               rotation: f32, scale: Option<Vector2<f32>>, color: Color,
               /*SpriteEffects *effects, */ layerDepth: f32) {
        let mut baseOrigin = Vector2::new(0.0, 0.0);
        let mut baseScale = Vector2::new(1.0, 1.0);
        // Assign default values to null parameters here, as they are not compile-time
        // constants
        // if (color == nullptr) {
        //    color = sf::Color(255, 255, 255, 255);
        //}
        if origin.is_some() {
            baseOrigin = origin.unwrap();
        }
        if scale.is_some() {
            baseScale = scale.unwrap();
        }

        // If both drawRectangle and position are null, or if both have been assigned
        // a value, raise an error
        if (destinationRectangle.is_some() && position.is_some()) ||
            (destinationRectangle.is_none() && position.is_none()) {
            Log::error(
                "Expected drawRectangle or position, but received neither or both.");
        } else if position.is_some() {
            // Call Draw() using position
            Log::error(
                "Calling draw_vector_scale");
            Log::debug("SpriteBatch::draw() sourceRectangle");
            //Log::debug("{:?}", sourceRectangle);
            Log::debug("SpriteBatch::draw() position");
            //Log::debug("{:?}", position);
            self.draw_vector_scale(texture, position, sourceRectangle, color, rotation, baseOrigin, baseScale,
                /*effects,*/ layerDepth);
        } else {
            // Call Draw() using drawRectangle
            Log::error(
                "This should call with drawRectangle but we're not yet supporting it");
            // Draw(texture, (Rectangle)destinationRectangle, sourceRectangle,
            // (Color)color, rotation, (Vector2)origin, effects, layerDepth);
        }
    }

    pub fn draw_vector_scale(&mut self, texture: Rc<Texture>, position: Option<Vector2<f32>>,
                       sourceRectangle: Option<Rectangle>, color: Color,
                       rotation: f32, origin: Vector2<f32>, scale: Vector2<f32>,
                       /*SpriteEffects *effects,*/
                       layerDepth: f32) {
        // CheckValid(texture);

        let mut w = texture.get_width() as f32 * scale.x;
        let mut h = texture.get_height() as f32 * scale.y;
        let mut src: Option<Rectangle>;
        match sourceRectangle
        {
            Some(v) => {
                w = v.w as f32 * scale.x;
                h = v.h as f32 * scale.y;
                src = Some(v);
            },
            None => {
                src = None;
            },
        }

        let pos = position.unwrap();
        self.scaled_origin.x = origin.x * scale.x;
        self.scaled_origin.y = origin.y * scale.y;
        self.origin_rect.x = pos.x;
        self.origin_rect.y = pos.y;
        self.origin_rect.w = w as i32;
        self.origin_rect.h = h as i32;
        self.draw_internal(texture, /*self.origin_rect,*/ src, color, rotation,
                    /*self.scaled_origin,*/
                    /*effects,*/
                    layerDepth, true);
    }

    pub fn draw_float_scale(&mut self, texture: Rc<Texture>, position: Vector2<f32>,
                       sourceRectangle: Rectangle, color: Color,
                       rotation: f32, origin: Vector2<f32>, scale: f32,
                       /*SpriteEffects effects,*/
                       layerDepth: f32) {
        // CheckValid(texture);
        let s = Vector2::new(scale, scale);
        self.draw_vector_scale(texture, Some(position), Some(sourceRectangle), color, rotation, origin, s, layerDepth);
    }

    pub fn draw_position(&mut self, texture: Rc<Texture>, position: Vector2<f32>) {
        self.draw(texture, Some(position), None, None, None, 0.0, None, Color::white(), 0.0);
    }

    pub fn draw_noscale(&mut self, texture: Rc<Texture>, destinationRectangle: Rectangle,
                       sourceRectangle: Option<Rectangle>, color: Color,
                       rotation: f32, origin: Vector2<f32>,
                       /*SpriteEffects effects,*/
                       layerDepth: f32) {
        // CheckValid(texture);

        self.origin_rect.x = destinationRectangle.x;
        self.origin_rect.y = destinationRectangle.y;
        self.origin_rect.w = destinationRectangle.w;
        self.origin_rect.h = destinationRectangle.h;

        if sourceRectangle.is_some() && sourceRectangle.unwrap().w != 0 {
            self.scaled_origin.x =
                origin.x * (destinationRectangle.w as f32 /
                            sourceRectangle.unwrap().w as f32);
        } else {
            self.scaled_origin.x =
                origin.x * (destinationRectangle.w as f32 /
                            texture.get_width() as f32);
        }

        if sourceRectangle.is_some() && sourceRectangle.unwrap().h != 0 {
            self.scaled_origin.y =
                origin.y * (destinationRectangle.h as f32 /
                            sourceRectangle.unwrap().h as f32);
        } else {
            self.scaled_origin.y =
                origin.y * (destinationRectangle.h as f32 /
                            texture.get_height() as f32);
        }

        self.draw_internal(texture, /* self.origin_rect,*/ sourceRectangle, color, rotation,
                    /*self.scaled_origin,*/
                    /*effects,*/
                    layerDepth, true);
    }

    pub fn draw_dst_src_color(&mut self, texture: Rc<Texture>, destinationRectangle: Rectangle,
                        sourceRectangle : Rectangle, color: Color) {
        self.draw_noscale(texture, destinationRectangle, Some(sourceRectangle), color, 0.0, Vector2::new(0.0, 0.0),
        /*SpriteEffects.None,*/ 0.0);
    }

    /*
    void SpriteBatch::DrawString(SpriteFont *spriteFont, std::string text,
                             Vector2 position,
                             Color color, Vector2 scale, Vector2 origin,
                             float rotation, /*SpriteEffects effects,*/ float layerDepth) {
        //CheckValid(spriteFont, text);
        spriteFont->DrawInto(this, text, position, color, rotation, origin, scale, /*effects, */ layerDepth);

        // renderTarget->draw(this->text, renderStates);
    }
    */

}