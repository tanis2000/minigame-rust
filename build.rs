extern crate curl;
extern crate gl_generator;

use curl::easy::Easy;
use gl_generator::{Registry, Api, Profile, Fallbacks, GlobalGenerator};
use std::env;
use std::path::Path;
use std::process::Command;
use std::fs;
use std::fs::File;
use std::io::Write;

fn main() {
    let profile = env::var("PROFILE").unwrap_or("Debug".to_string());
    let current_dir = std::env::current_dir().unwrap();
    let target;
    let target_os = env::var("TARGET").unwrap();
    const SDL2_FILENAME: &'static str = "SDL2-2.0.9.zip";
    const SDL2_URL: &'static str = "https://www.libsdl.org/release/SDL2-2.0.9.zip";
    const SDL2_PATH: &'static str = "SDL2-2.0.9";

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

    // Download SDL if needed
    if !Path::new(SDL2_FILENAME).exists() {
        download_from_url(SDL2_URL, SDL2_FILENAME);
    }

    if !Path::new(SDL2_PATH).exists() {
        unzip_file(SDL2_FILENAME);
    }

    //if target_os.contains("ios") {
        if !Path::new(&current_dir).join(SDL2_PATH).join("Xcode-iOS").join("SDL").join("build").join("Release-iphoneos").join("libSDL2.a").exists() {
            Command::new("xcodebuild")
            .args(&["-project", "SDL2-2.0.9/Xcode-iOS/SDL/SDL.xcodeproj", "-target", "libSDL-iOS", "-sdk", "iphoneos12.1"])
            .status()
            .expect("Error building iOS project");
        }

        if !Path::new(&current_dir).join(SDL2_PATH).join("Xcode-iOS").join("SDL").join("build").join("Release-iphonesimulator").join("libSDL2.a").exists() {
            Command::new("xcodebuild")
            .args(&["-project", "SDL2-2.0.9/Xcode-iOS/SDL/SDL.xcodeproj", "-target", "libSDL-iOS", "-sdk", "iphonesimulator12.1"])
            .status()
            .expect("Error building iOS Simulator project");
        }


        //fs::copy(Path::new(&current_dir).join(SDL2_PATH).join("Xcode-iOS").join("SDL").join("build").join("Release-iphoneos").join("libSDL2.a"), Path::new(&current_dir).join("target").join(target_os).join("debug").join("libSDL2.a"));
        println!("{:?}", Path::new(&current_dir).join(SDL2_PATH).join("Xcode-iOS").join("SDL").join("build").join("Release-iphonesimulator").join("libSDL2.a"));
        println!("{:?}", Path::new(&current_dir).join("target").join("x86_64-apple-ios").join("debug").join("libSDL2.a"));
        fs::copy(Path::new(&current_dir).join(SDL2_PATH).join("Xcode-iOS").join("SDL").join("build").join("Release-iphonesimulator").join("libSDL2.a"), Path::new(&current_dir).join("target").join("x86_64-apple-ios").join("debug").join("libSDL2.a")).expect("Cannot copy libSDL2 for iPhone Simulator");
    //}

    if target_os.contains("android") {
        println!("cargo:rustc-flags=-L android/Minigame/sdl/build/intermediates/cmake/debug/obj/armeabi",);
        println!("cargo:rustc-flags=-L android/Minigame/sdl/build/intermediates/cmake/debug/obj/armeabi-v7a",);
        println!("cargo:rustc-flags=-L android/Minigame/sdl/build/intermediates/cmake/debug/obj/x86",);
        println!("cargo:rustc-flags=-L android/Minigame/sdl/build/intermediates/cmake/debug/obj/arm64-v8a",);
        println!("cargo:rustc-flags=-L android/Minigame/sdl/build/intermediates/cmake/debug/obj/x86_64",);

        // We should also add the following instead of defining our toolchain in .cargo/config
        // -C link-arg=--sysroot=$NDK_ROOT/platforms/android-<api level you are targeting>/arch-arm

        let abi = if target_os.contains("armv7") {
            "armeabi-v7a"
        } else if target_os.contains("aarch64") {
            "arm64-v8a"
        } else if target_os.contains("arm") {
            "armeabi"
        } else if target_os.contains("x86_64") {
            "x86_64"
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
        let res = fs::copy(src, dst);
    }
}

fn download_from_url(url: &str, dst_file: &str) {
    File::create(dst_file).and_then(|mut file| {
        let mut curl = Easy::new();
        curl.url(url).expect("Error setting url");
        curl.write_function(move |data| Ok(file.write(data).expect("Error writing data")))
            .expect("Error setting write function");
        curl.perform().expect("Error downloading archive");
        Ok(())
    }).expect("Could not open output file");
}

fn unzip_file(filename: &str) {
    Command::new("unzip")
    .args(&[filename])
    .status()
    .expect("Error unzipping SDL2");
}