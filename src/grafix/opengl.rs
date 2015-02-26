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

// This function calls glGetError and returns a string describing any error found.
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

macro_rules! __expr {
    ($e:expr) => $e
}

macro_rules! gl_trace {
    ($call:expr) => (if cfg!(trace_gl) {
        let __result = $call;
        println!("{}{}", stringify!($call), error_suffix());
        __result
    })
}

pub struct Tex2D(GLuint);

impl Tex2D {
    pub fn new() -> Tex2D {
        let mut gl_texid = 0;
        println!("glGenTextures(1, <ptr>)");
        unsafe { gl_trace!(gl::GenTextures(1, &mut gl_texid)); }
        Tex2D(gl_texid)
    }
}
