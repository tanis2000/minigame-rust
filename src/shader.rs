use engine::gl::types::*;
use engine::gl as gl;
use std::ffi::CString;
use std::ptr;
use std::str;
use std::path::Path;
use std::io::Read;
use sdl2::rwops::RWops;
use log::Log;

enum Type {
    Vertex,
    Fragment,
}

#[cfg(any(target_os="android", target_os="ios"))]
fn precision() -> String {
    String::from("precision mediump float;\n")
}

#[cfg(not(any(target_os="android", target_os="ios")))]
fn precision() -> String {
    String::from("")
}

static VS_SRC: &'static str = "\n\
        attribute vec3 vertexPosition;\n\
        attribute vec2 vertexTCoord;\n\
        attribute vec4 vertexColor;\n\
        attribute vec3 vertexNormal;\n\
        \n\
        varying vec2 tcoord;\n\
        varying vec4 color;\n\
        \n\
        uniform mat4 projectionMatrix;\n\
        uniform mat4 modelViewMatrix;\n\
        \n\
        void main(void) {\n\
        \n\
        gl_Position = projectionMatrix * modelViewMatrix * vec4(vertexPosition, 1.0);\n\
        tcoord = vertexTCoord;\n\
        color = vertexColor;\n\
        vec3 n = vertexNormal;\n\
        gl_PointSize = 1.0;\n\
        \n\
    }";

static FS_SRC: &'static str = "\n\
    uniform sampler2D tex0;\n\
    varying vec2 tcoord;\n\
    varying vec4 color;\n\
    \n\
    void main(void) {\n\
        \n\
        vec4 texcolor = texture2D(tex0, tcoord);\n\
        gl_FragColor = color * texcolor;\n\
        \n\
    }";

#[derive(Debug, Copy, Clone)]
pub struct Shader {
    pub vert_shader: GLuint,
    pub frag_shader: GLuint,
    pub program: GLuint,
}

fn default_vertex_source() -> String {
    precision() + VS_SRC
}

fn default_fragment_source() -> String {
    precision() + FS_SRC
}

impl Shader {
    pub fn new() -> Shader {
        Shader {
            frag_shader: 0,
            vert_shader: 0,
            program: 0,
        }
    }

    fn compile(&mut self, vertex_source: &str, fragment_source: &str) {
        self.vert_shader = self.compile_shader(vertex_source, gl::VERTEX_SHADER);
        self.frag_shader = self.compile_shader(fragment_source, gl::FRAGMENT_SHADER);
        self.program = self.link_program(self.vert_shader, self.frag_shader)
    }

    fn compile_shader(&self, src: &str, ty: GLenum) -> GLuint {
        let shader;
        unsafe {
            shader = gl::CreateShader(ty);
            // Attempt to compile the shader
            let c_str = CString::new(src.as_bytes()).unwrap();
            gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
            gl::CompileShader(shader);

            // Get the compile status
            let mut status = gl::FALSE as GLint;
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);

            // Fail on error
            if status != (gl::TRUE as GLint) {
                let mut len = 0;
                gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
                let mut buf = Vec::with_capacity(len as usize);
                buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
                gl::GetShaderInfoLog(shader,
                                     len,
                                     ptr::null_mut(),
                                     buf.as_mut_ptr() as *mut GLchar);
                panic!("{}",
                       str::from_utf8(&buf)
                           .ok()
                           .expect("ShaderInfoLog not valid utf8"));
            }
        }
        shader
    }

    pub fn link_program(&self, vs: GLuint, fs: GLuint) -> GLuint {
        unsafe {
            let program = gl::CreateProgram();
            gl::AttachShader(program, vs);
            gl::AttachShader(program, fs);
            gl::LinkProgram(program);
            // Get the link status
            let mut status = gl::FALSE as GLint;
            gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);

            // Fail on error
            if status != (gl::TRUE as GLint) {
                let mut len: GLint = 0;
                gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
                let mut buf = Vec::with_capacity(len as usize);
                buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
                gl::GetProgramInfoLog(program,
                                      len,
                                      ptr::null_mut(),
                                      buf.as_mut_ptr() as *mut GLchar);
                panic!("{}",
                       str::from_utf8(&buf)
                           .ok()
                           .expect("ProgramInfoLog not valid utf8"));
            }
            program
        }
    }

    pub fn load_default(&mut self) {
        self.compile(&default_vertex_source(), &default_fragment_source());
    }

    pub fn load_vert(&mut self, path: &Path) {
        let fs = RWops::from_file(path, "rb");
        match fs {
            Ok(mut r) => {
                let mut data : Vec<u8>;
                match r.len() {
                    Some(size) => {
                        data = vec![0; size];
                        let read_res = r.read(&mut data);
                        match read_res {
                            Ok(_read_size) => {
                                let src = String::from_utf8(data).unwrap();
                                self.vert_shader = self.compile_shader(&src, gl::VERTEX_SHADER);
                            },
                            Err(_read_error) => {
                                Log::error("Cannot read file");
                                return;
                            }
                        }
                    },
                    None => {
                        Log::error("Cannot read size of stream");
                        return;
                    }
                }
            },
            Err(s) => {
                Log::error(&s);
                return;
            }
        }
    }

    pub fn load_frag(&mut self, path: &Path, primitives: &str) {
        let fs = RWops::from_file(path, "rb");
        match fs {
            Ok(mut r) => {
                let mut data : Vec<u8>;
                match r.len() {
                    Some(size) => {
                        data = vec![0; size];
                        let read_res = r.read(&mut data);
                        match read_res {
                            Ok(_read_size) => {
                                let mut src = String::from_utf8(data).unwrap();
                                src = src.replace("#include \"primitives.frag\"", primitives);
                                self.frag_shader = self.compile_shader(&src, gl::FRAGMENT_SHADER);
                            },
                            Err(_read_error) => {
                                Log::error("Cannot read file");
                                return;
                            }
                        }
                    },
                    None => {
                        Log::error("Cannot read size of stream");
                        return;
                    }
                }
            },
            Err(s) => {
                Log::error(&s);
                return;
            }
        }
    }
}
