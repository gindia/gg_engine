extern crate sdl_wrapper;

use sdl_wrapper as plt;

fn main() {
    plt::init_gles2_static("window", 1600, 900);

    'main_loop: while plt::poll_events() {
        let kbd = unsafe { *plt::keyboard() };

        if kbd.current[plt::sys::KEY_ESCAPE as usize] == 1 {
            break 'main_loop;
        }

        plt::gl_swap_buffers();
    }

    plt::quit();
}
