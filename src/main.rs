#![cfg_attr(any(target_os="ios", target_os="android"), no_main)]

extern crate minigame;

use minigame::engine::Engine;

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
}
