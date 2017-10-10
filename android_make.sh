#!/bin/sh
# This will only compile and run for the x86 Android simulator
#cargo build --no-default-features --target i686-linux-android --lib
#cp target/i686-linux-android/debug/libminigame.so android/Minigame/app/src/main/jniLibs/x86/

# This is for armv7
CC=/Users/tanis/Documents/android-ndk-arm/bin/arm-linux-androideabi-gcc CXX=/Users/tanis/Documents/android-ndk-arm/bin/arm-linux-androideabi-g++ AR=/Users/tanis/Documents/android-ndk-arm/bin/arm-linux-androideabi-ar cargo build --no-default-features --target armv7-linux-androideabi --lib
cp target/armv7-linux-androideabi/debug/libminigame.so android/Minigame/app/src/main/jniLibs/armeabi/
cp target/armv7-linux-androideabi/debug/libminigame.so android/Minigame/app/src/main/jniLibs/armeabi-v7a/
cp assets/* android/Minigame/app/src/main/assets
cd android/Minigame/app
../gradlew assemble
adb -d uninstall org.libsdl.app
adb -d install build/outputs/apk/app-debug.apk
cd ../../..
