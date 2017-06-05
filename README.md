Edit `src/test_shared.rs` and run `cargo build` to see hot reloading in action.

Run `cargo run` to run with the dynamic library
Run `cargo run --no-default-features` to run the application with all of the code statically linked and with hotloading disabled.

To build for iOS and Android:
```
cargo lipo --release
cargo build --target aarch64-linux-android --release
cargo build --target armv7-linux-androideabi --release
cargo build --target i686-linux-android --release
```

