extern crate gg_engine;

use gg_engine::*;

fn main() -> Result<(), &'static str> {
    sdl_wrapper::init_gles2_static("window", 1600, 900);

    let raw_chunk = read_res("mixkit-retro-game-notification-212-edit.wav")?;

    let chunk = audio::Audio::init(raw_chunk.as_slice());

    'main_loop: while sdl_wrapper::poll_events() {
        if keyboard_key_clicked(sdl_wrapper::sys::KEY_ESCAPE) {
            break 'main_loop;
        }

        if keyboard_key_clicked(sdl_wrapper::sys::KEY_W) {
            chunk.play(-1, 0);
        }

        unsafe {
            use gles_wrapper::gl::*;
            glClear(GL_COLOR_BUFFER_BIT);
        }

        sdl_wrapper::gl_swap_buffers();
    }

    sdl_wrapper::quit();
    Ok(())
}
