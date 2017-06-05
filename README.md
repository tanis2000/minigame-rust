Edit `src/test_shared.rs` and run `cargo build` to see hot reloading in action.

Run `cargo run` to run with the dynamic library
Run `cargo run --no-default-features` to run the application with all of the code statically linked and with hotloading disabled.

Installing needed targets:

```
# iOS. Note: you need *all* five targets
rustup target add aarch64-apple-ios armv7-apple-ios armv7s-apple-ios x86_64-apple-ios i386-apple-ios

# Android.
rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android
```

To build for iOS and Android:
```
cargo lipo --release
cargo build --target aarch64-linux-android --release
cargo build --target armv7-linux-androideabi --release
cargo build --target i686-linux-android --release
```

```
cargo build --no-default-features --lib
```