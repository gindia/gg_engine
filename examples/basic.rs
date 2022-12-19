extern crate gg_engine;

use gg_engine::*;

fn main() -> Result<(), &'static str>{
    sdl_wrapper::init_gles2_static("123", 1600, 900);

    'main_loop: while sdl_wrapper::poll_events() {
        if keyboard_key_clicked(sdl_wrapper::sys::KEY_ESCAPE) {
            break 'main_loop;
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
