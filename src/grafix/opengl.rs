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

use std::convert::From;
use std::ffi;
use std::iter;
use std::ptr;
use std::mem;

use gl::types::*;
use gl;
use png;
use sdl2::video;

// If `trace_gl` is enabled, this macro will print the expression passed to it (assumed to be a call
// to an OpenGL function), and then call `glGetError` and print any error it finds.
//
// Enabling `trace_gl` will slow down code a lot, but provide a detailed view of what's going on in
// the GL.
macro_rules! trace {
    ($call:expr) => (if cfg!(feature = "trace_gl") {
        let __result = $call;
        println!("{}{}", stringify!($call), error_suffix());
        __result
    } else {
        $call
    })
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
        _                                 => " : unrecognized error",
    }
}

/// A RAII container for a window and its OpenGL context. This object needs to be around for as long
/// as OpenGL is being used with that window.
///
/// In general the user shouldn't touch this and should instead use grafix::Context, which takes
/// care of some SDL preliminaries. Maybe I'll jump through the necessary hoops to make this
/// not-public at some point.
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

        gl::load_with(|s| unsafe { mem::transmute(video::gl_get_proc_address(s)) });

        unsafe {
            trace!(gl::Enable(gl::DEPTH_TEST));
            trace!(gl::DepthFunc(gl::LEQUAL));
            trace!(gl::ClearDepth(1.0));

            trace!(gl::Enable(gl::BLEND));
            trace!(gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA));
        }

        Ok(Context{ window: window, gl_ctx: gl_ctx })
    }

    /// Swap OpenGL buffers, drawing the frame to the screen.
    pub fn draw_frame(&self) {
        self.window.gl_swap_window();
    }
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

impl Drop for Tex2D {
    /// Call `glDeleteTextures` on this texture.
    fn drop(&mut self) {
        unsafe { trace!(gl::DeleteTextures(1, &self.0)) }
    }
}

/// An error that occurred while compiling a shader.
#[derive(Debug)]
pub struct CompileError {
    /// The info log retrieved from OpenGL, which may describe the cause of the error.
    pub info_log: String
}

/// An error that occurred while linking a shader program.
#[derive(Debug)]
pub struct LinkError {
    /// The info log retrieved from OpenGL, which may describe the cause of the error.
    pub info_log: String
}

/// A compiled OpenGL shader object. Its only purpose is to be linked with other `Shader`s into a
/// `ShaderProgram`.
pub struct Shader(GLuint);

impl Shader {
    /// Create a new vertex shader from a source string.
    pub fn new_vertex(src: &str) -> Result<Shader, CompileError> {
        Shader::new(src, gl::VERTEX_SHADER)
    }

    /// Create a new geometry shader from a source string.
    pub fn new_geometry(src: &str) -> Result<Shader, CompileError> {
        Shader::new(src, gl::GEOMETRY_SHADER)
    }

    /// Create a new fragment shader from a source string.
    pub fn new_fragment(src: &str) -> Result<Shader, CompileError> {
        Shader::new(src, gl::FRAGMENT_SHADER)
    }

    // Hooray for FFI and boilerplate!
    fn new(src: &str, typ: GLenum) -> Result<Shader, CompileError> {
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
                    .unwrap_or(From::from("Info log was not valid utf-8"));

                Err(CompileError{info_log: log})
            } else {
                Ok(Shader(gl_shader))
            }
        }
    }
}

impl Drop for Shader {
    /// Call `glDeleteShader` on this shader. Shaders should be dropped as soon as possible after
    /// linking, since they keep unnecessary source and object code around in GL memory.
    fn drop(&mut self) {
        unsafe { trace!(gl::DeleteShader(self.0)) }
    }
}

/// A TransformFeedback buffer, at the moment this is just intended for debugging purposes. I don't
/// see them being very useful for anything else in a 2D isometric engine.
pub struct TransformFeedback<T: Clone>{
    name: GLuint,
    data: Vec<T>
}

impl<T: Clone> TransformFeedback<T> {

    /// Create a new TransformFeedback buffer with room for `count` vertices. Initialize each of
    /// these vertices with `init` in main memory (but not on the GPU).
    pub fn new(count: usize, init: T) -> TransformFeedback<T> {
        let gl_xfb = unsafe {
            let mut gl_xfb = 0;

            trace!(gl::GenBuffers(1, &mut gl_xfb));

            trace!(gl::BindBuffer(gl::ARRAY_BUFFER, gl_xfb));

            trace!(gl::BufferData(
                gl::ARRAY_BUFFER,
                (mem::size_of::<T>() * count) as GLsizeiptr,
                ptr::null(),
                gl::STATIC_READ
            ));

            gl_xfb
        };

        TransformFeedback {
            name: gl_xfb,
            data: iter::repeat(init).take(count).collect(),
        }
    }

    /// Read the contents of the buffer into main memory and a reference to them. This buffer must
    /// be bound before calling read.
    pub fn read<'x>(&'x mut self) -> &'x [T] {
        unsafe {
            trace!(gl::GetBufferSubData(
                gl::TRANSFORM_FEEDBACK_BUFFER,
                0,
                (mem::size_of::<T>() * self.data.len()) as GLsizeiptr,
                self.data.as_ptr() as *mut GLvoid
            ));
        }

        &self.data
    }

    /// Bind this buffer to `TRANSFORM_FEEDBACK_BUFFER`.
    pub fn bind(&self) {
        unsafe { trace!(gl::BindBufferBase(gl::TRANSFORM_FEEDBACK_BUFFER, 0, self.name)) }
    }
}

/// A linked OpenGL shader program object.
pub struct ShaderProgram(GLuint);

impl ShaderProgram {
    /// Link several `Shader`s into a `ShaderProgram`.
    pub fn new(shaders: &[Shader]) -> Result<ShaderProgram, LinkError> {

        Ok( ShaderProgram( unsafe {
            let gl_prog = trace!(gl::CreateProgram());

            for s in shaders.iter() {
                // Attach shaders for linking.
                trace!(gl::AttachShader(gl_prog, s.0));
            }

            try!(ShaderProgram::link(gl_prog));

            for s in shaders.iter() {
                // Detach shaders so they can be deleted.
                trace!(gl::DetachShader(gl_prog, s.0))
            }

            gl_prog
        }))
    }

    /// Link several `Shader`s into a `ShaderProgram`, capturing the given attributes in a transform
    /// feedback buffer.
    pub fn new_xfb(shaders: &[Shader], xfb_attrs: &[&str]) -> Result<ShaderProgram, LinkError> {

        Ok( ShaderProgram( unsafe {
            let gl_prog = trace!(gl::CreateProgram());

            for s in shaders.iter() {
                // Attach shaders for linking.
                trace!(gl::AttachShader(gl_prog, s.0));
            }

            let cstr_xfb_attrs: Vec<ffi::CString> = xfb_attrs.iter()
                .map(|&rstr| ffi::CString::new(rstr).unwrap())
                .collect();

            let cptr_xfb_attrs: Vec<*const GLchar> = cstr_xfb_attrs.iter()
                .map(|cstr| cstr.as_ptr())
                .collect();

            trace!(gl::TransformFeedbackVaryings(gl_prog, cptr_xfb_attrs.len() as GLsizei,
                cptr_xfb_attrs.as_ptr(), gl::INTERLEAVED_ATTRIBS));

            try!(ShaderProgram::link(gl_prog));

            for s in shaders.iter() {
                // Detach shaders so they can be deleted.
                trace!(gl::DetachShader(gl_prog, s.0))
            }

            gl_prog
        }))
    }

    unsafe fn link(gl_prog: GLuint) -> Result<(), LinkError> {

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
                .unwrap_or(From::from("Info log was not valid utf-8"));

            return Err(LinkError{info_log: log});
        }

        Ok(())
    }

    /// Simple wrapper for `glUseProgram`.
    pub fn use_program(&self) {
        unsafe { trace!(gl::UseProgram(self.0)) }
    }

    /// Get a `VertexAttrib` corresponding to one of the active vertex attributes in this
    /// `ShaderProgram`..
    pub fn get_attrib(&self, name: &str) -> Result<VertexAttrib, NoSuchActiveAttrib> {
        let attrib = unsafe {
            let cname = ffi::CString::new(name).unwrap();

            trace!(gl::GetAttribLocation(self.0, cname.as_ptr() as *const GLchar))
        };

        if attrib == -1 {
            Err(NoSuchActiveAttrib(From::from(name)))
        } else {
            Ok(VertexAttrib(attrib as GLuint))
        }
    }

    /// Get a `Uniform` corresponding to one of the active uniforms in this `ShaderProgram`.
    pub fn get_uniform(&self, name: &str) -> Result<Uniform, NoSuchActiveUniform> {
        let gl_uniform = unsafe {
            let cname = ffi::CString::new(name).unwrap();

            trace!(gl::GetUniformLocation(self.0, cname.as_ptr() as *const GLchar))
        };

        if gl_uniform == -1 {
            Err(NoSuchActiveUniform(From::from(name)))
        } else {
            Ok(Uniform(gl_uniform))
        }
    }
}

impl Drop for ShaderProgram {
    /// Call `glDeleteProgram` on this shader program.
    fn drop(&mut self) {
        unsafe { trace!(gl::DeleteProgram(self.0)) }
    }
}

/// Simplified interface to OpenGL's Vertex Array Objects.
pub struct VertexArray(GLuint);

impl VertexArray {
    /// Generate a new `VertexArray`
    pub fn new() -> VertexArray {
        let mut gl_vao = 0;
        unsafe { trace!(gl::GenVertexArrays(1, &mut gl_vao)) }

        VertexArray(gl_vao)
    }

    /// Call `glBindVertexArray` on this `VertexArray`.
    pub fn bind(&self) {
        unsafe { trace!(gl::BindVertexArray(self.0)) }
    }
}

impl Drop for VertexArray {
    /// Call `glDeleteVertexArrays` on this Vertex Array Object.
    fn drop(&mut self) {
        unsafe { trace!(gl::DeleteVertexArrays(1, &self.0)) }
    }
}

/// Simplified, safer interface to OpenGL's Vertex Buffer Objects.
pub struct VertexBuffer(GLuint);

impl VertexBuffer {
    /// Generate a new `VertexBuffer` and allocate `size` bytes of storage on the GPU. The buffer
    /// will be created with the `STREAM_DRAW` usage constant.
    pub fn new(size: usize) -> VertexBuffer {
        let mut gl_vbo = 0;
        unsafe {
            trace!(gl::GenBuffers(1, &mut gl_vbo));
            trace!(gl::BindBuffer(gl::ARRAY_BUFFER, gl_vbo));
            trace!(gl::BufferData(
                gl::ARRAY_BUFFER,
                size as GLsizeiptr,
                ptr::null(),
                gl::STREAM_DRAW,
            ));
        }

        VertexBuffer(gl_vbo)
    }

    /// Make this the active Vertex Buffer. This amounts to calling `glBindBuffer` with the
    /// `ARRAY_BUFFER` target constant.
    pub fn bind(&self) {
        unsafe { trace!(gl::BindBuffer(gl::ARRAY_BUFFER, self.0)) }
    }

    /// Load data into the buffer, it must not be larger than the size of the buffer.
    pub fn buffer_data<T>(&self, data: &[T]) {
        unsafe {
            trace!(gl::BindBuffer(gl::ARRAY_BUFFER, self.0));
            trace!(gl::BufferSubData(
                gl::ARRAY_BUFFER,
                0, // Offset is always 0.
                (mem::size_of::<T>() * data.len()) as GLsizeiptr,
                data.as_ptr() as *const GLvoid,
            ));
        }
    }

    /// Get the OpenGL name of this buffer.
    pub fn dbg_name(&self) -> GLint { self.0 as GLint }
}

impl Drop for VertexBuffer {
    /// Call `glDeleteBuffers` on this Vertex Buffer Object.
    fn drop(&mut self) {
        unsafe { trace!(gl::DeleteBuffers(1, &self.0)) }
    }
}

/// Error returned to indicate that the requested attribute does not exist (or that the user has
/// requested the location of a built-in attributed beginning with `gl_`).
#[derive(Debug)]
pub struct NoSuchActiveAttrib(pub String);

/// Simplified interface to an OpenGL Vertex Attribute.
pub struct VertexAttrib(GLuint);

impl VertexAttrib {
    /// A somewhat rustier interface to `glVertexAttribPointer`. It's still a mess, though.
    pub fn set_pointer(&self, size: usize, typ: GLenum, norm: bool, stride: usize, off: usize) {
        unsafe {
            trace!(gl::VertexAttribPointer(
                self.0,
                size as GLint,
                typ,
                match norm { true => gl::TRUE, false => gl::FALSE },
                stride as GLsizei,
                mem::transmute(off)
            ));
        }
    }

    /// Enable this vertex attribute.
    pub fn enable(&self) {
        unsafe { trace!(gl::EnableVertexAttribArray(self.0)) }
    }

    /// Returns the name of the buffer object currently bound to this attributes binding point.
    pub fn dbg_buffer_binding(&self) -> GLint {
        let mut gl_name = 0;
        unsafe { gl::GetVertexAttribiv(self.0, gl::VERTEX_ATTRIB_ARRAY_BUFFER_BINDING, &mut gl_name) }
        gl_name
    }
}

/// Error returned to indicate that the requested attribute does not exist (or that the user has
/// requested the location of a built-in attributed beginning with `gl_`).
#[derive(Debug)]
pub struct NoSuchActiveUniform(pub String);

/// A uniform variable from a shader program.
pub struct Uniform(GLint);

impl Uniform {
    /// Call glUniform1i on the underlying uniform. The corresponding program must be active in
    /// order for this to work as expected.
    pub fn set1i(&self, x: i32) {
        unsafe { trace!(gl::Uniform1i(self.0, x as GLint)) }
    }
}
