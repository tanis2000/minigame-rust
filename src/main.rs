#![cfg_attr(any(target_os="ios", target_os="android"), no_main)]

extern crate minigame;

use std::cell::RefCell;
use std::os::raw::{c_int, c_void, c_uchar};
use std::ptr::null_mut;

use minigame::engine::Engine;

#[cfg(target_arch = "wasm32")]
#[allow(non_camel_case_types)]
type em_callback_func = unsafe extern "C" fn();

#[cfg(target_arch = "wasm32")]
extern "C" {
    // This extern is built in by Emscripten.
    pub fn emscripten_run_script_int(x: *const c_uchar) -> c_int;
    pub fn emscripten_cancel_main_loop();
    pub fn emscripten_set_main_loop(func: em_callback_func,
                                    fps: c_int,
                                    simulate_infinite_loop: c_int);
}

#[cfg(target_arch = "wasm32")]
thread_local!(static MAIN_LOOP_CALLBACK: RefCell<Option<Box<dyn FnMut()>>> = RefCell::new(None));

#[cfg(target_arch = "wasm32")]
pub fn set_main_loop_callback<F: 'static>(callback : F) where F : FnMut() {
    MAIN_LOOP_CALLBACK.with(|log| {
        *log.borrow_mut() = Some(Box::new(callback));
    });

    unsafe { emscripten_set_main_loop(wrapper::<F>, 0, 1); }

    extern "C" fn wrapper<F>() where F : FnMut() {
        MAIN_LOOP_CALLBACK.with(|z| {
            if let Some(ref mut callback) = *z.borrow_mut() {
                callback();
            }
        });
    }
}

fn main() {
    main2();
}

#[cfg(any(target_os="ios", target_os="android"))]
#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn SDL_main() -> i32 {
    main2();
    0
}

fn main2() {
    let mut e = Engine::new();
    e.run_loop();
    #[cfg(target_arch = "wasm32")]
    {
        set_main_loop_callback(move || {
            e.main_loop();
        });
    }

}
