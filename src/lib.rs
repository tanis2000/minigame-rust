extern crate sdl2;
extern crate rand;
extern crate imgui;
extern crate stb_image;

#[cfg(feature = "hotload")]
extern crate dynamic_reload;
#[cfg(target_os="android")]
extern crate jni;


#[cfg(target_os="android")]
use jni::objects::JObject;
#[cfg(target_os="android")]
use jni::objects::JClass;
#[cfg(target_os="android")]
use jni::JNIEnv;
#[cfg(target_os="android")]
use jni::sys::jint;
#[cfg(target_os="android")]
use sdl2::libc::c_char;

pub mod test_shared;
pub mod engine;
pub mod blendmode;
pub mod shader;
pub mod renderstate;
pub mod rectangle;
pub mod texture;
pub mod color;
pub mod vertexpositioncolortexture;
pub mod log;
pub mod graphicsdevice;
pub mod spritebatchitem;
pub mod spritebatch;
pub mod spritebatcher;
pub mod spritefont;
pub mod texturemanager;
pub mod camera;
pub mod utils;
pub mod viewportadapter;
pub mod entity;
pub mod component;
pub mod componentlist;
pub mod entitylist;
pub mod scene;
pub mod collider;
pub mod colliderlist;
pub mod subtexture;
pub mod imagecomponent;
pub mod renderer;
pub mod everythingrenderer;

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn SDL_main() -> i32 {
    let mut e = engine::Engine::new();
    e.run_loop();
    //engine::run_loop();
    0
}

#[cfg(target_os="android")]
extern "C" {
    fn SDL_Android_Init(env: JNIEnv, cls: JClass);
    fn SDL_SetMainReady();
}

#[cfg(target_os="android")]
#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_org_libsdl_app_SDLActivity_nativeInit(env: JNIEnv, cls: JClass, array: JObject) -> jint
{
    let mut i: i32;
    let mut argc: i32;
    let mut status: i32;
    let mut len: i32;
    let mut argv: *const *const c_char;

    /* This interface could expand with ABI negotiation, callbacks, etc. */
    SDL_Android_Init(env, cls);

    SDL_SetMainReady();

    /* Prepare the arguments. */

/*
    len = (*env)->GetArrayLength(env, array);
    argv = SDL_stack_alloc(char*, 1 + len + 1);
    argc = 0;
    */
    /* Use the name "app_process" so PHYSFS_platformCalcBaseDir() works.
       https://bitbucket.org/MartinFelis/love-android-sdl2/issue/23/release-build-crash-on-start
     */
     /*
    argv[argc++] = SDL_strdup("app_process");
    for (i = 0; i < len; ++i) {
        const char* utf;
        char* arg = NULL;
        jstring string = (*env)->GetObjectArrayElement(env, array, i);
        if (string) {
            utf = (*env)->GetStringUTFChars(env, string, 0);
            if (utf) {
                arg = SDL_strdup(utf);
                (*env)->ReleaseStringUTFChars(env, string, utf);
            }
            (*env)->DeleteLocalRef(env, string);
        }
        if (!arg) {
            arg = SDL_strdup("");
        }
        argv[argc++] = arg;
    }
    argv[argc] = NULL;
*/

    /* Run the application. */

    status = SDL_main(/*argc, argv*/);

    /* Release the arguments. */
/*
    for (i = 0; i < argc; ++i) {
        SDL_free(argv[i]);
    }
    SDL_stack_free(argv);*/
    /* Do not issue an exit or the whole application will terminate instead of just the SDL thread */
    /* exit(status); */

    return status;
}
