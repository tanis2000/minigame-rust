#!/bin/sh
# This will only compile and run for the x86 Android simulator
#cargo build --no-default-features --target i686-linux-android --lib
#cp target/i686-linux-android/debug/libminigame.so android/Minigame/app/src/main/jniLibs/x86/

# This is for armv7
# Run cargo with all the correct compiler settings without resorting to ~/.cargo/config
CC=/Users/tanis/android-ndk/toolchains/arm-linux-androideabi-4.9/prebuilt/darwin-x86_64/bin/arm-linux-androideabi-gcc CXX=/Users/tanis/android-ndk/toolchains/arm-linux-androideabi-4.9/prebuilt/darwin-x86_64/bin/arm-linux-androideabi-g++ AR=/Users/tanis/android-ndk/toolchains/arm-linux-androideabi-4.9/prebuilt/darwin-x86_64/bin/arm-linux-androideabi-ar CFLAGS=--sysroot=/Users/tanis/android-ndk/platforms/android-24/arch-arm/ CARGO_TARGET_ARMV7_LINUX_ANDROIDEABI_LINKER=/Users/tanis/android-ndk/toolchains/arm-linux-androideabi-4.9/prebuilt/darwin-x86_64/bin/arm-linux-androideabi-ld cargo build --no-default-features --target armv7-linux-androideabi --lib
# This was the old way using ~/.cargo/config
#CC=/Users/tanis/Documents/android-ndk-arm/bin/arm-linux-androideabi-gcc CXX=/Users/tanis/Documents/android-ndk-arm/bin/arm-linux-androideabi-g++ AR=/Users/tanis/Documents/android-ndk-arm/bin/arm-linux-androideabi-ar cargo build --no-default-features --target armv7-linux-androideabi --lib
cp target/armv7-linux-androideabi/debug/libminigame.so android/Minigame/app/src/main/jniLibs/armeabi/
cp target/armv7-linux-androideabi/debug/libminigame.so android/Minigame/app/src/main/jniLibs/armeabi-v7a/
cp assets/* android/Minigame/app/src/main/assets
cd android/Minigame/app
../gradlew assemble
adb -d uninstall org.libsdl.app
adb -d install build/outputs/apk/app-debug.apk
cd ../../..
