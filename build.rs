use std::env;
use std::path::Path;
use std::process::Command;

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
    }
}