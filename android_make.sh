#!/bin/sh
# This will only compile and run for the x86 Android simulator
cargo build --no-default-features --target i686-linux-android --lib
cp target/i686-linux-android/debug/libminigame.so android/Minigame/app/src/main/jniLibs/x86/