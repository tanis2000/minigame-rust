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
        let vp = self.renderer.viewport();
        //SDL_Rect vp;
        //SDL_RenderGetViewport(renderer, &vp);

        //In OpenGL the viewport is bottom left origin, so we flip the y
        //when submitting our top left based coordinates.
        //We use the target size property of the renderer, which
        //when rendering to the screen matches the window and when
        //rendering to a texture/render target, matches the target.
        let mut _y: f32 = 0.0;
        let (rendererWidth, rendererHeight) = self.renderer.output_size().unwrap();
        //SDL_GetRendererOutputSize(renderer, &rendererWidth, &rendererHeight);
        _y = (rendererHeight - (vp.y as u32 + vp.h as u32)) as f32;

        self.render_state.viewport = Rectangle::new(vp.x as f32, _y, vp.w as i32, vp.h as i32);
        
        
        // Normal 3D cameras look into the -z direction (z = 1 is in font of z = 0). The
        // sprite batch layer depth is the opposite (z = 0 is in front of z = 1).
        // --> We get the correct matrix with near plane 0 and far plane -1.
        let mut projection: Matrix4<f32> = GraphicsDevice::createOrthographicMatrixOffCenter(0.0, vp.w as f32, vp.h as f32, 0.0, 0.0, -1.0);
        projection = self.matrix * projection;
        //projection.Multiply(matrix, projection);
        
        //_matrixTransform.SetValue(projection);
        //_spritePass.Apply();
        
        
    }

    pub fn draw_internal(&mut self, texture: &Texture,
                            destinationRectangle: Rectangle,
                               sourceRectangle: Option<Rectangle>, color: Color,
                               rotation: f32, origin: Vector2<f32>,
                               /*SpriteEffects *effects, */ depth: f32,
                               autoFlush: bool) {
        
        // Cull geometry outside the viewport
        self.vertexToCullTL.x = destinationRectangle.x + -origin.x * rotation.cos() - -origin.y * rotation.sin();
        self.vertexToCullTL.y = destinationRectangle.y + -origin.x * rotation.sin() + -origin.y * rotation.cos();
        
        self.vertexToCullTR.x = destinationRectangle.x + (-origin.x + destinationRectangle.w as f32) * rotation.cos() - -origin.y * rotation.sin();
        self.vertexToCullTR.y = destinationRectangle.y + (-origin.x + destinationRectangle.w as f32) * rotation.sin() + -origin.y * rotation.cos();
        
        self.vertexToCullBL.x = destinationRectangle.x + -origin.x * rotation.cos() - (-origin.y + destinationRectangle.h as f32) * rotation.sin();
        self.vertexToCullBL.y = destinationRectangle.y + -origin.x * rotation.sin() + (-origin.y + destinationRectangle.h as f32) * rotation.cos();

        self.vertexToCullBR.x = destinationRectangle.x + (-origin.x + destinationRectangle.w as f32) * rotation.cos() - (-origin.y + destinationRectangle.h as f32) * rotation.sin();
        self.vertexToCullBR.y = destinationRectangle.y + (-origin.x + destinationRectangle.w as f32) * rotation.sin() + (-origin.y + destinationRectangle.h as f32) * rotation.cos();
        
        
        /*if (!cullRect.containsAnyPoint(vertexToCullTL, vertexToCullTR, vertexToCullBL, vertexToCullBR)) {
            return;
        }*/

        
        
        //let item = SpriteBatcher::create_batch_item();
        //item.set_texture(texture);


        if sourceRectangle.is_some() {
            self.temp_rect.x = sourceRectangle.unwrap().x;
            self.temp_rect.y = sourceRectangle.unwrap().y;
            self.temp_rect.w = sourceRectangle.unwrap().w;
            self.temp_rect.h = sourceRectangle.unwrap().h;
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

        let mut item = SpriteBatchItem::with_rotation(destinationRectangle.x, destinationRectangle.y, 
                    -origin.x, -origin.y, destinationRectangle.w as f32, destinationRectangle.h as f32,
                    rotation.sin(), rotation.cos(), color, self.texCoordTL,
                    self.texCoordBR, depth, &mut texture);

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

        if autoFlush {
            self.flush_if_needed();
        }
    }

    // Mark the end of a draw operation for Immediate SpriteSortMode.
    pub fn flush_if_needed(&self) {
        match self.sort_mode {
            SpriteSortMode::SpriteSortModeImmediate => {
                self.batcher.draw_batch(self.sort_mode/*, _effect*/, &mut self.render_state, &mut self.graphics_device);
            },
            _ => {}
        }
    }

    pub fn draw(&self, texture: &Texture, position: Option<Vector2<f32>>,
                        destinationRectangle: Option<Rectangle>,
                        sourceRectangle: Option<Rectangle>, origin: Option<Vector2<f32>>,
                        rotation: f32, scale: Option<Vector2<f32>>, color: Color,
                        /*SpriteEffects *effects, */ layerDepth: f32) {
        let baseOrigin = Vector2::new(0.0, 0.0);
        let baseScale = Vector2::new(1.0, 1.0);
        // Assign default values to null parameters here, as they are not compile-time
        // constants
        // if (color == nullptr) {
        //    color = sf::Color(255, 255, 255, 255);
        //}
        if origin.is_none() {
            origin = Some(baseOrigin);
        }
        if scale.is_none() {
            scale = Some(baseScale);
        }

        // If both drawRectangle and position are null, or if both have been assigned
        // a value, raise an error
        if (destinationRectangle.is_some() && position.is_some()) ||
            (destinationRectangle.is_none() && position.is_none()) {
            Log::error(
                "Expected drawRectangle or position, but received neither or both.");
        } else if position.is_some() {
            // Call Draw() using position
            self.draw_vector_scale(texture, position, sourceRectangle, color, rotation, origin.unwrap(), scale.unwrap(),
                /*effects,*/ layerDepth);
        } else {
            // Call Draw() using drawRectangle
            // Draw(texture, (Rectangle)destinationRectangle, sourceRectangle,
            // (Color)color, rotation, (Vector2)origin, effects, layerDepth);
        }
    }

    pub fn draw_vector_scale(&mut self, texture: &Texture, position: Option<Vector2<f32>>,
                       sourceRectangle: Option<Rectangle>, color: Color,
                       rotation: f32, origin: Vector2<f32>, scale: Vector2<f32>,
                       /*SpriteEffects *effects,*/
                       layerDepth: f32) {
        // CheckValid(texture);

        let mut w = texture.get_width() as f32 * scale.x;
        let mut h = texture.get_height() as f32 * scale.y;
        if sourceRectangle.is_some() {
            w = sourceRectangle.unwrap().w as f32 * scale.x;
            h = sourceRectangle.unwrap().h as f32 * scale.y;
        }

        self.scaled_origin.x = origin.x * scale.x;
        self.scaled_origin.y = origin.y * scale.y;
        self.origin_rect.x = position.unwrap().x;
        self.origin_rect.y = position.unwrap().y;
        self.origin_rect.w = w as i32;
        self.origin_rect.h = h as i32;
        self.draw_internal(texture, self.origin_rect, sourceRectangle, color, rotation,
                    self.scaled_origin,
                    /*effects,*/
                    layerDepth, true);
    }

    pub fn draw_float_scale(&self, texture: &Texture, position: Vector2<f32>,
                       sourceRectangle: Rectangle, color: Color,
                       rotation: f32, origin: Vector2<f32>, scale: f32,
                       /*SpriteEffects effects,*/
                       layerDepth: f32) {
        // CheckValid(texture);
        let s = Vector2::new(scale, scale);
        self.draw_vector_scale(texture, Some(position), Some(sourceRectangle), color, rotation, origin, s, layerDepth);
    }

    pub fn draw_position(&self, texture: &Texture, position: Vector2<f32>) {
        self.draw(texture, Some(position), None, None, None, 0.0, None, Color::white(), 0.0);
    }

    pub fn draw_noscale(&mut self, texture: &Texture, destinationRectangle: Rectangle,
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

        self.draw_internal(texture, self.origin_rect, sourceRectangle, color, rotation,
                    self.scaled_origin,
                    /*effects,*/
                    layerDepth, true);
    }

    pub fn draw_dst_src_color(&self, texture: &Texture, destinationRectangle: Rectangle,
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