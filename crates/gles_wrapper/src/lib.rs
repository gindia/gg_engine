//! A warpper around GLESv2.
//!
//! for generating GLES bindings, read https://rust-lang.github.io/rust-bindgen/requirements.html
//! for more info.
//!

pub mod gl {
    // can check https://registry.khronos.org/OpenGL/api/GLES3/gl3.h for more info
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]

    include!(concat!(env!("OUT_DIR"), "/gles_bindings.rs"));
}

extern crate vector_math;

use std::{ffi::CString, mem::size_of, os::raw::c_void, ptr::null};

use gl::*;
use vector_math::*;

#[derive(Debug, Eq, PartialEq)]
pub enum UnifomType {
    I32,
    F32,
    Matrix4x4,
    Vec3,
    Vec4,
}

#[derive(Debug)]
pub struct Uniform {
    pub location: gl::GLint,
    pub u_type: UnifomType,
}

pub trait UniformUpdate<T> {
    fn update_value(&self, value: T) -> Result<(), &str>;
}

impl UniformUpdate<i32> for Uniform {
    fn update_value(&self, value: i32) -> Result<(), &str> {
        if self.u_type != UnifomType::I32 {
            Err("Trying to update a shader Uniform with wrong type.")
        } else {
            unsafe {
                if self.location != -1 {
                    glUniform1i(self.location, value);
                } else {
                    return Err("Was Unable to update value!");
                }
            }

            Ok(())
        }
    }
}

impl UniformUpdate<f32> for Uniform {
    fn update_value(&self, value: f32) -> Result<(), &str> {
        if self.u_type != UnifomType::F32 {
            Err("Trying to update a shader Uniform with wrong type.")
        } else {
            unsafe {
                if self.location != -1 {
                    glUniform1f(self.location, value);
                } else {
                    return Err("Was Unable to update value!");
                }
            }

            Ok(())
        }
    }
}

impl UniformUpdate<&Mat4> for Uniform {
    fn update_value(&self, value: &Mat4) -> Result<(), &str> {
        if self.u_type != UnifomType::Matrix4x4 {
            Err("Trying to update a shader Uniform with wrong type.")
        } else {
            unsafe {
                if self.location != -1 {
                    glUniformMatrix4fv(self.location, 1, GL_FALSE as u8, value.as_ptr());
                } else {
                    return Err("Was Unable to update value!");
                }
            }

            Ok(())
        }
    }
}

impl UniformUpdate<Vec3> for Uniform {
    fn update_value(&self, value: Vec3) -> Result<(), &str> {
        if self.u_type != UnifomType::Vec3 {
            Err("Trying to update a shader Uniform with wrong type.")
        } else {
            unsafe {
                if self.location != -1 {
                    glUniform3fv(self.location, 1, value.as_ptr());
                } else {
                    return Err("Was Unable to update value!");
                }
            }

            Ok(())
        }
    }
}

impl UniformUpdate<Vec4> for Uniform {
    fn update_value(&self, value: Vec4) -> Result<(), &str> {
        if self.u_type != UnifomType::Vec4 {
            Err("Trying to update a shader Uniform with wrong type.")
        } else {
            unsafe {
                if self.location != -1 {
                    glUniform4fv(self.location, 1, value.as_ptr());
                } else {
                    return Err("Was Unable to update value!");
                }
            }

            Ok(())
        }
    }
}

#[derive(Debug)]
pub struct Shader {
    pub program: gl::GLuint,
}

const GLSL_VERSION: &str = "#version 300 es\n";
const SHADER_VERTEX_DEF: &str = "#define VERTEX_SHADER\n";
const SHADER_FRAGMENT_DEF: &str = "#define FRAGMENT_SHADER\n";

impl Shader {
    /// takes in 1 source file that contains both vertex and fragmnet shader seprated with ifdefs.
    /// `#ifdef VERTEX_SHADER`.
    /// `#ifdef FRAGMENT_SHADER`.
    pub fn init<'a>(src: &str) -> Result<Self, &'a str> {
        unsafe {
            let program = glCreateProgram();
            let v_shader = glCreateShader(GL_VERTEX_SHADER);
            let f_shader = glCreateShader(GL_FRAGMENT_SHADER);

            let v_src: [*const GLchar; 3] = [
                GLSL_VERSION.as_ptr() as *const i8,
                SHADER_VERTEX_DEF.as_ptr() as *const i8,
                src.as_ptr() as *const i8,
            ];
            let f_src: [*const GLchar; 3] = [
                GLSL_VERSION.as_ptr() as *const i8,
                SHADER_FRAGMENT_DEF.as_ptr() as *const i8,
                src.as_ptr() as *const i8,
            ];
            let v_len: [GLint; 3] = [
                GLSL_VERSION.len() as GLint,
                SHADER_VERTEX_DEF.len() as GLint,
                src.len() as GLint,
            ];
            let f_len: [GLint; 3] = [
                GLSL_VERSION.len() as GLint,
                SHADER_FRAGMENT_DEF.len() as GLint,
                src.len() as GLint,
            ];

            glShaderSource(
                v_shader,
                3,
                v_src.as_ptr() as *const *const GLchar,
                v_len.as_ptr() as *const GLint,
            );
            glShaderSource(
                f_shader,
                3,
                f_src.as_ptr() as *const *const GLchar,
                f_len.as_ptr() as *const GLint,
            );

            let mut success = 0;

            glCompileShader(v_shader);
            glGetShaderiv(v_shader, GL_COMPILE_STATUS, &mut success);

            if success != 0 {
                glCompileShader(f_shader);
                glGetShaderiv(f_shader, GL_COMPILE_STATUS, &mut success);

                if success != 0 {
                    glAttachShader(program, v_shader);
                    glAttachShader(program, f_shader);
                    glLinkProgram(program);
                    glGetProgramiv(program, GL_LINK_STATUS, &mut success);

                    if success != 0 {
                        glDetachShader(program, v_shader);
                        glDetachShader(program, f_shader);
                    } else {
                        let mut error_log: [u8; 1024] = [0; 1024];
                        glGetProgramInfoLog(
                            program,
                            1024,
                            std::ptr::null_mut(),
                            error_log.as_mut_ptr() as *mut i8,
                        );
                        let e = String::from_utf8_lossy(error_log.as_slice());
                        println!("Shader Program :: {}", e);
                        glDeleteProgram(program);
                    }
                } else {
                    let mut error_log: [u8; 1024] = [0; 1024];
                    glGetShaderInfoLog(
                        f_shader,
                        1024,
                        std::ptr::null_mut(),
                        error_log.as_mut_ptr() as *mut i8,
                    );
                    let e = String::from_utf8_lossy(error_log.as_slice());
                    println!("FRAGMENT_SHADER :: {}", e);
                }
            } else {
                let mut error_log: [u8; 1024] = [0; 1024];
                glGetShaderInfoLog(
                    v_shader,
                    1024,
                    std::ptr::null_mut(),
                    error_log.as_mut_ptr() as *mut i8,
                );
                let e = String::from_utf8_lossy(error_log.as_slice());
                println!("VERTEX_SHADER :: {}", e);
            }

            glDeleteShader(v_shader);
            glDeleteShader(f_shader);

            if (success == 0) && (glIsProgram(program) != 0) {
                return Err("Shader:: Failed to compile");
            }

            Ok(Self { program })
        }
    }

    pub fn uniform(&self, name: &str, u_type: UnifomType) -> Uniform {
        unsafe {
            glUseProgram(self.program);
            let c_name = CString::new(name).unwrap();
            let location = glGetUniformLocation(self.program, c_name.as_c_str().as_ptr());

            Uniform { location, u_type }
        }
    }

    /// same as glUseProgram(self.program);
    pub fn use_(&self) {
        unsafe { glUseProgram(self.program) };
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe { glDeleteProgram(self.program) };
    }
}

#[derive(Debug)]
pub struct Texture {
    pub handle: gl::GLuint,
}

impl Texture {
    pub fn init<'a>(
        pixels: *const u8,
        width: i32,
        height: i32,
        channels: i32,
    ) -> Result<Self, &'a str> {
        if pixels.is_null() {
            return Err("Passing Null to Texture");
        }

        unsafe {
            let mut texture = 0;

            glGenTextures(1, &mut texture);
            glBindTexture(GL_TEXTURE_2D, texture);

            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_CLAMP_TO_EDGE as i32);
            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_CLAMP_TO_EDGE as i32);

            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_NEAREST as i32);
            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_NEAREST as i32);
            match channels {
                1 => {
                    glPixelStorei(GL_UNPACK_ALIGNMENT, 1);
                    glTexImage2D(
                        GL_TEXTURE_2D,
                        0,
                        GL_RED as i32,
                        width,
                        height,
                        0,
                        GL_RED,
                        GL_UNSIGNED_BYTE,
                        pixels as *mut std::os::raw::c_void,
                    );
                }
                3 => {
                    glPixelStorei(GL_UNPACK_ALIGNMENT, 1);
                    glTexImage2D(
                        GL_TEXTURE_2D,
                        0,
                        GL_RGB as i32,
                        width,
                        height,
                        0,
                        GL_RGB,
                        GL_UNSIGNED_BYTE,
                        pixels as *mut std::os::raw::c_void,
                    );
                }
                4 => {
                    glPixelStorei(GL_UNPACK_ALIGNMENT, 4);
                    glTexImage2D(
                        GL_TEXTURE_2D,
                        0,
                        GL_RGBA as i32,
                        width,
                        height,
                        0,
                        GL_RGBA,
                        GL_UNSIGNED_BYTE,
                        pixels as *mut std::os::raw::c_void,
                    );
                }

                _ => {
                    return Err("Passing image with unsupported number of channels.");
                }
            }

            glBindTexture(GL_TEXTURE_2D, 0);

            Ok(Self { handle: texture })
        }
    }

    pub fn bind(&self, index: u32) {
        unsafe {
            assert!(
                index < GL_MAX_COMBINED_TEXTURE_IMAGE_UNITS,
                "GLES, Texture index out of bounds, please pick an index between 0 and {}.",
                GL_MAX_COMBINED_TEXTURE_IMAGE_UNITS
            );

            glActiveTexture(GL_TEXTURE0 + index);
            glBindTexture(GL_TEXTURE_2D, self.handle);
        }
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            glDeleteTextures(1, &self.handle);
        }
    }
}

#[derive(Debug)]
pub enum VertexFormat {
    StaticF32_3_2_3,
    DynamicF32_2_2,
}

#[derive(Debug)]
pub struct Vbo {
    pub handle: gl::GLuint,
    pub vertex_format: VertexFormat,
    pub vertices_count: gl::GLsizei,
}

impl Vbo {
    pub fn init_f32_3_2_3(vertices: &[f32]) -> Self {
        Self::init(Some(vertices), VertexFormat::StaticF32_3_2_3)
    }

    /// expecting to use Vbo::update() after creation.
    pub fn init_f32_2_2() -> Self {
        Self::init(None, VertexFormat::DynamicF32_2_2)
    }

    fn init(vertices: Option<&[f32]>, vertex_format: VertexFormat) -> Self {
        match vertex_format {
            VertexFormat::StaticF32_3_2_3 => {
                let vertices = vertices.expect(
                    "passig invalid vertices (None) with worng vertex_format (StaticF32_3_2_3)",
                );

                assert_ne!(vertices.len(), 0, "passing empty vertices to Vbo.");

                let mut handle: gl::GLuint = 0;
                let vertices_count: gl::GLsizei = vertices.len() as i32 / 8;

                unsafe {
                    glGenBuffers(1, &mut handle);
                    glBindBuffer(GL_ARRAY_BUFFER, handle);
                    glBufferData(
                        GL_ARRAY_BUFFER,
                        (vertices.len() * size_of::<GLfloat>()) as i64,
                        vertices.as_ptr() as *const GLvoid,
                        GL_STATIC_DRAW,
                    );
                    glBindBuffer(GL_ARRAY_BUFFER, 0);
                }

                Self {
                    handle,
                    vertex_format,
                    vertices_count,
                }
            }
            VertexFormat::DynamicF32_2_2 => {
                let mut handle: gl::GLuint = 0;
                let vertices_count: gl::GLsizei = 6;

                unsafe {
                    glGenBuffers(1, &mut handle);
                    glBindBuffer(GL_ARRAY_BUFFER, handle);
                    glBufferData(
                        GL_ARRAY_BUFFER,
                        (6 * 4 * size_of::<GLfloat>()) as i64,
                        null(),
                        GL_DYNAMIC_DRAW,
                    );
                    glBindBuffer(GL_ARRAY_BUFFER, 0);
                }

                Self {
                    handle,
                    vertex_format,
                    vertices_count,
                }
            }
        }
    }

    pub fn update(&self, vertices: &[f32]) {
        match self.vertex_format {
            VertexFormat::DynamicF32_2_2 => {
                assert_eq!(
                    (6 * 4),
                    vertices.len(),
                    "Passing invalid format/size of vertices to update Vbo.",
                );

                unsafe {
                    glBindBuffer(GL_ARRAY_BUFFER, self.handle);
                    glBufferData(
                        GL_ARRAY_BUFFER,
                        (vertices.len() * size_of::<f32>()) as i64,
                        vertices.as_ptr() as *const GLvoid,
                        GL_DYNAMIC_DRAW,
                    );

                    glBindBuffer(GL_ARRAY_BUFFER, 0);
                }
            }

            _ => panic!("Vbo Trying to update static vertex buffer"),
        }
    }
}

impl Drop for Vbo {
    fn drop(&mut self) {
        unsafe { glDeleteBuffers(1, &self.handle) };
    }
}

#[derive(Debug)]
pub struct Vao {
    pub handle: gl::GLuint,
    pub vertices_count: gl::GLsizei,
}

impl Vao {
    pub fn init() -> Self {
        unsafe {
            let mut handle = 0;
            glGenVertexArrays(1, &mut handle);
            Self {
                handle,
                vertices_count: 0,
            }
        }
    }

    pub fn bind_vbo(&mut self, vbo: &Vbo) {
        match vbo.vertex_format {
            VertexFormat::StaticF32_3_2_3 => {
                unsafe {
                    glBindVertexArray(self.handle);
                    {
                        glBindBuffer(GL_ARRAY_BUFFER, vbo.handle);
                        // pos uv norm
                        // 3   2  3
                        glVertexAttribPointer(
                            0,
                            3,
                            GL_FLOAT,
                            GL_FALSE as u8,
                            (8 * size_of::<f32>()) as i32,
                            null::<c_void>(), // 0 as *const c_void,
                        );
                        glEnableVertexAttribArray(0);

                        glVertexAttribPointer(
                            1,
                            2,
                            GL_FLOAT,
                            GL_FALSE as u8,
                            (8 * size_of::<f32>()) as i32,
                            (3 * size_of::<f32>()) as *const c_void,
                        );
                        glEnableVertexAttribArray(1);

                        glVertexAttribPointer(
                            2,
                            3,
                            GL_FLOAT,
                            GL_FALSE as u8,
                            (8 * size_of::<f32>()) as i32,
                            (5 * size_of::<f32>()) as *const c_void,
                        );
                        glEnableVertexAttribArray(2);
                    }
                    glBindVertexArray(0);
                }

                self.vertices_count += vbo.vertices_count;
            }

            VertexFormat::DynamicF32_2_2 => {
                unsafe {
                    glBindVertexArray(self.handle);
                    glBindBuffer(GL_ARRAY_BUFFER, vbo.handle);
                    {
                        // pos uv
                        // 2   2
                        glVertexAttribPointer(
                            0,
                            4,
                            GL_FLOAT,
                            GL_FALSE as u8,
                            (4 * size_of::<f32>()) as i32,
                            null::<c_void>(), // 0 as *const c_void,
                        );
                        glEnableVertexAttribArray(0);
                    }
                    glBindVertexArray(0);
                }

                self.vertices_count += vbo.vertices_count;
            }
        }
    }

    pub fn draw_triangles(&self) {
        unsafe {
            glBindVertexArray(self.handle);
            glDrawArrays(GL_TRIANGLES, 0, self.vertices_count);
            glBindVertexArray(0);
        };
    }
}

impl Drop for Vao {
    fn drop(&mut self) {
        unsafe { glDeleteVertexArrays(1, &self.handle) };
    }
}

#[derive(Debug)]
pub struct FrameBuffer {
    // TODO
}

impl FrameBuffer {
    // TODO
}

fn gl_get_error<'a>() -> Option<&'a str> {
    unsafe {
        match glGetError() {
            GL_NO_ERROR => None,
            GL_INVALID_ENUM => Some("GL_INVALID_ENUM: An unacceptable value is specified for an enumerated argument. The offending command is ignored and has no other side effect than to set the error flag."),
            GL_INVALID_VALUE => Some("GL_INVALID_VALUE: A numeric argument is out of range. The offending command is ignored and has no other side effect than to set the error flag."),
            GL_INVALID_OPERATION => Some("GL_INVALID_OPERATION: The specified operation is not allowed in the current state. The offending command is ignored and has no other side effect than to set the error flag."),
            GL_INVALID_FRAMEBUFFER_OPERATION => Some("GL_INVALID_FRAMEBUFFER_OPERATION: The framebuffer object is not complete. The offending command is ignored and has no other side effect than to set the error flag."),
            GL_OUT_OF_MEMORY => Some("GL_OUT_OF_MEMORY: There is not enough memory left to execute the command. The state of the GL is undefined, except for the state of the error flags, after this error is recorded."),

            _ => unreachable!(),
        }
    }
}

pub fn gl_drain_errors() {
    let mut e = gl_get_error();
    while e.is_some() {
        println!("ERROR: {}", e.unwrap());
        e = gl_get_error();
    }
}
