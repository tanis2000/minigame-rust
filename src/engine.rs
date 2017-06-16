#[cfg(feature = "hotload")]
extern crate dynamic_reload;

extern crate sdl2;
extern crate cgmath;

//#[cfg(not(feature = "hotload"))]
//extern crate minigame;

#[cfg(feature = "hotload")]
use dynamic_reload::{DynamicReload, Lib, Symbol, Search, PlatformName, UpdateState};
#[cfg(feature = "hotload")]
use std::rc::Rc;
use std::time::Duration;
use std::thread;
use std::str;
use std::ffi::CString;
use std::ptr;
use std::mem;
use std::path::Path;
use std::cell::RefCell;
use sdl2::image::{LoadTexture, INIT_PNG, INIT_JPG};
use sdl2::pixels::Color as SdlColor;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::TextureCreator;
use sdl2::render::Texture as SdlTexture;
use sdl2::video::WindowContext;
#[cfg(not(feature = "hotload"))]
use test_shared::shared_fun;

use spritebatch::SpriteBatch;
use spritebatch::SpriteSortMode;
use color::Color;
use texture::Texture;
use texturemanager::TextureManager;
use shader::Shader;
use self::cgmath::Vector2;
use self::cgmath::Matrix4;
use self::cgmath::One;

pub mod gl {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}
use self::gl::types::*;

#[cfg(feature = "hotload")]
struct Plugins {
    plugins: Vec<Rc<Lib>>,
}

#[cfg(feature = "hotload")]
impl Plugins {
    fn add_plugin(&mut self, plugin: &Rc<Lib>) {
        self.plugins.push(plugin.clone());
    }

    fn unload_plugins(&mut self, lib: &Rc<Lib>) {
        for i in (0..self.plugins.len()).rev() {
            if &self.plugins[i] == lib {
                self.plugins.swap_remove(i);
            }
        }
    }

    fn reload_plugin(&mut self, lib: &Rc<Lib>) {
        Self::add_plugin(self, lib);
    }

    // called when a lib needs to be reloaded.
    fn reload_callback(&mut self, state: UpdateState, lib: Option<&Rc<Lib>>) {
        match state {
            UpdateState::Before => Self::unload_plugins(self, lib.unwrap()),
            UpdateState::After => Self::reload_plugin(self, lib.unwrap()),
            UpdateState::ReloadFailed(_) => println!("Failed to reload"),
        }
    }
}

fn find_sdl_gl_driver() -> Option<u32> {
    for (index, item) in sdl2::render::drivers().enumerate() {
        sdl2::log::log(item.name);
        if item.name.contains("opengl") {
            sdl2::log::log("Driver chosen follows");
            sdl2::log::log(item.name);
            return Some(index as u32);
        }
    }
    sdl2::log::log("No OpenGL driver chosen");
    None
}

// Vertex data
static VERTEX_DATA: [GLfloat; 6] = [0.0, 0.5, 0.5, -0.5, -0.5, -0.5];

// Shader sources
#[cfg(any(target_os="android", target_os="ios"))]
fn precision() -> String {
    String::from("precision mediump float;\n")
}

#[cfg(not(any(target_os="android", target_os="ios")))]
fn precision() -> String {
    String::from("")
}

static VS_SRC: &'static str = "\n\
    attribute vec2 position;\n\
    varying vec4 color;\n\
    void main() {\n\
       gl_Position = vec4(position, 0.0, 1.0);\n\
       color = vec4(1.0, 1.0, 1.0, 1.0);\n\
       gl_PointSize = 1.0;\n\
    }";

static FS_SRC: &'static str = "\n\
    varying vec4 color;\n\
    void main() {\n\
       gl_FragColor = color;\n\
    }";

pub fn foo() {
    
}

fn compile_shader(src: &str, ty: GLenum) -> GLuint {
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

fn link_program(vs: GLuint, fs: GLuint) -> GLuint {
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

#[cfg(feature = "hotload")]
fn plugin_load<'a>() -> (Plugins, DynamicReload<'a>) {
    // Setup the reload handler. A temporary directory will be created inside the target/debug
    // where plugins will be loaded from. That is because on some OS:es loading a shared lib
    // will lock the file so we can't overwrite it so this works around that issue.
    let mut reload_handler = DynamicReload::new(Some(vec!["target/debug"]),
                                                Some("target/debug"),
                                                Search::Default);
    let mut plugs = Plugins { plugins: Vec::new() };

    // test_shared is generated in build.rs
    match reload_handler.add_library("test_shared", PlatformName::Yes) {
        Ok(lib) => plugs.add_plugin(&lib),
        Err(e) => {
            println!("Unable to load dynamic lib, err {:?}", e);
            return (plugs, reload_handler);
        }
    }

    (plugs, reload_handler)
}

#[cfg(not(feature = "hotload"))]
fn plugin_load() -> (i32, i32) {
    (0, 0)
}

#[cfg(feature = "hotload")]
fn plugin_update<'a>(mut plugs: &mut Plugins, mut reload_handler: &mut DynamicReload<'a>) {
        reload_handler.update(Plugins::reload_callback, &mut plugs);

        if plugs.plugins.len() > 0 {
            // In a real program you want to cache the symbol and not do it every time if your
            // application is performance critical
            let fun: Symbol<extern "C" fn() -> i32> =
                unsafe { plugs.plugins[0].lib.get(b"shared_fun\0").unwrap() };

            println!("Value {}", fun());
        }
}

#[cfg(not(feature = "hotload"))]
fn plugin_update(mut plugs: &mut i32, mut reload_handler: &mut i32) {
    println!("Value {}", shared_fun());
}


//#[cfg(feature = "hotload")]
pub fn run_loop() {
    let (mut plugs, mut reload_handler) = plugin_load();

    // Init SDL2
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rust-sdl2 demo: Video", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    sdl2::log::log("Looking for OpenGL drivers");
    let mut canvas = window
    .into_canvas()
    .index(find_sdl_gl_driver().unwrap())
    .build()
    .unwrap();

    sdl2::log::log("Loading GL extensions");
    gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const _);
    sdl2::log::log("Setting current GL context");
    canvas.window().gl_set_context_to_current();

    // Create GLSL shaders
    /*
    sdl2::log::log("Compiling vertex shader");
    let vs_src = precision()+VS_SRC;
    let vs = compile_shader(&vs_src, gl::VERTEX_SHADER);
    sdl2::log::log("Compiling fragment shader");
    let fs_src = precision()+FS_SRC;
    let fs = compile_shader(&fs_src, gl::FRAGMENT_SHADER);
    sdl2::log::log("Linking shaders");
    let program = link_program(vs, fs);

    let mut vao = 0;
    let mut vbo = 0;
*/
    unsafe {
        /*
        // Use shader program
        gl::UseProgram(program);

        //let col_attr = gl::GetAttribLocation(program, CString::new("color").unwrap().as_ptr());

        // Specify the layout of the vertex data
        let pos_attr = gl::GetAttribLocation(program, CString::new("position").unwrap().as_ptr());
        gl::EnableVertexAttribArray(pos_attr as GLuint);
        gl::VertexAttribPointer(pos_attr as GLuint,
                                2,
                                gl::FLOAT,
                                gl::FALSE as GLboolean,
                                0,
                                mem::transmute(&VERTEX_DATA[0]));
        */
    }

    let mut event_pump = sdl_context.event_pump().unwrap();

    /*
    // test_shared is generated in build.rs
    match reload_handler.add_library("test_shared", PlatformName::Yes) {
        Ok(lib) => plugs.add_plugin(&lib),
        Err(e) => {
            println!("Unable to load dynamic lib, err {:?}", e);
            return;
        }
    }
    */

    let texture_creator = canvas.texture_creator();
    let mut tm = TextureManager::new(&texture_creator);
    tm.load(String::from("wabbit"), Path::new("assets/wabbit_alpha.png"));
    let wabbit = tm.get(&String::from("wabbit"));

    //
    // While this is running (printing a number) change return value in file src/test_shared.rs
    // build the project with cargo build and notice that this code will now return the new value
    //
    'running: loop {
        plugin_update(&mut plugs, &mut reload_handler);

        /*
        reload_handler.update(Plugins::reload_callback, &mut plugs);

        if plugs.plugins.len() > 0 {
            // In a real program you want to cache the symbol and not do it every time if your
            // application is performance critical
            let fun: Symbol<extern "C" fn() -> i32> =
                unsafe { plugs.plugins[0].lib.get(b"shared_fun\0").unwrap() };

            println!("Value {}", fun());
        }
        */

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                _ => {}
            }
        }

        canvas.set_draw_color(SdlColor::RGB(255, 0, 0));
        canvas.clear();

        /*
        sdl2::log::log("Drawing triangle");
        unsafe {
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
        */

        {
            let mut shader = Shader::new();
            shader.load_default();
            let mut sb = SpriteBatch::new(&canvas);
            let position = Vector2::new(0.0, 0.0);
            let matrix: Matrix4<f32> = Matrix4::one();
            sdl2::log::log(&wabbit.get_height().to_string());
            sdl2::log::log(&wabbit.get_width().to_string());
            sb.begin(SpriteSortMode::SpriteSortModeDeferred, Some(&shader), Some(matrix));
            sb.draw(wabbit.clone(), Some(position), None, None, None, 0.0, None, Color::white(), 0.0);
            sb.end();
        }


        canvas.present();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        // The rest of the game loop goes here...


        // Wait for 0.5 sec
        thread::sleep(Duration::from_millis(500));
    }

     // Cleanup
    unsafe {
        /*
        gl::DeleteProgram(program);
        gl::DeleteShader(fs);
        gl::DeleteShader(vs);
        gl::DeleteBuffers(1, &vbo);
        */
    }
}
