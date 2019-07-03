use engine::gl::types::*;
use engine::gl as gl;
use std::ptr::null_mut;

pub struct RenderTarget {
    frame_buffer: GLuint,
    render_buffer: GLuint, // used for depth
    texture: GLuint,
}

impl RenderTarget {
    pub fn new(width: u32, height: u32, user_depth: bool, format: GLenum) -> Self {
        let mut res = RenderTarget {
            frame_buffer: 0,
            render_buffer: 0,
            texture: 0,
        };
        unsafe {
              /*
              glGenBuffers creates regular buffers for vertex data, etc.
              glGenFrameBuffers creates a framebuffer object primarily used as render targets for offscreen rendering.
              glGenRenderBuffers creates a renderbuffer object that are specifically used with framebuffer objects for any depth-testing required.
                */

            let mut fb: GLuint = 0;
            gl::GenBuffers(1, &mut fb);
            res.frame_buffer = fb;

            let mut rb: GLuint = 0;
            gl::GenRenderbuffers(1, &mut rb);
            res.render_buffer = rb;

            let mut t: GLuint = 0;
            gl::GenTextures(1, &mut t);
            res.texture = t;

            // set up framebuffer

            // bind the framebuffer
            gl::BindFramebuffer(gl::FRAMEBUFFER, res.frame_buffer);

            // bind the newly created texture: all future texture functions will modify this texture
            gl::BindTexture(gl::TEXTURE_2D, res.texture);
            // Give an empty image to OpenGL ( the last "0" )
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as i32, width as i32, height as i32, 0, gl::RGBA, gl::UNSIGNED_BYTE, std::ptr::null_mut());
            // filtering
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            // attach the texture to the bound framebuffer object
            gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_2D, res.texture, 0);

            // set up renderbuffer (depth buffer)
            gl::BindRenderbuffer(gl::RENDERBUFFER, res.render_buffer);
            gl::RenderbufferStorage(gl::RENDERBUFFER, gl::DEPTH_COMPONENT16, width as i32, height as i32);
            gl::FramebufferRenderbuffer(gl::FRAMEBUFFER, gl::DEPTH_ATTACHMENT, gl::RENDERBUFFER, res.render_buffer);

            // clean up
            gl::BindTexture(gl::TEXTURE_2D, 0);
            gl::BindRenderbuffer(gl::RENDERBUFFER, 0);
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);

            //if(glCheckFramebufferStatus(GL_FRAMEBUFFER) != GL_FRAMEBUFFER_COMPLETE) {
            //  binocle_log_error("Framebuffer isn't complete");
            //}

        }

        res
    }

    pub fn get_frame_buffer(&self) -> GLuint {
        self.frame_buffer
    }

    pub fn get_render_buffer(&self) -> GLuint {
        self.render_buffer
    }

    pub fn get_texture(&self) -> GLuint {
        self.texture
    }

}