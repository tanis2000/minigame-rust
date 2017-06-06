extern crate sdl2;
#[cfg(feature = "hotload")]
extern crate dynamic_reload;

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn SDL_main() -> i32 {
    engine::run_loop();
    0
}

pub mod test_shared;
pub mod engine;