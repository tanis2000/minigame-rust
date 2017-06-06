# minigame

This is a basic sample game made with Rust that runs on both desktop and mobile platforms.
Right now it's been tested on macOS and iOS and it works as long as you do some steps by hand.
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
cargo build --no-default-features --target x86_64-apple-ios
```

To get iOS running:

- Clone the SDL repo
- Open the Xcode-iOS project and build the library for both the simulator and the device
- Copy the resulting libraries in your target/[platform] folders 
- Open the ios/minigame Xcode project and run it