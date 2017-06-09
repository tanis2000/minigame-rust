extern crate gl_generator;

use gl_generator::{Registry, Api, Profile, Fallbacks, GlobalGenerator};
use std::env;
use std::path::Path;
use std::process::Command;
use std::fs::File;

fn main() {
    let profile = env::var("PROFILE").unwrap_or("Debug".to_string());
    let current_dir = std::env::current_dir().unwrap();
    let target;
    let target_os = env::var("TARGET").unwrap();

    if profile == "Release" {
        target = Path::new(&current_dir).join("target/release");
    } else {
        target = Path::new(&current_dir).join("target/debug");
    }

    let dest = env::var("OUT_DIR").unwrap();
    let mut file = File::create(&Path::new(&dest).join("bindings.rs")).unwrap();

    Registry::new(Api::Gles2, (2, 0), Profile::Core, Fallbacks::All, [])
        .write_bindings(GlobalGenerator, &mut file)
        .unwrap();

    Command::new("rustc")
        .arg("src/test_shared.rs")
        .arg("--crate-name")
        .arg("test_shared")
        .arg("--crate-type")
        .arg("dylib")
        .arg("--out-dir")
        .arg(target)
        .output()
        .unwrap_or_else(|e| panic!("failed to execute process: {}", e));

    if target_os.contains("android") {
        println!("cargo:rustc-flags=-L android/Minigame/sdl/build/intermediates/cmake/debug/obj/armeabi",);
        println!("cargo:rustc-flags=-L android/Minigame/sdl/build/intermediates/cmake/debug/obj/armeabi-v7a",);
        println!("cargo:rustc-flags=-L android/Minigame/sdl/build/intermediates/cmake/debug/obj/x86",);

        // We should also add the following instead of defining our toolchain in .cargo/config
        // -C link-arg=--sysroot=$NDK_ROOT/platforms/android-<api level you are targeting>/arch-arm

        let abi = if target_os.contains("armv7") {
            "armeabi-v7a"
        } else if target_os.contains("aarch64") {
            "arm64-v8a"
        } else if target_os.contains("arm") {
            "armeabi"
        } else if target_os.contains("x86") {
            "x86"
        } else if target_os.contains("i686") {
            "x86"
        } else {
            panic!("Invalid target architecture {}", target_os);
        };

        let src = Path::new(&current_dir).join("target").join(target_os).join("debug").join("libminigame.so");
        let dst = Path::new(&current_dir).join("android/Minigame/app/src/main/jniLibs").join(abi).join("libminigame.so");
        //panic!("{:?}", dst);
        //std::fs::remove_file(Path::new(&dst)).unwrap();
        // This won't work as it's being executed before the actual library has finished building :(
        std::fs::copy(src, dst).unwrap();
    }
}