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

#![feature(collections, core, non_ascii_idents, old_path, std_misc)]

#![warn(missing_docs)]

//! An Encumbered and Closed Source 2D isometric game engine.

extern crate gl;
extern crate png;
extern crate sdl2;

extern crate core;

#[macro_use] extern crate mkprim;

#[macro_use] extern crate bitflags;

/// Vector math.
#[macro_use]
pub mod math;

/// Units used throughout the engine.
pub mod units;

/// High-level graphics abstractions built on top of OpenGL.
#[macro_use] pub mod grafix;

/// Abstractions for dealing with time.
pub mod time;

/// The scene abstraction which arranges entities and effects on the map.
pub mod scene;

use grafix::opengl;

/// A RAII handle for the whole engine. Once this baby leaves scope, it's curtains.
///
/// ...I'm tired, okay?
#[allow(dead_code)]
pub struct Context {
    gfx: opengl::Context,
    sdl: sdl2::Sdl,
}

impl Context {
    /// Create a new isoengine context. This will create a window and an OpenGL context, as well as
    /// initialize all SDL subsystems.
    pub fn new(title: &str, x_res: i32, y_res: i32) -> Result<Context, String> {
        let sdl = try!(sdl2::init(sdl2::INIT_EVERYTHING));
        let gfx = try!(opengl::Context::new(title, x_res, y_res));

        Ok(Context { sdl: sdl, gfx: gfx })
    }

    /// Swap OpenGL buffers, drawing the frame to the screen.
    pub fn draw_frame(&self) {
        self.gfx.draw_frame();
    }

    /// A debug method to get the sdl. Really this is to work around Ubuntu fading the fucking
    /// window when I'm not polling for events. Yes, it's very helpful when you make it difficult
    /// for me to see what's going on in a window that "isn't responding". Thanks Ubuntu.
    pub fn dbg_get_sdl(&self) -> &sdl2::Sdl { &self.sdl }
}
