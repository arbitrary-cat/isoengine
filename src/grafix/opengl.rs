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

use gl::types::*;
use gl;
use png;

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

    pub fn bind_to_unit(&self, unit: usize) {
        unsafe {
            trace!(gl::ActiveTexture(gl::TEXTURE0 + (unit as GLenum)));
            trace!(gl::BindTexture(gl::TEXTURE_2D, self.0));
        }
    }
}
