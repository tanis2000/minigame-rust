# TODO

This is a collection of TODO items I should take care of in the long run.

## wasm32 support

In order to enable wasm32 support we need to get rid of all the libc dependencies we currently have.
The main issues are due to `stbi_image` and `SDL2`.

The best would probably be to replace them with the `image` crate and the `glutin` + `winit` combo so that we have pure Rust replacements.

