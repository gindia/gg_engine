//!
//!

#![allow(dead_code, unused_variables)]

pub extern crate c_utils;
pub extern crate gles_wrapper;
pub extern crate sdl_wrapper;
pub extern crate vector_math;

use gles_wrapper::*;
use vector_math::*;

pub type Hash = u32;
pub fn one_at_a_time_hash(key: &str) -> Hash {
    unsafe { c_utils::sys::utl_hash_one_at_time(key.as_ptr() as *const i8, key.len() as u64) }
}

pub type Color = u32;

/// converts a hex to r, g, b, a floats from 0.0 -> 1.0
pub fn rgba(hex: u32) -> Vec4 {
    let inv = 1.0 / 255.0;
    let r = ((hex & 0xFF000000) >> 24) as f32 * inv;
    let g = ((hex & 0x00FF0000) >> 16) as f32 * inv;
    let b = ((hex & 0x0000FF00) >> 8) as f32 * inv;
    let a = (hex & 0x000000FF) as f32 * inv;

    vec4(r, g, b, a)
}

pub mod parsers {
    use crate::*;
    use c_utils::sys;

    #[derive(Debug)]
    pub struct Image {
        pub raw: Vec<u8>,
        pub width: i32,
        pub height: i32,
        pub channels: i32,
    }

    impl Image {
        /// takes in png, jpg .. etc buffer.
        pub fn init<'a>(buffer: &[u8]) -> Result<Self, &'a str> {
            let mut width: i32 = 0;
            let mut height: i32 = 0;
            let mut channels: i32 = 0;

            let raw_ptr = unsafe {
                sys::utl_image_load(
                    buffer.as_ptr(),
                    buffer.len() as i32,
                    &mut width,
                    &mut height,
                    &mut channels,
                )
            };

            if raw_ptr.is_null() || (width == 0) || (height == 0) || (channels == 0) {
                return Err("Failed to parse image");
            }

            let size = (width * height * channels) as usize;
            let mut raw: Vec<u8> = Vec::with_capacity(size);

            unsafe {
                std::ptr::copy_nonoverlapping(raw_ptr, raw.as_mut_ptr(), size);
                raw.set_len(size);
            };

            unsafe { sys::utl_image_free(raw_ptr) };

            Ok(Self {
                raw,
                width,
                height,
                channels,
            })
        }
    }

    #[derive(Debug)]
    pub struct TrueTypeFont {
        pub sys_font: sys::Font,
        pub pixels: Vec<u8>,
    }

    impl TrueTypeFont {
        pub fn init<'a>(buffer: &[u8], font_size: i32) -> Result<Self, &'a str> {
            let mut font: sys::Font = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
            let ttf_parsing_success =
                unsafe { sys::utl_font_parse(buffer.as_ptr() as *mut u8, font_size, &mut font) };

            if ttf_parsing_success == 0 {
                return Err("Failed to parse TTF");
            }

            let size = (font.img_width * font.img_height * font.img_channels) as usize;
            let mut pixels: Vec<u8> = Vec::with_capacity(size);

            unsafe {
                std::ptr::copy_nonoverlapping(font.img_buffer, pixels.as_mut_ptr(), size);
                pixels.set_len(size);
            };

            unsafe { sys::utl_font_free(&mut font) };

            Ok(Self {
                sys_font: font,
                pixels,
            })
        }

        /// returns (pixels, width, height, channels)
        pub fn get_image(&self) -> (&[u8], i32, i32, i32) {
            (
                self.pixels.as_slice(),
                self.sys_font.img_width,
                self.sys_font.img_height,
                self.sys_font.img_channels,
            )
        }

        /// returns (quad, nextXpos, nextYpos)
        pub fn get_quad_and_next_position(
            &self,
            c: char, // maybe will change this to u8 insted
            x: f32,
            y: f32,
        ) -> (sys::stbtt_aligned_quad, f32, f32) {
            let mut xpos = x;
            let mut ypos = y;
            let mut quad: sys::stbtt_aligned_quad;
            unsafe {
                quad = std::mem::MaybeUninit::zeroed().assume_init();
                sys::utl_font_get_quad(
                    &self.sys_font as *const sys::Font as *mut sys::Font,
                    c as std::os::raw::c_char,
                    &mut xpos,
                    &mut ypos,
                    &mut quad,
                );
            }

            (quad, xpos, ypos)
        }
    }

    #[derive(Debug)]
    pub struct Obj {
        pub position: Vec<[f32; 3]>,
        pub uv: Vec<[f32; 2]>,
        pub normal: Vec<[f32; 3]>,
        pub face: Vec<[usize; 3]>,
    }

    impl Obj {
        pub fn init<'a>(buffer: &[u8]) -> Result<Self, &'a str> {
            let file_raw = String::from_utf8_lossy(buffer);

            let mut position: Vec<[f32; 3]> = Vec::new();
            let mut uv: Vec<[f32; 2]> = Vec::new();
            let mut normal: Vec<[f32; 3]> = Vec::new();
            let mut face: Vec<[usize; 3]> = Vec::new();

            for line in file_raw.lines() {
                if line.starts_with("v ") {
                    let mut v: [f32; 3] = [0.0; 3];
                    let mut index = 0;
                    for x in line.split(' ') {
                        if let Ok(a) = x.parse::<f32>() {
                            v[index] = a;
                            index += 1;
                        }
                    }
                    assert!(index == 3);
                    position.push(v);
                } else if line.starts_with("vt ") {
                    let mut vt: [f32; 2] = [0.0; 2];
                    let mut index = 0;
                    for x in line.split(' ') {
                        if let Ok(a) = x.parse::<f32>() {
                            vt[index] = a;
                            index += 1;
                        }
                    }
                    assert!(index == 2);
                    uv.push(vt);
                } else if line.starts_with("vn ") {
                    let mut vn: [f32; 3] = [0.0; 3];
                    let mut index = 0;
                    for x in line.split(' ') {
                        if let Ok(a) = x.parse::<f32>() {
                            vn[index] = a;
                            index += 1;
                        }
                    }
                    assert!(index == 3);
                    normal.push(vn);
                } else if line.starts_with("f ") {
                    let itr: Vec<&str> = line.split(' ').collect();
                    match itr.len() {
                        4 => {
                            //triangular faces
                            for x in itr {
                                if x.contains('f') {
                                    continue;
                                }
                                let mut f: [usize; 3] = [0; 3];
                                let mut index = 0;
                                for y in x.split('/') {
                                    if let Ok(a) = y.parse::<usize>() {
                                        f[index] = a;
                                        index += 1;
                                    }
                                }
                                assert!(index == 3);
                                face.push(f);
                            }
                        }

                        5 => {
                            // square faces
                            todo!()
                        }

                        _ => unreachable!(),
                    }
                } else {
                    // do nothing
                }
            }

            Ok(Self {
                position,
                uv,
                normal,
                face,
            })
        }

        pub fn gl_3_2_3_vertices(&self) -> Vec<f32> {
            let mut vertices: Vec<f32> = Vec::new();
            for i in 0..self.face.len() {
                // NOTE: subtract 1 from face-indices becuse they start with 1 not 0.
                let pos0 = self.position[self.face[i][0] - 1];
                let uv0 = self.uv[self.face[i][1] - 1];
                let norm0 = self.normal[self.face[i][2] - 1];

                vertices.push(pos0[0]);
                vertices.push(pos0[1]);
                vertices.push(pos0[2]);
                vertices.push(uv0[0]);
                vertices.push(uv0[1]);
                vertices.push(norm0[0]);
                vertices.push(norm0[1]);
                vertices.push(norm0[2]);
            }

            vertices
        }
    }
}

pub type Texture = gles_wrapper::Texture;
pub type Shader = gles_wrapper::Shader;

pub fn keyboard_key_clicked(key: u32) -> bool {
    let keyboard: sdl_wrapper::sys::Keyboard = unsafe { *sdl_wrapper::keyboard() };
    keyboard.current[key as usize] != 0 && keyboard.previous[key as usize] == 0
}

pub fn keyboard_key_released(key: u32) -> bool {
    let keyboard = unsafe { *sdl_wrapper::keyboard() };
    keyboard.current[key as usize] == 0 && keyboard.previous[key as usize] != 0
}

pub fn keyboard_key_hold(key: u32) -> bool {
    let keyboard = unsafe { *sdl_wrapper::keyboard() };
    keyboard.current[key as usize] != 0 && keyboard.previous[key as usize] != 0
}

/// return dt in seconds
pub fn clock_delta_time() -> f64 {
    let clock = unsafe { *sdl_wrapper::clock() };
    clock.delta_time
}

pub fn clock_milliseconds_from_start() -> u64 {
    let clock = unsafe { *sdl_wrapper::clock() };
    clock.milliseconds
}

pub struct SpriteSheet {
    pub texture: Texture,
    pub texture_width: i32,
    pub texture_height: i32,
    pub cell_size: i32,
}

impl SpriteSheet {
    pub fn init<'a>(
        pixels: &[u8],
        width: i32,
        height: i32,
        channels: i32,
        cell_size: i32,
    ) -> Result<Self, &'a str> {
        let texture = Texture::init(pixels.as_ptr(), width, height, channels)?;
        let texture_width = width;
        let texture_height = height;

        Ok(Self {
            texture,
            texture_width,
            texture_height,
            cell_size,
        })
    }
}

pub struct SpriteRenderer {
    shader: Shader,
    vao: Vao,
    vbo: Vbo,
}

impl SpriteRenderer {
    pub fn init() -> Self {
        let shader_src = r#"
#ifdef GL_ES
precision lowp float;
#endif

//////////////////////////////////////////////////////////////////////////////////////////////////
#if defined(VERTEX_SHADER)
//////////////////////////////////////////////////////////////////////////////////////////////////

layout (location = 0) in vec4 in_data;

out vec2 frag_uv;

uniform mat4 u_space_matrix;
uniform mat4 u_model;

void
main()
{
    gl_Position = u_space_matrix * u_model * vec4(in_data.xy, 0.f, 1.f);
    frag_uv     = in_data.zw;
}

//////////////////////////////////////////////////////////////////////////////////////////////////
#elif defined(FRAGMENT_SHADER)
//////////////////////////////////////////////////////////////////////////////////////////////////

in vec2 frag_uv;

out vec4 out_frag_color;

uniform vec4      u_taint;
uniform sampler2D u_tex0;
uniform int       u_use_texture;

void
main()
{
    vec4 mapped_tex;
    if (u_use_texture == 1) {
        mapped_tex = texture(u_tex0, frag_uv);
    } else {
        mapped_tex = vec4(1.0);
    }

    out_frag_color  = mapped_tex * u_taint;
}

//////////////////////////////////////////////////////////////////////////////////////////////////
#endif
//////////////////////////////////////////////////////////////////////////////////////////////////
// vim: sw=4 ft=glsl
        "#;

        let shader = Shader::init(shader_src).expect("Failed to compile the builtin shader");

        let mut vao = Vao::init();
        let vbo = Vbo::init_f32_2_2();
        vao.bind_vbo(&vbo);

        Self { shader, vao, vbo }
    }

    pub fn draw(
        &self,
        sheet: &SpriteSheet,
        sprite_colomn: i32,
        sprite_row: i32,
        pos: Vec2,
        rotation: f32,
        taint: u32,
    ) {
        use gles_wrapper::gl::*;

        let (window_width, window_height) = sdl_wrapper::window_size();

        let u_space_matrix = self.shader.uniform("u_space_matrix", UnifomType::Matrix4x4);
        let u_model = self.shader.uniform("u_model", UnifomType::Matrix4x4);
        let u_tex0 = self.shader.uniform("u_tex0", UnifomType::I32);
        let u_use_texture = self.shader.uniform("u_use_texture", UnifomType::I32);
        let u_taint = self.shader.uniform("u_taint", UnifomType::Vec4);

        let space_matrix = Mat4::ortho(
            0.0,
            window_width as f32,
            window_height as f32,
            0.0,
            -1.0,
            1.0,
        );

        let rot = Mat4::rotation_deg(rotation, 0.0, 0.0);
        let transform = Mat4::identity()
            .scale(Vec3 {
                x: sheet.cell_size as f32,
                y: sheet.cell_size as f32,
                z: 1.0,
            })
            .translate(Vec3 {
                x: pos.x + (sheet.cell_size as f32 * 0.5),
                y: pos.y + (sheet.cell_size as f32 * 0.5),
                z: 0.0,
            });
        let model = transform * rot;

        self.shader.use_();
        sheet.texture.bind(0);

        u_space_matrix.update_value(&space_matrix).unwrap();
        u_model.update_value(&model).unwrap();
        u_tex0.update_value(0).unwrap();
        u_use_texture.update_value(1).unwrap();
        u_taint.update_value(rgba(taint)).unwrap();

        unsafe {
            glEnable(GL_BLEND);
            glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);
        }

        let iw = 1.0 / sheet.texture_width as f32;
        let ih = 1.0 / sheet.texture_height as f32;

        let (x0, x1, y0, y1, s0, s1, t0, t1);
        x0 = -0.5;
        x1 = 0.5;
        y0 = -0.5;
        y1 = 0.5;
        s0 = (sheet.cell_size * sprite_colomn) as f32 * iw;
        s1 = (sheet.cell_size * (sprite_colomn + 1)) as f32 * iw;
        t0 = (sheet.cell_size * sprite_row) as f32 * ih;
        t1 = (sheet.cell_size * (sprite_row + 1)) as f32 * ih;

        debug_assert!(
            s0 <= 1.0,
            "asking for non existing sprite colomn in sprite sheet"
        );
        debug_assert!(
            s1 <= 1.0,
            "asking for non existing sprite colomn in sprite sheet"
        );
        debug_assert!(
            t0 <= 1.0,
            "asking for non existing sprite row in sprite sheet"
        );
        debug_assert!(
            t1 <= 1.0,
            "asking for non existing sprite row in sprite sheet"
        );

        let quads = [
            x1, y1, s1, t1, // 0
            x1, y0, s1, t0, // 1
            x0, y1, s0, t1, // 2
            x1, y0, s1, t0, // 3
            x0, y0, s0, t0, // 4
            x0, y1, s0, t1, // 5
        ];

        self.vbo.update(quads.as_slice());
        self.vao.draw_triangles();

        unsafe {
            glBindVertexArray(0);
            glDisable(GL_BLEND);
        }
    }

    pub fn blit_rect(&self, min: Vec2, max: Vec2, taint: u32) {
        use gles_wrapper::gl::*;

        let (window_width, window_height) = sdl_wrapper::window_size();

        let u_space_matrix = self.shader.uniform("u_space_matrix", UnifomType::Matrix4x4);
        let u_model = self.shader.uniform("u_model", UnifomType::Matrix4x4);
        let u_use_texture = self.shader.uniform("u_use_texture", UnifomType::I32);
        let u_taint = self.shader.uniform("u_taint", UnifomType::Vec4);

        let space_matrix = Mat4::ortho(
            0.0,
            window_width as f32,
            window_height as f32,
            0.0,
            -1.0,
            1.0,
        );

        let model = Mat4::identity();

        self.shader.use_();

        u_space_matrix.update_value(&space_matrix).unwrap();
        u_model.update_value(&model).unwrap();
        u_use_texture.update_value(0).unwrap();
        u_taint.update_value(rgba(taint)).unwrap();

        unsafe {
            glEnable(GL_BLEND);
            glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);
        }

        let (x0, x1, y0, y1, s0, s1, t0, t1);
        x0 = min.x;
        x1 = max.x;
        y0 = min.y;
        y1 = max.y;
        s0 = 0.0;
        s1 = 1.0;
        t0 = 0.0;
        t1 = 1.0;

        let quads = [
            x1, y1, s1, t1, // 0
            x1, y0, s1, t0, // 1
            x0, y1, s0, t1, // 2
            x1, y0, s1, t0, // 3
            x0, y0, s0, t0, // 4
            x0, y1, s0, t1, // 5
        ];

        self.vbo.update(quads.as_slice());
        self.vao.draw_triangles();

        unsafe {
            glBindVertexArray(0);
            glDisable(GL_BLEND);
        }
    }
}

pub struct TextRenderer {
    font: parsers::TrueTypeFont,
    shader: Shader,
    texture: Texture,
    font_size: i32,
    vao: Vao,
    vbo: Vbo,
}

impl TextRenderer {
    pub fn init(ttf: parsers::TrueTypeFont) -> Self {
        let (pixels, width, height, channels) = ttf.get_image();
        let texture = Texture::init(pixels.as_ptr(), width, height, channels)
            .expect("Failed to create GL Texture");

        let mut vao = Vao::init();
        let vbo = Vbo::init_f32_2_2();
        vao.bind_vbo(&vbo);

        let shader_src = r#"
#ifdef GL_ES
precision lowp float;
#endif

//////////////////////////////////////////////////////////////////////////////////////////////////
#if defined(VERTEX_SHADER)
//////////////////////////////////////////////////////////////////////////////////////////////////

layout (location = 0) in vec4 in_data;

out vec2 frag_uv;

uniform mat4 u_space_matrix;

void
main()
{
  gl_Position = u_space_matrix * vec4(in_data.xy, 0.f, 1.f);
  frag_uv     = in_data.zw;
}

//////////////////////////////////////////////////////////////////////////////////////////////////
#elif defined(FRAGMENT_SHADER)
//////////////////////////////////////////////////////////////////////////////////////////////////

in vec2 frag_uv;

out vec4 out_frag_color;

uniform vec4      u_taint;
uniform sampler2D u_tex0;

void
main()
{
  vec4 mapped_tex = texture(u_tex0, frag_uv);
  mapped_tex.a = mapped_tex.r;
  mapped_tex.rgb = vec3(1.0);

  out_frag_color = mapped_tex * u_taint;
}

//////////////////////////////////////////////////////////////////////////////////////////////////
#endif
//////////////////////////////////////////////////////////////////////////////////////////////////
// vim: sw=2 ft=glsl
        "#;
        let shader = Shader::init(shader_src).expect("Failed to compile the builtin shader");

        let font_size = ttf.sys_font.font_size;
        Self {
            font: ttf,
            shader,
            texture,
            font_size,
            vao,
            vbo,
        }
    }

    pub fn draw(&self, txt: &str, pos: Vec2, taint: u32) {
        use gles_wrapper::gl::*;

        let (window_width, window_height) = sdl_wrapper::window_size();

        let u_space_matrix = self.shader.uniform("u_space_matrix", UnifomType::Matrix4x4);
        let u_tex0 = self.shader.uniform("u_tex0", UnifomType::I32);
        let u_taint = self.shader.uniform("u_taint", UnifomType::Vec4);

        let space_matrix = Mat4::ortho(
            0.0,
            window_width as f32,
            window_height as f32,
            0.0,
            -1.0,
            1.0,
        );

        u_space_matrix.update_value(&space_matrix).unwrap();
        u_tex0.update_value(0).unwrap();
        u_taint.update_value(rgba(taint)).unwrap();

        unsafe {
            glEnable(GL_BLEND);
            glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);
        }

        self.shader.use_();
        self.texture.bind(0);

        let mut xpos = pos.x;
        let mut ypos = pos.y + (self.font_size as f32 * 0.5);
        let mut q;
        for c in txt.chars() {
            (q, xpos, ypos) = self.font.get_quad_and_next_position(c, xpos, ypos);

            let quads = [
                q.x1, q.y1, q.s1, q.t1, // 0
                q.x1, q.y0, q.s1, q.t0, // 1
                q.x0, q.y1, q.s0, q.t1, // 2
                q.x1, q.y0, q.s1, q.t0, // 3
                q.x0, q.y0, q.s0, q.t0, // 4
                q.x0, q.y1, q.s0, q.t1, // 5
            ];

            self.vbo.update(quads.as_slice());
            self.vao.draw_triangles();
        }

        unsafe {
            glBindVertexArray(0);
            glDisable(GL_BLEND);
        }
    }
}
