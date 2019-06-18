#[cfg(feature = "hotload")]
extern crate dynamic_reload;

extern crate sdl2;
extern crate cgmath;
extern crate rand;
//extern crate imgui;

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
use std::path::Path;
//use sdl2::image::{LoadTexture, INIT_PNG, INIT_JPG};
use sdl2::pixels::Color as SdlColor;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::sys;
use rand::Rng;
//use imgui::*;
//use time;

#[cfg(not(feature = "hotload"))]
use test_shared::shared_fun;

use spritebatch::SpriteBatch;
use spritebatch::SpriteSortMode;
use color::Color;
use texturemanager::TextureManager;
use shader::Shader;
use camera::Camera;
use viewportadapter::ScalingViewportAdapter;
use viewportadapter::ViewportAdapterTrait;
use scene::Scene;
use imagecomponent::ImageComponent;
use log::Log;
use everythingrenderer::EverythingRenderer;
use debugnamecomponentmanager::DebugNameComponentManager;
use rectangle::Rectangle;
use timer;
use timer::Timer;
use texture::Texture;
use transformcomponent::TransformComponent;
use self::cgmath::Vector2;
use self::cgmath::Matrix4;
use self::cgmath::One;

pub mod gl {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}
use self::gl::types::*;

#[cfg(feature = "hotload")]
pub struct Plugins {
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
    }
    for (index, item) in sdl2::render::drivers().enumerate() {
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
        Err(_e) => {
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

pub struct MainLoopContext {
    running: bool,
    current_frame_delta: u64,
    frame_delay: u64,
    //canvas: sdl2::render::Canvas<sdl2::video::Window>,
    framerate_timer: Timer,
    event_pump: sdl2::EventPump,
    last_time: u64,
    sb: SpriteBatch,
    debug_name_manager: DebugNameComponentManager,
    framerate: u64,
    shader: Shader,
    bunnies: Vec<Bunny>,
    wabbit: std::rc::Rc<Texture>,
    scene: Scene,
    window: sdl2::video::Window,
    gl_context: sdl2::video::GLContext,
}

impl MainLoopContext {
    pub fn event_pump(&mut self) -> &mut sdl2::EventPump {
        &mut self.event_pump
    }

    pub fn set_running(&mut self, value: bool) {
        self.running = value;
    }
}

pub struct Engine {
    main_loop_context: Option<MainLoopContext>,
}

impl Engine {
    pub fn new() -> Self {
        Engine {
            main_loop_context: None,
        }
    }

    #[cfg(any(target_os="android", target_os="ios"))]
    fn assets_path(&self) -> String {
        String::from("assets/")
    }

    #[cfg(not(any(target_os="android", target_os="ios")))]
    fn assets_path(&self) -> String {
        String::from("assets/")
    }

    pub fn set_running(&mut self, value: bool) {
        match &mut self.main_loop_context {
            Some(main_loop_context) => {
                main_loop_context.set_running(value);
            }, 
            None => {}
        }
    }

    pub fn main_loop(&mut self) {
        let main_loop_context = &mut self.main_loop_context;
        match main_loop_context {
            Some(main_loop_context) => {
                let sh = main_loop_context.shader.clone();

                if main_loop_context.framerate == 0 {
                    Log::error("framerate zero");
                    return;
                }

                //#[cfg(not(target_arch = "wasm32"))]
                // Uncomment the following line to re-enable dynamic loading of code. You will have to set up lifetimes correctly, though
                //plugin_update(&mut plugs, &mut reload_handler);

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

                let mut running = true;
                for event in main_loop_context.event_pump.poll_iter() {
                    match event {
                        Event::Quit { .. } |
                        Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                            running = false;
                            Log::info("Quit requested");
                        },
                        _ => {}
                    }
                }
                main_loop_context.set_running(running);
                //main_loop_context.canvas.set_draw_color(SdlColor::RGB(191, 255, 255));
                //main_loop_context.canvas.clear();
                unsafe {
                    gl::ClearColor(191.0/255.0, 255.0/255.0, 255.0/255.0, 1.0);
                    gl::Clear(gl::COLOR_BUFFER_BIT);
                }

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
                
                let current_time = timer::precise_time_ns();
                let delta_time = ((current_time - main_loop_context.last_time) as f64) / 1_000_000_000.0;
                main_loop_context.last_time = current_time;
                {
                    let position = Vector2::new(0.0, 0.0);
                    let matrix: Matrix4<f32> = Matrix4::one();
                    //sdl2::log::log("wabbit width and height follows");
                    //sdl2::log::log(&wabbit.get_height().to_string());
                    //sdl2::log::log(&wabbit.get_width().to_string());

                    let viewport = Rectangle::new(0.0, 0.0, 800, 600);
                    main_loop_context.sb.begin(viewport, SpriteSortMode::SpriteSortModeDeferred, Some(main_loop_context.shader), Some(matrix));
                    for bunny in main_loop_context.bunnies.iter_mut() {
                        bunny.update(delta_time);
                        main_loop_context.sb.draw(main_loop_context.wabbit.clone(), Some(bunny.position), None, None, None, 0.0, None, Color::white(), 0.0);
                    }
                    let e = 0;
                    let ic_compo = main_loop_context.scene.get_component::<ImageComponent>(e);
                    let tc_compo = main_loop_context.scene.get_component::<TransformComponent>(e);
                    let ic = ic_compo.unwrap();
                    let tc = tc_compo.unwrap();
                    let tex = ic.get_texture().unwrap();
                    main_loop_context.sb.draw(tex, Some(tc.get_position()), None, None, None, 0.0, None, Color::white(), 0.0);
                    main_loop_context.sb.end(viewport);
                }

                main_loop_context.debug_name_manager.update(0.0);
                main_loop_context.scene.render_entities();

                //main_loop_context.canvas.present();
                main_loop_context.window.gl_swap_window();

                //::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
                // The rest of the game loop goes here...


                // Wait for 0.5 sec
                //thread::sleep(Duration::from_millis(500));
                // Replace with the following once we're done with testing
                //thread::sleep(Duration::from_millis(0))


                // How many nanoseconds the last frame took
                main_loop_context.current_frame_delta = main_loop_context.framerate_timer.delta();

                main_loop_context.frame_delay = 1_000_000_000 / main_loop_context.framerate;
                if main_loop_context.frame_delay < main_loop_context.current_frame_delta {
                    main_loop_context.frame_delay = 0;
                } else {
                    main_loop_context.frame_delay = main_loop_context.frame_delay - main_loop_context.current_frame_delta;
                }

                Log::info("before sleep");
                thread::sleep(Duration::from_millis((main_loop_context.frame_delay / 1_000_000) as u64));
                Log::info("after sleep");

                if main_loop_context.current_frame_delta < main_loop_context.frame_delay {
                    /*
                    unsafe {
                        sdl2::sys::timer::SDL_Delay((frame_delay) - current_frame_delta);
                        thread::sleep(Duration::from_millis(((frame_delay - current_frame_delta) / 1_000_000) as u64))
                    }
                    */
                }

                /*
                let f = (1_000_000_000.0 / framerate_timer.delta() as f64); // frames per second
                println!("FPS: {}",  &f.to_string());
                */

                //thread::yield_now();
                main_loop_context.framerate_timer.restart();
            },
            None => {
                Log::error("main_loop(): Context shouldn't be None");
            }
        }

    }

    pub fn get_is_running(context: &Option<MainLoopContext>) -> bool {
        match context {
            Some(context) => {
                return context.running;
            },
            None => {
                return false
            }
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn run_main_loop<'a>(&mut self, mut plugs: &mut Plugins, mut reload_handler: &mut DynamicReload<'a>) {
        while Self::get_is_running(&self.main_loop_context) {
            plugin_update(&mut plugs, &mut reload_handler);
            self.main_loop();
        }
    }

    //#[cfg(feature = "hotload")]
    pub fn run_loop(&mut self) {
        let (mut plugs, mut reload_handler) = plugin_load();

        // Init SDL2
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        #[cfg(any(target_os="ios", target_os="android", target_arch="wasm32"))]
        {
            video_subsystem.gl_attr().set_context_profile(sdl2::video::GLProfile::GLES);
            video_subsystem.gl_attr().set_context_major_version(2);
            video_subsystem.gl_attr().set_context_minor_version(0);
        }

        #[cfg(not(any(target_os="android", target_os="ios", target_arch="wasm32")))]
        {
            video_subsystem.gl_attr().set_context_profile(sdl2::video::GLProfile::Compatibility);
        }

        let window = video_subsystem
            .window("minigame-rust", 800, 600)
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        sdl2::log::log("Looking for OpenGL drivers");
        /*let mut canvas = window
        .into_canvas()
        .index(find_sdl_gl_driver().unwrap())
        .build()
        .unwrap();*/

        let gl_context = window.gl_create_context().unwrap();
        window.gl_make_current(&gl_context).unwrap();

        sdl2::log::log("Loading GL extensions");
        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const _);
        /*
        sdl2::log::log("Setting current GL context");
        let gl_set_context_res = canvas.window().gl_set_context_to_current();
        match gl_set_context_res {
            Ok(_context) => {
            },
            Err(_error) => {
                panic!("Cannot set current GL context");
            }
        }
        */
        
        #[cfg(not(target_arch = "wasm32"))]
        {
            Log::info("Enabling VSYNC");
            video_subsystem.gl_set_swap_interval(1);
        }


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
        /*
        unsafe {
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
        }
        */

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

        let mut tm = TextureManager::new();
        let wabbit_path = [self.assets_path(), String::from("wabbit_alpha.png")].concat();
        #[cfg(target_arch = "wasm32")]
        {
            let img_data = include_bytes!("../assets/wabbit_alpha.png");
            tm.load_from_memory(String::from("wabbit"), &img_data[..]);
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            tm.load(String::from("wabbit"), Path::new(&String::from(wabbit_path)));
        }
        let wabbit = tm.get(&String::from("wabbit"));

        let player_va = ScalingViewportAdapter::with_size_and_virtual(800, 600, 320, 240);
        let mut player_camera = Camera::new();
        player_camera.set_viewport_adapter(Some(player_va));

        let mut debug_name_manager = DebugNameComponentManager::new();

        let mut scene = Scene::new(32);
        scene.register_component::<ImageComponent>();
        scene.register_component::<TransformComponent>();
        let entity_id = scene.create_entity();
        sdl2::log::log("Entity id follows");
        sdl2::log::log(&entity_id.to_string());
        let debug_name_instance = debug_name_manager.create(entity_id);
        Log::debug("Debug name instance");
        Log::debug(&debug_name_instance.i.to_string());
        debug_name_manager.set_name(debug_name_instance, String::from("entity1"));
        {
            let mut ic = ImageComponent::new();//with_texture(tm.get(&String::from("wabbit")));
            ic.texture = Some(tm.get(&String::from("wabbit")));
            scene.add_component_to_entity(entity_id, ic);

            let mut tc = TransformComponent::new();
            tc.set_position(50.0, 50.0);
            scene.add_component_to_entity(entity_id, tc);
        }

        let er = EverythingRenderer::new();
        scene.add_renderer(Rc::new(er));

        let mut rng = rand::thread_rng();


        let mut bunnies = vec![Bunny::new(); 10];
        for bunny in bunnies.iter_mut() {
            bunny.speed.x = rng.gen::<f64>() * 500.0;
            bunny.speed.x = (rng.gen::<f64>() * 500.0) - 250.0;
        }

        let mut sb = SpriteBatch::new();

        //let mut imgui = ImGui::init();
        //let ui = imgui.frame((800, 600), (800, 600), 0.0);
        //imgui.set_texture_id(0);

        let mut framerate_timer = Timer::new();
        framerate_timer.start();
        // How many frames per second we're running.
        let framerate: u64 = 60;

        // Time (in nanoseconds) each frame must have.
        //
        // If the actual delay passed is less than this,
        // we'll wait.
        // If the delay is greater, we'll skip right away.
        //
        let mut frame_delay: u64 = 0;

        // How much time have passed since
        // last frame (in nanoseconds).
        //
        let mut current_frame_delta: u64 = 0;

        // Last time (in nanoseconds)
        let mut last_time = timer::precise_time_ns();

        let mut running = true;

        self.main_loop_context = Some(MainLoopContext {
            running: running,
            current_frame_delta: current_frame_delta,
            frame_delay: frame_delay,
            //canvas: canvas,
            framerate_timer: framerate_timer,
            event_pump: event_pump,
            last_time: last_time,
            sb: sb,
            debug_name_manager: debug_name_manager,
            framerate: framerate,
            shader: shader,
            bunnies: bunnies,
            wabbit: wabbit,
            scene: scene,
            window: window,
            gl_context: gl_context,
        });

        //
        // While this is running (printing a number) change return value in file src/test_shared.rs
        // build the project with cargo build and notice that this code will now return the new value
        //
        #[cfg(not(target_arch = "wasm32"))]
        {
            self.run_main_loop(&mut plugs, &mut reload_handler);
        }

        // Cleanup
        /*
        unsafe {
            gl::DeleteProgram(program);
            gl::DeleteShader(fs);
            gl::DeleteShader(vs);
            gl::DeleteBuffers(1, &vbo);
        }
        */
    }
}

#[derive(Clone, Copy, Debug)]
struct Bunny {
    position: Vector2<f32>,
    sub_position: Vector2<f32>,
    speed: Vector2<f64>,
    min: Vector2<f32>,
    max: Vector2<f32>,
    gravity: f64,
}

impl Bunny {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        Bunny {
            position: Vector2::new(0.0, 0.0),
            sub_position: Vector2::new(0.0, 0.0),
            speed: Vector2::new(rng.gen::<f64>() * 500.0, (rng.gen::<f64>() * 500.0) - 250.0),
            min: Vector2::new(0.0, 0.0),
            max: Vector2::new(800.0, 600.0),
            gravity: 0.5 * 100.0,
        }
    }

    pub fn update(&mut self, dt: f64) {
        let mut rng = rand::thread_rng();
        self.position.x += (self.speed.x * dt) as f32;
        self.position.y += (self.speed.y * dt) as f32;
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

            if rng.gen::<f64>() > 0.5 {
                self.speed.y -= 3.0 + rng.gen::<f64>() * 4.0;
            }
        } else if self.position.y < self.min.y {
            self.speed.y = 0.0;
            self.position.y = self.min.y;
        }

    }
}