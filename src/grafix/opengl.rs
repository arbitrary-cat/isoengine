// Copyright (c) 2015, Sam Payson
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and
// associated documentation files (the "Software"), to deal in the Software without restriction,
// including without limitation the rights to use, copy, modify, merge, publish, distribute,
// sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or
// substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT
// NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
// DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

use std::ffi;
use std::iter;
use std::ptr;

use gl::types::*;
use gl;
use png;
use sdl2::video;

/// A RAII container for a window and its OpenGL context. This object needs to be around for as long
/// as OpenGL is being used with that window.
#[allow(dead_code)] // The code isn't really dead, we're relying on drop being called.
pub struct Context {
    window: video::Window,
    gl_ctx: video::GLContext,
}

impl Context {
    /// Create a new window with an associated (thread-local) OpenGL context.
    pub fn new(title: &str, x_res: i32, y_res: i32) -> Result<Context, String> {
        use sdl2::video::{Window, OPENGL};
        use sdl2::video::WindowPos::*;

        let window = try!(Window::new(title, PosCentered, PosCentered, x_res, y_res, OPENGL));
        let gl_ctx = try!(window.gl_create_context());

        Ok(Context{ window: window, gl_ctx: gl_ctx })
    }
}

// This function calls glGetError and returns a suffix string describing any error found. It is
// intended 100% for debug purposes, and should only be called from the trace!(..) macro.
unsafe fn error_suffix() -> &'static str {
    match gl::GetError() {
        gl::NO_ERROR                      => "",
        gl::INVALID_ENUM                  => " : GL_INVALID_ENUM",
        gl::INVALID_VALUE                 => " : GL_INVALID_VALUE",
        gl::INVALID_OPERATION             => " : GL_INVALID_OPERATION",
        gl::INVALID_FRAMEBUFFER_OPERATION => " : GL_INVALID_FRAMEBUFFER_OPERATION",
        gl::OUT_OF_MEMORY                 => " : GL_OUT_OF_MEMORY",
        _                                 => " : unknown error",
    }
}

// If `trace_gl` is enabled, this macro will print the expression passed to it (assumed to be a call
// to an OpenGL function), and then call `glGetError` and print any error it finds.
//
// Enabling `trace_gl` will slow down code a lot, but provide a detailed view of what's going on in
// the GL.
macro_rules! trace {
    ($call:expr) => (if cfg!(trace_gl) {
        let __result = $call;
        println!("{}{}", stringify!($call), error_suffix());
        __result
    } else {
        $call
    })
}
/// A 2D OpenGL Texture
pub struct Tex2D(GLuint);

impl Tex2D {
    /// Create a `Tex2D` from a PNG.
    ///
    /// # Panics
    ///
    /// This function will panic if `img` is not either BW (`K8`), RGB (`RGB8`), or RGBA (`RGBA8`).
    pub fn from_png(img: &png::Image) -> Tex2D {
        use png::PixelsByColorType::*;

        let mut gl_texid = 0;
        unsafe {
            trace!(gl::GenTextures(1, &mut gl_texid));
            trace!(gl::BindTexture(gl::TEXTURE_2D, gl_texid));
            trace!(gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_BORDER as GLint));
            trace!(gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_BORDER as GLint));
            trace!(gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as GLint));
            trace!(gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as GLint));
        }

        match img.pixels {

            RGBA8(ref pix) => unsafe {
                trace!(gl::TexImage2D(
                    gl::TEXTURE_2D,
                    0,
                    gl::RGBA as GLint,
                    img.width  as GLsizei,
                    img.height as GLsizei,
                    0,
                    gl::RGBA,
                    gl::UNSIGNED_BYTE,
                    pix.as_ptr() as *const GLvoid,
                ));
            },

            RGB8(ref pix) => unsafe {
                trace!(gl::TexImage2D(
                    gl::TEXTURE_2D,
                    0,
                    gl::RGB as GLint,
                    img.width  as GLsizei,
                    img.height as GLsizei,
                    0,
                    gl::RGB,
                    gl::UNSIGNED_BYTE,
                    pix.as_ptr() as *const GLvoid,
                ));
            },

            K8(ref pix) => unsafe {
                trace!(gl::TexImage2D(
                    gl::TEXTURE_2D,
                    0,
                    gl::RED as GLint,
                    img.width  as GLsizei,
                    img.height as GLsizei,
                    0,
                    gl::RED,
                    gl::UNSIGNED_BYTE,
                    pix.as_ptr() as *const GLvoid,
                ));
            },

            _ => panic!("PNGs must be either BW, RGB or RGBA!"),

        }

        Tex2D(gl_texid)
    }

    /// Bind this texture to `GL_TEXTURE_2D` for the given texture unit. This function results in a
    /// single call to `glActiveTexture` followed by a single call to `glBindTexture`.
    pub fn bind_to_unit(&self, unit: usize) {
        unsafe {
            trace!(gl::ActiveTexture(gl::TEXTURE0 + (unit as GLenum)));
            trace!(gl::BindTexture(gl::TEXTURE_2D, self.0));
        }
    }
}

#[allow(missing_docs)]
pub enum ShaderError {
    CompileError(String),
    LinkError(String),
}

/// A compiled OpenGL shader object. Its only purpose is to be linked with other `Shader`s into a
/// `ShaderProgram`.
pub struct Shader(GLuint);

impl Shader {
    /// Create a new vertex shader from a source string.
    pub fn new_vertex(src: &str) -> Result<Shader, ShaderError> {
        Shader::new(src, gl::VERTEX_SHADER)
    }

    /// Create a new geometry shader from a source string.
    pub fn new_geometry(src: &str) -> Result<Shader, ShaderError> {
        Shader::new(src, gl::GEOMETRY_SHADER)
    }

    /// Create a new fragment shader from a source string.
    pub fn new_fragment(src: &str) -> Result<Shader, ShaderError> {
        Shader::new(src, gl::FRAGMENT_SHADER)
    }

    // Hooray for FFI and boilerplate!
    fn new(src: &str, typ: GLenum) -> Result<Shader, ShaderError> {
        unsafe {
            let gl_shader = trace!(gl::CreateShader(typ));

            // Jump through hoops to create a `const GLchar **` for glShaderSource.
            let src_cstr    = ffi::CString::new(src).unwrap();
            let src_ptr_ptr = &src_cstr.as_ptr() as *const *const GLchar;

            trace!(gl::ShaderSource(gl_shader, 1, src_ptr_ptr, ptr::null()));
            trace!(gl::CompileShader(gl_shader));

            // Check if the shader compile successfully
            let mut status: GLint = 0;
            trace!(gl::GetShaderiv(gl_shader, gl::COMPILE_STATUS, &mut status));

            // If the shader failed to compile, get the info log and return it as an error.
            if status != (gl::TRUE as GLint) {
                let mut log_len = 0;
                trace!(gl::GetShaderiv(gl_shader, gl::INFO_LOG_LENGTH, &mut log_len));
                let mut log_buf: Vec<u8> = iter::repeat(0u8).take(log_len as usize).collect();
                let log_ptr = log_buf.as_mut_ptr() as *mut GLchar;

                let mut real_len = 0;
                trace!(gl::GetShaderInfoLog(gl_shader, log_len as GLsizei, &mut real_len, log_ptr));
                // real_len doesn't include the null terminator.
                log_buf.truncate(real_len as usize);

                let log = String::from_utf8(log_buf)
                    .unwrap_or(String::from_str("Info log was not valid utf-8"));

                Err(ShaderError::CompileError(log))
            } else {
                Ok(Shader(gl_shader))
            }
        }
    }
}

/// A linked OpenGL shader program object.
pub struct ShaderProgram(GLuint);

impl ShaderProgram {
    /// Link several `Shader`s into a `ShaderProgram`.
    pub fn from_shaders<I>(shaders: I) -> Result<ShaderProgram, ShaderError>
        where I: Iterator<Item = Shader> {

        let gl_prog = unsafe { trace!(gl::CreateProgram()) };

        for s in shaders {
            unsafe { trace!(gl::AttachShader(gl_prog, s.0)) };
        }

        unsafe {
            trace!(gl::LinkProgram(gl_prog));

            // Check if the program linked successfully.
            let mut status: GLint = 0;
            trace!(gl::GetProgramiv(gl_prog, gl::LINK_STATUS, &mut status));

            // If the program failed to link, get the info log and return it as an error.
            if status != (gl::TRUE as GLint) {
                let mut log_len = 0;
                trace!(gl::GetProgramiv(gl_prog, gl::INFO_LOG_LENGTH, &mut log_len));
                let mut log_buf: Vec<u8> = iter::repeat(0u8).take(log_len as usize).collect();
                let log_ptr = log_buf.as_mut_ptr() as *mut GLchar;

                let mut real_len = 0;
                trace!(gl::GetProgramInfoLog(gl_prog, log_len as GLsizei, &mut real_len, log_ptr));
                // real_len doesn't include the null terminator.
                log_buf.truncate(real_len as usize);

                let log = String::from_utf8(log_buf)
                    .unwrap_or(String::from_str("Info log was not valid utf-8"));

                Err(ShaderError::LinkError(log))
            } else {
                Ok(ShaderProgram(gl_prog))
            }
        }
    }

    /// Simple wrapper for `glUseProgram`.
    pub fn use_program(&self) {
        unsafe { gl::UseProgram(self.0) }
    }
}
