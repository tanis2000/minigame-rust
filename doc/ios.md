# iOS related documentation

This document contains both the documentation about the old manual way of building for iOS and the new one which is way more streamlined.

## New way

We have all of the configuration and building automated in the custom `build.rs` script. All you need to do is build the Rust library that contains the game code and then compile the iOS Xcode project.

### Building the Rust library for the iOS simulator

To build for iOS simulator:

```sh
cargo build --no-default-features --target x86_64-apple-ios --lib
```

This command will also download SDL2 and compile it for both the simulator and iOS device

### Compiling the iOS project

Just open the `ios/minigame/minigame.xcodeproj` Xcode project and run it and you should be done. 

## Old way

### Building the Rust library for the iOS simulator (OLD)

To build for iOS simulator:

```sh
cargo build --no-default-features --target x86_64-apple-ios --lib
```

## Getting the iOS toolchain up and running

To get iOS running:

- Clone the SDL repo
- Open the Xcode-iOS project and build the library for both the simulator and the device
- Copy the resulting libraries in your target/[platform] folders
- Clone the sdl2-image repo from http://hg.libsdl.org/SDL_image/
- Open the Xcode-iOS project and build the library for both the simulator and the device
- Copy the resulting libraries in your target/[platform] folders
- Open the ios/minigame Xcode project and run it

Note: this is no longer needed as we have the whole iOS toolchain integrated in a custom cargo build script.
