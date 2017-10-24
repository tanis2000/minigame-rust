#[cfg(feature = "hotload")]
extern crate dynamic_reload;

extern crate sdl2;
extern crate cgmath;
extern crate rand;
extern crate imgui;

//#[cfg(not(feature = "hotload"))]
//extern crate minigame;

#[cfg(feature = "hotload")]
use dynamic_reload::{DynamicReload, Lib, Symbol, Search, PlatformName, UpdateState};
use std::rc::Rc;
use std::time::Duration;
use std::thread;
use std::str;
use std::ffi::CString;
use std::ptr;
use std::mem;
use std::path::Path;
use std::cell::RefCell;
use std::ops::Deref;
//use sdl2::image::{LoadTexture, INIT_PNG, INIT_JPG};
use sdl2::pixels::Color as SdlColor;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::TextureCreator;
use sdl2::render::Texture as SdlTexture;
use sdl2::video::WindowContext;
use rand::Rng;
use imgui::*;

#[cfg(not(feature = "hotload"))]
use test_shared::shared_fun;

use spritebatch::SpriteBatch;
use spritebatch::SpriteSortMode;
use color::Color;
use texture::Texture;
use texturemanager::TextureManager;
use shader::Shader;
use camera::Camera;
use viewportadapter::ScalingViewportAdapter;
use viewportadapter::ViewportAdapterTrait;
use scene::Scene;
use imagecomponent::ImageComponent;
use log::Log;
use everythingrenderer::EverythingRenderer;
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
            UpdateState::ReloadFailed(_) => Log::error("Failed to reload"),
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
            //Log::error("Unable to load dynamic lib, err {:?}", e);
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

            //println!("Value {}", fun());
        }
}

#[cfg(not(feature = "hotload"))]
fn plugin_update(mut plugs: &mut i32, mut reload_handler: &mut i32) {
    //println!("Value {}", shared_fun());
}

pub struct Engine {
}

impl Engine {
    pub fn new() -> Self {
        Engine {
        }
    }

    #[cfg(any(target_os="android", target_os="ios"))]
    fn assets_path(&self) -> String {
        String::from("")
    }

    #[cfg(not(any(target_os="android", target_os="ios")))]
    fn assets_path(&self) -> String {
        String::from("assets/")
    }


    //#[cfg(feature = "hotload")]
    pub fn run_loop(&mut self) {
        let (mut plugs, mut reload_handler) = plugin_load();

        // Init SDL2
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window("minigame-rust", 800, 600)
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
                Log::error("Unable to load dynamic lib, err {:?}", e);
                return;
            }
        }
        */

        let mut shader = Shader::new();
        shader.load_default();

        let mut tc = canvas.texture_creator();
        let mut tm = TextureManager::new(&tc);
        let mut wabbit_path = [self.assets_path(), String::from("wabbit_alpha.png")].concat();
        tm.load(String::from("wabbit"), Path::new(&String::from(wabbit_path)));
        let wabbit = tm.get(&String::from("wabbit"));

        let playerVA = ScalingViewportAdapter::with_size_and_virtual(800, 600, 320, 240);
        let mut playerCamera = Camera::new();
        playerCamera.set_viewport_adapter(Some(playerVA));

        let mut scene = Scene::new(32);
        let mut entity_id = scene.create_entity();
        sdl2::log::log("Entity id follows");
        sdl2::log::log(&entity_id.to_string());
        {
            let mut e = scene.get_entity_mut(entity_id);
            //assert!(Rc::make_mut(&mut e).is_some());
            //let tm = self.texture_manager.as_ref().unwrap();
            let mut ic = ImageComponent::new();//with_texture(tm.get(&String::from("wabbit")));
            ic.texture = Some(tm.get(&String::from("wabbit")));
            match e {
                Some(entity) => {
                    entity.add(Rc::new(ic));
                },
                None => {
                    sdl2::log::log("Something went wrong with the entity")
                },
            }
        }

        let mut er = EverythingRenderer::new();
        scene.add_renderer(Rc::new(er));

        let mut rng = rand::thread_rng();


        let mut bunnies = [Bunny::new(); 1];
        for bunny in bunnies.iter_mut() {
            bunny.speed.x = rng.gen::<f32>() * 5.0;
            bunny.speed.x = (rng.gen::<f32>() * 5.0) - 2.5;
        }

        let mut sb = SpriteBatch::new();

        let mut imgui = ImGui::init();
        //let ui = imgui.frame((800, 600), (800, 600), 0.0);
        //imgui.set_texture_id(0);

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

                Log::info("Value {}", fun());
            }
            */

            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                    _ => {}
                }
            }

            canvas.set_draw_color(SdlColor::RGB(191, 255, 255));
            canvas.clear();

            /*
            sdl2::log::log("Drawing triangle");
            unsafe {
                gl::DrawArrays(gl::TRIANGLES, 0, 3);
            }
            */

            /*
            ui.window(im_str!("Hello world"))
                .size((300.0, 100.0), ImGuiSetCond_FirstUseEver)
                .build(|| {
                    ui.text(im_str!("Hello world!"));
                    ui.text(im_str!("This...is...imgui-rs!"));
                    ui.separator();
                    let mouse_pos = ui.imgui().mouse_pos();
                    ui.text(im_str!("Mouse Position: ({:.1},{:.1})", mouse_pos.0, mouse_pos.1));
                });
            */
            
            {
                let position = Vector2::new(0.0, 0.0);
                let matrix: Matrix4<f32> = Matrix4::one();
                //sdl2::log::log("wabbit width and height follows");
                //sdl2::log::log(&wabbit.get_height().to_string());
                //sdl2::log::log(&wabbit.get_width().to_string());

                sb.begin(&mut canvas, SpriteSortMode::SpriteSortModeDeferred, Some(shader), Some(matrix));
                for bunny in bunnies.iter_mut() {
                    bunny.update();
                    sb.draw(wabbit.clone(), Some(bunny.position), None, None, None, 0.0, None, Color::white(), 0.0);
                }
                sb.end(&mut canvas);
            }

            scene.render_entities();

            canvas.present();

            //::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
            // The rest of the game loop goes here...


            // Wait for 0.5 sec
            //thread::sleep(Duration::from_millis(500));
            // Replace with the following once we're done with testing
            //thread::sleep(Duration::from_millis(0))
            thread::yield_now();
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
}

#[derive(Clone, Copy, Debug)]
struct Bunny {
    position: Vector2<f32>,
    speed: Vector2<f32>,
    min: Vector2<f32>,
    max: Vector2<f32>,
    gravity: f32,
}

impl Bunny {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        Bunny {
            position: Vector2::new(0.0, 0.0),
            speed: Vector2::new(rng.gen::<f32>() * 5.0, (rng.gen::<f32>() * 5.0) - 2.5),
            min: Vector2::new(0.0, 0.0),
            max: Vector2::new(800.0, 600.0),
            gravity: 0.5,
        }
    }

    pub fn update(&mut self) {
        let mut rng = rand::thread_rng();
        self.position.x += self.speed.x;
        self.position.y += self.speed.y;
        self.speed.y += self.gravity;

        if self.position.x > self.max.x {
            self.speed.x *= -1.0;
            self.position.x = self.max.x;
        } else if self.position.x < self.min.x {
            self.speed.x *= -1.0;
            self.position.x = self.min.x;
        }

        if self.position.y > self.max.y {
            self.speed.y *= -0.8;
            self.position.y = self.max.y;

            if rng.gen::<f32>() > 0.5 {
                self.speed.y -= 3.0 + rng.gen::<f32>() * 4.0;
            }
        } else if self.position.y < self.min.y {
            self.speed.y = 0.0;
            self.position.y = self.min.y;
        }

    }
}