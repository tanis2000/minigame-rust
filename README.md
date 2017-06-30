# minigame

This is a basic sample game made with Rust that runs on both desktop and mobile platforms.
Right now it's been tested on macOS, iOS and Android and it works as long as you do some steps by hand.
The idea is to have the whole show running on its own without the need of any manual step.

# Hot code reloading

This is a feature that is only available on desktop but it's very handy when working with gameplay code.

Edit `src/test_shared.rs` and run `cargo build` to see hot reloading in action.

Run `cargo run` to run with the dynamic library
Run `cargo run --no-default-features` to run the application with all of the code statically linked and with hotloading disabled.

# Installing needed targets for mobile:

```
# iOS. Note: you need *all* five targets
rustup target add aarch64-apple-ios armv7-apple-ios armv7s-apple-ios x86_64-apple-ios i386-apple-ios

# Android.
rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android
```

# Building without hot reloading

```
cargo build --no-default-features --lib
```

# Building the Rust library for the iOS simulator

To build for iOS simulator:
```
cargo build --no-default-features --target x86_64-apple-ios --lib
```

To get iOS running:

- Clone the SDL repo
- Open the Xcode-iOS project and build the library for both the simulator and the device
- Copy the resulting libraries in your target/[platform] folders 
- Clone the sdl2-image repo from http://hg.libsdl.org/SDL_image/
- Open the Xcode-iOS project and build the library for both the simulator and the device
- Copy the resulting libraries in your target/[platform] folders 
- Open the ios/minigame Xcode project and run it

# Building the Android standalone toolchain
```
/Users/tanis/Documents/android-sdk/ndk-bundle/build/tools/make_standalone_toolchain.py --arch arm --install-dir /Users/tanis/Documents/android-ndk-arm
/Users/tanis/Documents/android-sdk/ndk-bundle/build/tools/make_standalone_toolchain.py --arch arm64 --install-dir /Users/tanis/Documents/android-ndk-arm64
/Users/tanis/Documents/android-sdk/ndk-bundle/build/tools/make_standalone_toolchain.py --arch x86 --install-dir /Users/tanis/Documents/android-ndk-x86
```

# Configuration for Android linking

For the time being you have to use a standalone toolchain. I'm pretty sure this can be solved with some clever
code in `build.rs` by setting the correct sysroot, but that's something left for later. 

Edit `.cargo/config` and add the following:

```
[target.armv7-linux-androideabi]
linker = "/Users/tanis/Documents/android-ndk-arm/bin/arm-linux-androideabi-gcc"

[target.aarch64-linux-android]
linker = "/Users/tanis/Documents/android-ndk-arm64/bin/aarch64-linux-android-gcc"

[target.i686-linux-android]
linker = "/Users/tanis/Documents/android-ndk-x86/bin/i686-linux-android-gcc"
```

# Building the SDL2 library for Android

```
cd android/Minigame/sdl
../gradlew assemble
```

# Building the Rust library for Android

```
cargo build --no-default-features --target armv7-linux-androideabi --lib
cargo build --no-default-features --target i686-linux-android --lib
```

# Copying the Rust library to the Android project

```
cp target/armv7-linux-androideabi/debug/libminigame.so android/Minigame/app/src/main/jniLibs/armeabi/
cp target/armv7-linux-androideabi/debug/libminigame.so android/Minigame/app/src/main/jniLibs/armeabi-v7a/
cp target/i686-linux-android/debug/libminigame.so android/Minigame/app/src/main/jniLibs/x86/
```

# Buiding the actual Android application
```
cd android/Minigame/app
../gradlew assemble
```
