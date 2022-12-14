pub mod sys {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(improper_ctypes)]

    include!(concat!(env!("OUT_DIR"), "/sdl2_bindings.rs"));
}

pub fn init_gles2_static(window_name: &str, width: i32, height: i32) {
    unsafe {
        let c_window_name = std::ffi::CString::new(window_name).unwrap();
        sys::plt_init_gles2_static(c_window_name.as_ptr(), width, height);
    }
}

pub fn quit() {
    unsafe {
        sys::plt_quit();
    }
}

pub fn poll_events() -> bool {
    unsafe { sys::plt_poll_events() != 0 }
}

pub fn gl_swap_buffers() {
    unsafe {
        sys::plt_gl_swap_buffers();
    }
}

pub fn gl_set_vsync(on: bool) {
    unsafe {
        sys::plt_gl_set_vsync(on as i32);
    }
}

pub fn window_set_fullscreen(full: bool) {
    unsafe {
        sys::plt_window_set_fullscreen(full as i32);
    }
}

pub fn window_get_size() -> (i32, i32) {
    let mut w: i32 = 0;
    let mut h: i32 = 0;

    unsafe { sys::plt_window_get_size(&mut w, &mut h) };

    assert!(w != 0);
    assert!(h != 0);

    (w, h)
}

//void plt_log_info (const char *fmt, ...);
//void plt_log_warn (const char *fmt, ...);
//void plt_log_error(const char *fmt, ...);
//
//void plt_bin_save_to_desk  (const char *filename, u8 *__restrict buffer, size_t buffer_size);
//u8*  plt_bin_read_from_desk(const char *filename, size_t *size);
//void plt_bin_free(void *buffer);

pub fn mouse() -> *const sys::Mouse {
    unsafe { sys::plt_mouse() }
}

pub fn keyboard() -> *const sys::Keyboard {
    unsafe { sys::plt_keyboard() }
}

pub fn clock() -> *const sys::Clock {
    unsafe { sys::plt_clock() }
}
