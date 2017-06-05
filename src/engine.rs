#[cfg(feature = "hotload")]
extern crate dynamic_reload;

extern crate sdl2;

//#[cfg(not(feature = "hotload"))]
//extern crate minigame;

#[cfg(feature = "hotload")]
use dynamic_reload::{DynamicReload, Lib, Symbol, Search, PlatformName, UpdateState};
#[cfg(feature = "hotload")]
use std::rc::Rc;
use std::time::Duration;
use std::thread;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
#[cfg(not(feature = "hotload"))]
use test_shared::shared_fun;

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

#[cfg(feature = "hotload")]
pub fn run_loop() {
    let mut plugs = Plugins { plugins: Vec::new() };

    // Setup the reload handler. A temporary directory will be created inside the target/debug
    // where plugins will be loaded from. That is because on some OS:es loading a shared lib
    // will lock the file so we can't overwrite it so this works around that issue.
    let mut reload_handler = DynamicReload::new(Some(vec!["target/debug"]),
                                                Some("target/debug"),
                                                Search::Default);

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rust-sdl2 demo: Video", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(255, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();

    // test_shared is generated in build.rs
    match reload_handler.add_library("test_shared", PlatformName::Yes) {
        Ok(lib) => plugs.add_plugin(&lib),
        Err(e) => {
            println!("Unable to load dynamic lib, err {:?}", e);
            return;
        }
    }

    //
    // While this is running (printing a number) change return value in file src/test_shared.rs
    // build the project with cargo build and notice that this code will now return the new value
    //
    'running: loop {
        reload_handler.update(Plugins::reload_callback, &mut plugs);

        if plugs.plugins.len() > 0 {
            // In a real program you want to cache the symbol and not do it every time if your
            // application is performance critical
            let fun: Symbol<extern "C" fn() -> i32> =
                unsafe { plugs.plugins[0].lib.get(b"shared_fun\0").unwrap() };

            println!("Value {}", fun());
        }

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                _ => {}
            }
        }
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        // The rest of the game loop goes here...


        // Wait for 0.5 sec
        thread::sleep(Duration::from_millis(500));
    }
}

#[cfg(not(feature = "hotload"))]
pub fn run_loop() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rust-sdl2 demo: Video", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(255, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        println!("Value {}", shared_fun());

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                _ => {}
            }
        }
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        // The rest of the game loop goes here...


        // Wait for 0.5 sec
        thread::sleep(Duration::from_millis(500));
    }
}