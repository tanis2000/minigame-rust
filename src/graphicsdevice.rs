extern crate cgmath;

use engine::gl::types::*;
use engine::gl as gl;
use self::cgmath::Matrix4;
use self::cgmath::Vector4;
use self::cgmath::Rad;
use self::cgmath::Matrix;
use std::mem;
use std::ptr;
use std::vec;
use std::f32;
use std::ops::Mul;
use std::ffi::CString;
use std::rc::Rc;
use vertexpositioncolortexture::VertexPositionColorTexture;
use renderstate::RenderState;
use blendmode::BlendMode;
use blendmode::Factor;
use blendmode::Equation;
use texture::Texture;
use log::Log;
use rectangle::Rectangle;
use shader::Shader;

pub struct GraphicsDevice {
    vertexAttribute: GLint,
    texCoordAttribute: GLint,
    colorAttribute: GLint,
    normalAttribute: GLint,
    projectionMatrixUniform: GLint,
    modelViewMatrixUniform: GLint,
    imageUniform: GLint,
    vbo: GLuint,
}

impl GraphicsDevice {
    pub fn new() -> Self {
        GraphicsDevice {
            vertexAttribute: 0,
            texCoordAttribute: 0,
            colorAttribute: 0,
            normalAttribute: 0,
            projectionMatrixUniform: 0,
            modelViewMatrixUniform: 0,
            imageUniform: 0,
            vbo: 0,
        }
    }

    pub fn initialize(&mut self) {
        unsafe {
            // Create a Vertex Buffer Object and copy the vertex data to it
            let mut vbo: u32 = 0;
            gl::GenBuffers(1, &mut vbo);
            self.vbo = vbo;
        }
    }

    pub fn createOrthographicMatrixOffCenter(left: f32, right: f32, bottom: f32, top: f32, z_near_plane: f32, z_far_plane: f32) -> Matrix4<f32> {
        Matrix4::from_cols(Vector4::new(2.0 / (right - left), 0.0, 0.0, 0.0),
                           Vector4::new(0.0, 2.0 / (top - bottom), 0.0, 0.0),
                           Vector4::new(0.0, 0.0, 1.0 / (z_near_plane - z_far_plane), 0.0),
                           Vector4::new((left + right) / (left - right), (top + bottom) / (bottom - top), z_near_plane / (z_near_plane - z_far_plane), 1.0))
    }

    pub fn createModelViewMatrix(x: f32, y: f32, scale: f32, rotation: f32) -> Matrix4<f32> {
        let theta: f32 = rotation * f32::consts::PI / 180.0;
        let c: f32 = theta.cos();
        let s: f32 = theta.sin();
        
        Matrix4::from_cols(Vector4::new(c*scale, -s*scale, 0.0, 0.0),
                           Vector4::new(s*scale, c*scale, 0.0, 0.0),
                           Vector4::new(0.0, 0.0, 1.0, 0.0),
                           Vector4::new(x, y, 0.0, 1.0))
    }

    pub fn draw(&mut self, vertices: &Vec<VertexPositionColorTexture>, vertexCount: i32, state: &RenderState) {
        GraphicsDevice::resetGLStates();
        GraphicsDevice::applyCurrentView(&state.viewport);
        GraphicsDevice::applyBlendMode(&state.blendMode);
        self.applyShader(&state.shader.unwrap());
        GraphicsDevice::applyTexture(&state.texture);

        let projectionMatrix: Matrix4<f32> = GraphicsDevice::createOrthographicMatrixOffCenter(0.0, state.viewport.w as f32, state.viewport.h as f32, 0.0, -1000.0, 1000.0);
        let modelViewMatrix: Matrix4<f32> = GraphicsDevice::createModelViewMatrix(0.0, 0.0, 1.0, 0.0);
        
        unsafe {
            gl::EnableVertexAttribArray (self.vertexAttribute as GLuint);
            gl::EnableVertexAttribArray (self.colorAttribute as GLuint);
            gl::EnableVertexAttribArray (self.texCoordAttribute as GLuint);
            
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            Log::debug("GraphicsDevice::draw()");
            Log::debug("vertexCount:");
            Log::debug(&vertexCount.to_string());
            gl::BufferData(gl::ARRAY_BUFFER, (mem::size_of::<VertexPositionColorTexture>() as i32 * vertexCount) as GLsizeiptr, mem::transmute(&vertices[0]), gl::STATIC_DRAW);
            
            gl::VertexAttribPointer(self.vertexAttribute as GLuint, 2, gl::FLOAT, gl::FALSE, mem::size_of::<VertexPositionColorTexture>() as i32, ptr::null());
            gl::VertexAttribPointer(self.colorAttribute as GLuint, 4, gl::FLOAT, gl::FALSE, mem::size_of::<VertexPositionColorTexture>() as i32, (2 * mem::size_of::<GLfloat>()) as *const _);
            gl::VertexAttribPointer(self.texCoordAttribute as GLuint, 2, gl::FLOAT, gl::FALSE, mem::size_of::<VertexPositionColorTexture>() as i32, (4 * mem::size_of::<GLfloat>() + 2 * mem::size_of::<GLfloat>()) as *const _);
            
            let finalMatrix = Matrix4::mul(state.transform,projectionMatrix);
            let inverseMatrix: Matrix4<f32> = Matrix4::from_nonuniform_scale(1.0, 1.0, 1.0);

            gl::UniformMatrix4fv( self.projectionMatrixUniform, 1, gl::FALSE, finalMatrix.as_ptr() );
            gl::UniformMatrix4fv( self.modelViewMatrixUniform, 1, gl::FALSE, inverseMatrix.as_ptr() );

            gl::Uniform1i( self.imageUniform, 0 );
            gl::DrawArrays(gl::TRIANGLES, 0, vertexCount);
            
            gl::DisableVertexAttribArray (self.vertexAttribute as GLuint);
            gl::DisableVertexAttribArray (self.colorAttribute as GLuint);
            gl::DisableVertexAttribArray (self.texCoordAttribute as GLuint);
            gl::UseProgram (gl::ZERO);

            match state.texture.as_ref() {
                None => {
                    Log::warning("GraphicsDevice::draw: Missing texture");
                }, 
                Some(v) => {
                    gl::BindTexture(gl::TEXTURE_2D, 0);
                    //let mut texture = state.texture.as_ref().unwrap().texture.borrow_mut();
                    //texture.gl_unbind_texture();
                }
            }
        }
    }

    pub fn resetGLStates() {
        unsafe {
            gl::Disable(gl::CULL_FACE);
            gl::Disable(gl::DEPTH_TEST);
            
            gl::Enable(gl::BLEND);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }
    }

    pub fn applyTransform(transformMatrix: &Matrix4<f32>) {

    }

    pub fn applyCurrentView(viewport: &Rectangle) {
        unsafe {
            gl::Viewport(viewport.x as i32, viewport.y as i32, viewport.w, viewport.h);
        }
    }

    pub fn applyBlendMode(blendMode: &BlendMode) {
        unsafe {
            gl::BlendFunc(
                                GraphicsDevice::factorToGLConstant(blendMode.colorSrcFactor),
                                GraphicsDevice::factorToGLConstant(blendMode.colorDstFactor));
        }
    }

    pub fn applyTexture(texture: &Option<Rc<Texture>>) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
            match texture.as_ref() {
                None => {
                    Log::warning("GraphicsDevice::applyTexture: Missing texture");
                },
                Some(v) => {
                    gl::BindTexture(gl::TEXTURE_2D, texture.as_ref().unwrap().tex_id);
                    //gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA, texW, texH, 0, gl::RGBA, gl::UNSIGNED_BYTE, texture.as_ref().unwrap.image);
                    //let mut t = texture.as_ref().unwrap().texture.borrow_mut();
                    //let (texW, texH) = t.gl_bind_texture();
                }
            }
        }
    }

    pub fn applyShader(&mut self, shader: &Shader) {
        unsafe {
            {
                let c_str = CString::new("vertexPosition".as_bytes()).unwrap();
                self.vertexAttribute = gl::GetAttribLocation (shader.program, c_str.as_ptr());
            }
            {let c_str = CString::new("vertexTCoord".as_bytes()).unwrap();
            self.texCoordAttribute = gl::GetAttribLocation (shader.program, c_str.as_ptr());}
            {let c_str = CString::new("vertexColor".as_bytes()).unwrap();
            self.colorAttribute = gl::GetAttribLocation (shader.program, c_str.as_ptr());}
            {let c_str = CString::new("vertexNormal".as_bytes()).unwrap();
            self.normalAttribute = gl::GetAttribLocation (shader.program, c_str.as_ptr());}
            {let c_str = CString::new("projectionMatrix".as_bytes()).unwrap();
            self.projectionMatrixUniform = gl::GetUniformLocation (shader.program, c_str.as_ptr());}
            {let c_str = CString::new("modelViewMatrix".as_bytes()).unwrap();
            self.modelViewMatrixUniform = gl::GetUniformLocation (shader.program, c_str.as_ptr());}
            {let c_str = CString::new("tex0".as_bytes()).unwrap();
            self.imageUniform = gl::GetUniformLocation (shader.program, c_str.as_ptr());}
            gl::UseProgram(shader.program);
        }
    }

    fn factorToGLConstant(blend_factor: Factor) -> GLuint {
        match blend_factor {
            Factor::Zero => gl::ZERO,
            Factor::One => gl::ONE,
            Factor::SrcColor => gl::SRC_COLOR,
            Factor::OneMinusSrcColor => gl::ONE_MINUS_SRC_COLOR,
            Factor::DstColor => gl::DST_COLOR,
            Factor::OneMinusDstColor => gl::ONE_MINUS_DST_COLOR,
            Factor::SrcAlpha => gl::SRC_ALPHA,
            Factor::OneMinusSrcAlpha => gl::ONE_MINUS_SRC_ALPHA,
            Factor::DstAlpha => gl::DST_ALPHA,
            Factor::OneMinusDstAlpha => gl::ONE_MINUS_DST_ALPHA,
            _ => {
                Log::error("Invalid value for BlendMode::Factor! Fallback to BlendMode::Zero.");
                //assert(false);
                gl::ZERO
            }
        }
    }

    fn equationToGLConstant(blend_equation: Equation) -> GLuint
    {
        match blend_equation {
            Equation::Add => gl::FUNC_ADD,
            Equation::Subtract => gl::FUNC_SUBTRACT,
            _ => {
                Log::error("Invalid value for BlendMode::Equation! Fallback to BlendMode::Add.");
                //assert(false);
                gl::FUNC_ADD
            }
        }
    }

}

impl Drop for GraphicsDevice {
    fn drop(&mut self) {
        if self.vbo != gl::ZERO {
            unsafe {
                gl::DeleteBuffers(1, &self.vbo)
            }
        }
    }
}