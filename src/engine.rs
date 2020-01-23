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
use std::ops::Mul;
use std::any::{Any, TypeId};
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
use crate::atlas::texturepacker;
use spritecomponent::{SpriteComponent, SpriteFrame};
use subtexture::Subtexture;
use tile::layercomponent::LayerComponent;
use render_target::RenderTarget;
use utils;
use graphicsdevice::GraphicsDevice;
use world::{BaseSystem, System, SystemData};
use entity::Entity;
use self::cgmath::{Vector2, Vector3, Matrix, Matrix4, One};

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
    for (_index, item) in sdl2::render::drivers().enumerate() {
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

fn build_scaling_viewport(window_width: u32, window_height: u32,
                            design_width: u32, design_height: u32,
                            viewport: &mut Rectangle, inverse_multiplier: &mut f32, scale_matrix: &mut Matrix4<f32>) {
  let mut multiplier: u32 = 1;
  let scale_x: f32 = window_width as f32 / design_width as f32;
  let scale_y: f32 = window_height as f32 / design_height as f32;

  // find the multiplier that fits both the new width and height
  let max_scale: u32;
  if (scale_x as u32) < (scale_y as u32) {
      max_scale = scale_x as u32;
  } else {
      max_scale = scale_y as u32;
  }
  if max_scale > multiplier {
    multiplier = max_scale;
  }

  // viewport origin translation
  let diff_x: f32 =
    (window_width as f32 / 2.0) - (design_width as f32 * multiplier as f32 / 2.0);
  let diff_y: f32 =
    (window_height as f32 / 2.0) - (design_height as f32 * multiplier as f32 / 2.0);

  // build the new viewport
  viewport.x = diff_x;
  viewport.y = diff_y;
  viewport.w = (design_width * multiplier) as i32;
  viewport.h = (design_height * multiplier) as i32;
  *inverse_multiplier = 1.0 / multiplier as f32;

  // compute the scaling matrix
  let mat_mul_x: f32 = (viewport.w as f32 - viewport.x)/design_width as f32;
  let mat_mul_y: f32 = (viewport.h as f32 - viewport.y)/design_height as f32;

  let identity = Matrix4::one();
  scale_matrix.x = identity.x;
  scale_matrix.y = identity.y;
  scale_matrix.z = identity.z;
  scale_matrix.w = identity.w;
  let trans_vector: Vector3<f32> = Vector3::new(diff_x, diff_y, 0.0);
  let trans_matrix = Matrix4::from_translation(trans_vector);
  let sc_matrix: Matrix4<f32> = Matrix4::from_nonuniform_scale(mat_mul_x, mat_mul_y, 1.0);
  let out_matrix = Matrix4::mul(trans_matrix, sc_matrix);
  scale_matrix.x = out_matrix.x;
  scale_matrix.y = out_matrix.y;
  scale_matrix.z = out_matrix.z;
  scale_matrix.w = out_matrix.w;
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
    window: sdl2::video::Window,
    gl_context: sdl2::video::GLContext,
    camera: Camera<ScalingViewportAdapter>,
    screen_render_target: RenderTarget,
    quad_shader: Shader,
}

impl MainLoopContext {
    pub fn event_pump(&mut self) -> &mut sdl2::EventPump {
        &mut self.event_pump
    }

    pub fn set_running(&mut self, value: bool) {
        self.running = value;
    }

    pub fn get_sb(&self) -> &SpriteBatch {
        &self.sb
    }

    pub fn get_sb_as_mut(&mut self) -> &mut SpriteBatch {
        &mut self.sb
    }
}

/*
impl SystemData for MainLoopContext {
    fn get_context<T>(&self) -> T {
        return self;
    }
}
*/

pub struct Engine {
    main_loop_context: Option<MainLoopContext>,
    scene: Option<Scene<MainLoopContext>>,
}

impl Engine {
    pub fn new() -> Self {
        Engine {
            main_loop_context: None,
            scene: None,
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
        let ref mut main_loop_context = self.main_loop_context;
        //let main_loop_context = &mut self.main_loop_context;
        match main_loop_context {
            Some(main_loop_context) => {
                match &mut self.scene {
                    Some(scene) => {
                        let sh = main_loop_context.shader.clone();

                        if main_loop_context.framerate == 0 {
                            Log::error("framerate zero, is this the first frame?");
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
                                Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                                    Log::info("Space pressed");
                                    scene.destroy_entity(1);
                                }
                                _ => {}
                            }
                        }
                        main_loop_context.set_running(running);
                        //main_loop_context.canvas.set_draw_color(SdlColor::RGB(191, 255, 255));
                        //main_loop_context.canvas.clear();
        
                        // Set the main render target
                        GraphicsDevice::set_render_target(&main_loop_context.screen_render_target);
                        let design_viewport = Rectangle::new(0.0, 0.0, 320, 240);
                        GraphicsDevice::apply_viewport(&design_viewport);
        
                        // clear the screen
                        unsafe {
                            gl::ClearColor(191.0/255.0, 255.0/255.0, 255.0/255.0, 1.0);
                            gl::Clear(gl::COLOR_BUFFER_BIT);
                        }
        
                        // apply the default shader
                        main_loop_context.sb.get_graphics_device_mut().apply_shader(&main_loop_context.shader);
        
        
        
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
        
                            let viewport = Rectangle::new(0.0, 0.0, 320, 240);
                            //let viewport = Rectangle::new(0.0, 0.0, 800, 600);
                            // TODO: fix the scaling viewport
                            //let viewport = main_loop_context.camera.get_viewport_adapter().unwrap().get_viewport();
                            //println!("{:?}", viewport);
                            let camera_matrix = main_loop_context.camera.get_transform_matrix();
                            println!("{:?}", camera_matrix);
                            main_loop_context.sb.begin(viewport, SpriteSortMode::SpriteSortModeDeferred, Some(main_loop_context.shader), Some(camera_matrix));
                            {
                                let e = 4;
                                let ic_compo = scene.get_component::<ImageComponent>(e);
                                let tc_compo = scene.get_component::<TransformComponent>(e);
                                let ic = ic_compo.unwrap();
                                let tc = tc_compo.unwrap();
                                let tex = ic.get_texture().unwrap();
                                let mut scale = Vector2::new(3.0, 3.0);
                                main_loop_context.sb.draw(tex, Some(*tc.get_position()), None, None, None, 0.0, Some(scale), Color::white(), 0.0);
                            }
        
                            for bunny in main_loop_context.bunnies.iter_mut() {
                                bunny.update(delta_time);
                                main_loop_context.sb.draw(main_loop_context.wabbit.clone(), Some(bunny.position), None, None, None, 0.0, None, Color::white(), 0.0);
                            }
        
                            {
                                let e = 0;
                                let ic_compo = scene.get_component::<ImageComponent>(e);
                                let tc_compo = scene.get_component::<TransformComponent>(e);
                                let ic = ic_compo.unwrap();
                                let tc = tc_compo.unwrap();
                                let tex = ic.get_texture().unwrap();
                                main_loop_context.sb.draw(tex, Some(*tc.get_position()), None, None, None, 0.0, None, Color::white(), 0.0);
                            }
                            {
                                let e = 1;
                                let ic_compo = scene.get_component::<ImageComponent>(e);
                                let tc_compo = scene.get_component::<TransformComponent>(e);
                                let ic = ic_compo.unwrap();
                                let tc = tc_compo.unwrap();
                                let tex = ic.get_texture().unwrap();
                                main_loop_context.sb.draw(tex, Some(*tc.get_position()), None, None, None, 0.0, None, Color::white(), 0.0);
                            }
                            /*
                            {
                                let e = 2;
                                let sc_compo = main_loop_context.scene.get_component::<SpriteComponent>(e);
                                let tc_compo = main_loop_context.scene.get_component::<TransformComponent>(e);
                                let sc = sc_compo.unwrap();
                                let tc = tc_compo.unwrap();
                                let current_frame = sc.get_current_frame();
                                let frame = sc.get_frame(current_frame).unwrap();
                                let tex = frame.get_texture().unwrap();
                                let clip_rect = sc.get_source_rect();
                                main_loop_context.sb.draw(tex, Some(*tc.get_position()), None, Some(*clip_rect), None, 0.0, None, Color::white(), 0.0);
                            }
                            */
                            {
                                let e = 3;
                                let lc_compo = scene.get_component::<LayerComponent>(e);
                                let tc_compo = scene.get_component::<TransformComponent>(e);
                                let lc = lc_compo.unwrap();
                                let tc = tc_compo.unwrap();
                                let tex = lc.get_texture().unwrap();
                                let layer = lc.get_layer().clone().unwrap();
                                match layer.layer_type {
                                    tiled_json_rs::LayerType::TileLayer(tiles) => {
                                        let mut count = 0;
                                        for tile in tiles.data {
                                            if tile != 0 && tile != 2684354639 {
                                                let gid = tile-1;
                                                let clip_rect = Rectangle::new((gid % 25 * 16) as f32, (gid / 25 * 16) as f32, 16, 16);
                                                //let clip_rect = Rectangle::new(16.0, 16.0, 16, 16);
                                                let mut pos = (*tc.get_position()).clone();
                                                pos.x = pos.x + (count % 58 * 16) as f32;
                                                pos.y = pos.y + (count / 58 * 16) as f32;
                                                //println!("{}", tile);
                                                main_loop_context.sb.draw(tex.clone(), Some(pos), None, Some(clip_rect), None, 0.0, None, Color::white(), 0.0);
                                            }
                                            count += 1;
                                        }
                                        //let clip_rect = sc.get_source_rect();
                                        //main_loop_context.sb.draw(tex, Some(tc.get_position()), None, Some(*clip_rect), None, 0.0, None, Color::white(), 0.0);
                                    }
                                    _ => {}
                                }
                            }
                            /*
                            {
                                let e = 5;
                                let sc_compo = main_loop_context.scene.get_component::<SpriteComponent>(e);
                                let tc_compo = main_loop_context.scene.get_component::<TransformComponent>(e);
                                let sc = sc_compo.unwrap();
                                let tc = tc_compo.unwrap();
                                let current_frame = sc.get_current_frame();
                                let frame = sc.get_frame(current_frame).unwrap();
                                let tex = frame.get_texture().unwrap();
                                let clip_rect = sc.get_source_rect();
                                main_loop_context.sb.draw(tex, Some(*tc.get_position()), None, Some(*clip_rect), None, 0.0, None, Color::white(), 0.0);
                            }
                            */
                            scene.process(delta_time as f32, main_loop_context);
                            main_loop_context.sb.end(viewport);
                        }
        
                        main_loop_context.debug_name_manager.update(0.0);
                        scene.render_entities();
        
                        main_loop_context.sb.get_graphics_device_mut().apply_shader(&main_loop_context.quad_shader);
                        let mut vp = Rectangle::new(0.0, 0.0, 0, 0);
                        let mut multiplier: f32 = 1.0;
                        let mut camera_transform_mat: Matrix4<f32> = Matrix4::one();
                        build_scaling_viewport(800, 600, 320, 240, &mut vp, &mut multiplier, &mut camera_transform_mat);
                        camera_transform_mat = Matrix4::one();
                        GraphicsDevice::apply_viewport(&vp);
                        GraphicsDevice::set_uniform_float2(&main_loop_context.quad_shader, "resolution", 320 as f32, 240 as f32);
                        GraphicsDevice::set_uniform_mat4(&main_loop_context.quad_shader, "transform", camera_transform_mat);
                        GraphicsDevice::set_uniform_float2(&main_loop_context.quad_shader, "scale", multiplier, multiplier);
                        GraphicsDevice::set_uniform_float2(&main_loop_context.quad_shader, "viewport", vp.x, vp.y);
                        GraphicsDevice::draw_quad_to_screen(&main_loop_context.quad_shader, &main_loop_context.screen_render_target);
        
        
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
        
                        //Log::info("before sleep");
                        thread::sleep(Duration::from_millis((main_loop_context.frame_delay / 1_000_000) as u64));
                        //Log::info("after sleep");
        
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
                    None => {}
                }
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
            match video_subsystem.gl_set_swap_interval(1) {
                Ok(_res) => {
                    Log::info("VSYNC enabled");
                },
                Err(_error) => {
                    Log::error("Cannot enable VSYNC");
                }
            }
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

        let mut quad_shader = Shader::new();
        let quad_shader_vert_path = [self.assets_path(), String::from("shaders/screen_vert.glsl")].concat();
        let quad_shader_vert_src = utils::load_string_from_file(Path::new(&String::from(quad_shader_vert_path))).unwrap();
        let quad_shader_frag_path = [self.assets_path(), String::from("shaders/screen_frag.glsl")].concat();
        let quad_shader_frag_src = utils::load_string_from_file(Path::new(&String::from(quad_shader_frag_path))).unwrap();
        quad_shader.compile(quad_shader_vert_src.as_str(), quad_shader_frag_src.as_str());

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

        let props_atlas_path = [self.assets_path(), String::from("atlas/atlas-props.json")].concat();
        let props_atlas = crate::atlas::texturepacker::load_atlas_from_file(props_atlas_path).unwrap();
        let props_atlas_texture_path = [self.assets_path(), String::from("atlas/atlas-props.png")].concat();
        tm.load(String::from("props"), Path::new(&String::from(props_atlas_texture_path)));

        let atlas_path = [self.assets_path(), String::from("atlas/atlas.json")].concat();
        let atlas = crate::atlas::texturepacker::load_atlas_from_file(atlas_path).unwrap();
        let atlas_texture_path = [self.assets_path(), String::from("atlas/atlas.png")].concat();
        tm.load(String::from("atlas"), Path::new(&String::from(atlas_texture_path)));

        let map_path = [self.assets_path(), String::from("maps/map.json")].concat();
        let map = crate::tile::tiled::load_map_from_file(map_path).unwrap();
        let map_texture_path = [self.assets_path(), String::from("environment/tileset.png")].concat();
        tm.load(String::from("tileset"), Path::new(&String::from(map_texture_path)));

        let back_texture_path = [self.assets_path(), String::from("environment/back.png")].concat();
        tm.load(String::from("back"), Path::new(&String::from(back_texture_path)));

        let screen_render_target = RenderTarget::new(320, 240, false, gl::RGBA);

        let player_va = ScalingViewportAdapter::with_size_and_virtual(800, 600, 320, 240);
        let mut player_camera = Camera::new();
        player_camera.set_viewport_adapter(Some(player_va));

        let mut debug_name_manager = DebugNameComponentManager::new();

        let mut scene = Scene::new(32);
        scene.register_component::<ImageComponent>();
        scene.register_component::<TransformComponent>();
        scene.register_component::<SpriteComponent>();
        scene.register_component::<LayerComponent>();

        let mut sprite_system = SpriteSystem::new();
        sprite_system.base.watch_component::<SpriteComponent>();
        scene.add_system(sprite_system);

        {
            let entity_id = scene.create_entity();
            sdl2::log::log("Entity id follows");
            sdl2::log::log(&entity_id.to_string());
            let debug_name_instance = debug_name_manager.create(entity_id);
            Log::debug("Debug name instance");
            Log::debug(&debug_name_instance.i.to_string());
            debug_name_manager.set_name(debug_name_instance, String::from("entity1"));

            let mut ic = ImageComponent::new();//with_texture(tm.get(&String::from("wabbit")));
            ic.texture = Some(tm.get(&String::from("wabbit")));
            scene.add_component_to_entity(entity_id, ic);

            let mut tc = TransformComponent::new();
            tc.set_position(50.0, 50.0);
            scene.add_component_to_entity(entity_id, tc);
        }
        {
            let entity_id = scene.create_entity();
            sdl2::log::log("Entity id follows");
            sdl2::log::log(&entity_id.to_string());
            let debug_name_instance = debug_name_manager.create(entity_id);
            Log::debug("Debug name instance");
            Log::debug(&debug_name_instance.i.to_string());
            debug_name_manager.set_name(debug_name_instance, String::from("entity2"));

            let mut ic = ImageComponent::new();
            ic.texture = Some(tm.get(&String::from("wabbit")));
            scene.add_component_to_entity(entity_id, ic);

            let mut tc = TransformComponent::new();
            tc.set_position(100.0, 50.0);
            scene.add_component_to_entity(entity_id, tc);
        }
        {
            let mut tx = 0.0;
            let mut ty = 0.0;
            let mut tx2 = 1.0;
            let mut ty2 = 1.0;
            let region = props_atlas
                .frames
                .iter()
                .filter(|frame| frame.filename == "block")
                .collect::<Vec<&crate::atlas::texturepacker::Frame>>()[0];
            let sw = props_atlas.meta.size.w as f32;
            let sh = props_atlas.meta.size.h as f32;
            /*
            tx = region.frame.x as f32 / sw;
            ty = region.frame.y as f32 / sh;
            tx2 = (region.frame.x as f32 + region.frame.w as f32) / sw;
            ty2 = (region.frame.y as f32 + region.frame.h as f32) / sh;
            */
            tx = region.frame.x as f32;
            ty = region.frame.y as f32;
            tx2 = region.frame.w as f32;
            ty2 = region.frame.h as f32;

            let entity_id = scene.create_entity();
            sdl2::log::log("Entity id follows");
            sdl2::log::log(&entity_id.to_string());
            let debug_name_instance = debug_name_manager.create(entity_id);
            Log::debug("Debug name instance");
            Log::debug(&debug_name_instance.i.to_string());
            debug_name_manager.set_name(debug_name_instance, String::from("entity3"));

            let mut sc = SpriteComponent::new();
            let sub = Subtexture::with_texture(Some(tm.get(&String::from("props"))), tx as i32, ty as i32, tx2 as i32, ty2 as i32);
            sc.add_frame_with_subtexture(sub);
            scene.add_component_to_entity(entity_id, sc);

            let mut tc = TransformComponent::new();
            tc.set_position(150.0, 50.0);
            scene.add_component_to_entity(entity_id, tc);

        }
        {
            let entity_id = scene.create_entity();
            sdl2::log::log("Entity id follows");
            sdl2::log::log(&entity_id.to_string());
            let debug_name_instance = debug_name_manager.create(entity_id);
            Log::debug("Debug name instance");
            Log::debug(&debug_name_instance.i.to_string());
            debug_name_manager.set_name(debug_name_instance, String::from("entity3"));

            let mut lc = LayerComponent::new();
            lc.set_texture(Some(tm.get(&String::from("tileset"))));
            lc.set_layer(Some(map.layers[0].clone()));
            scene.add_component_to_entity(entity_id, lc);

            let mut tc = TransformComponent::new();
            tc.set_position(0.0, 0.0);
            scene.add_component_to_entity(entity_id, tc);
        }
        {
            let entity_id = scene.create_entity();
            sdl2::log::log("Entity id follows");
            sdl2::log::log(&entity_id.to_string());
            let debug_name_instance = debug_name_manager.create(entity_id);
            Log::debug("Debug name instance");
            Log::debug(&debug_name_instance.i.to_string());
            debug_name_manager.set_name(debug_name_instance, String::from("entity4"));

            let mut ic = ImageComponent::new();
            ic.texture = Some(tm.get(&String::from("back")));
            scene.add_component_to_entity(entity_id, ic);

            let mut tc = TransformComponent::new();
            tc.set_position(0.0, 0.0);
            scene.add_component_to_entity(entity_id, tc);
        }
        {
            let mut tx = 0.0;
            let mut ty = 0.0;
            let mut tx2 = 1.0;
            let mut ty2 = 1.0;
            let region = atlas
                .frames
                .iter()
                .filter(|frame| frame.filename == "player/idle/player-idle-1")
                .collect::<Vec<&crate::atlas::texturepacker::Frame>>()[0];
            let sw = atlas.meta.size.w as f32;
            let sh = atlas.meta.size.h as f32;
            /*
            tx = region.frame.x as f32 / sw;
            ty = region.frame.y as f32 / sh;
            tx2 = (region.frame.x as f32 + region.frame.w as f32) / sw;
            ty2 = (region.frame.y as f32 + region.frame.h as f32) / sh;
            */
            tx = region.frame.x as f32;
            ty = region.frame.y as f32;
            tx2 = region.frame.w as f32;
            ty2 = region.frame.h as f32;

            let entity_id = scene.create_entity();
            sdl2::log::log("Entity id follows");
            sdl2::log::log(&entity_id.to_string());
            let debug_name_instance = debug_name_manager.create(entity_id);
            Log::debug("Debug name instance");
            Log::debug(&debug_name_instance.i.to_string());
            debug_name_manager.set_name(debug_name_instance, String::from("entity5"));

            let mut sc = SpriteComponent::new();
            let sub = Subtexture::with_texture(Some(tm.get(&String::from("atlas"))), tx as i32, ty as i32, tx2 as i32, ty2 as i32);
            sc.add_frame_with_subtexture(sub);
            scene.add_component_to_entity(entity_id, sc);

            let mut tc = TransformComponent::new();
            let x = 54.0 * 16.0;
            let y = 9.0 * 16.0;
            tc.set_position(x, y);
            scene.add_component_to_entity(entity_id, tc);
            player_camera.set_position(x - 320.0/2.0, y - 240.0/2.0);
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
            window: window,
            gl_context: gl_context,
            camera: player_camera,
            screen_render_target: screen_render_target,
            quad_shader: quad_shader,
        });
        self.scene = Some(scene);

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

pub struct SpriteSystem {
    base: BaseSystem,
}

impl System<MainLoopContext> for SpriteSystem {
    fn get_entities(&self) -> Vec<Entity> {
        return self.base.get_entities();
    }

    fn process(&self, entity: Entity, dt: f32, scene: &Scene<MainLoopContext>, user_data: &mut MainLoopContext) {
        //println!("SpriteSystem process()");
        let main_loop_data = user_data;
        let sc_compo = &mut scene.get_component::<SpriteComponent>(entity);
        let tc_compo = &mut scene.get_component::<TransformComponent>(entity);
        let sc = sc_compo.unwrap();
        let tc = tc_compo.unwrap();
        let current_frame = sc.get_current_frame();
        let frame = sc.get_frame(current_frame).unwrap();
        let tex = frame.get_texture().unwrap();
        let clip_rect = sc.get_source_rect();
        main_loop_data.sb.draw(tex, Some(*tc.get_position()), None, Some(*clip_rect), None, 0.0, None, Color::white(), 0.0);
    }

    fn get_components(&self) -> Vec<TypeId> {
        return self.base.get_components();
    }

    fn add_entity(&mut self, entity: Entity) {
        self.base.add_entity(entity);
    }
}

impl SpriteSystem {
    pub fn new() -> Self {
        SpriteSystem {
            base: BaseSystem::new()
        }
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