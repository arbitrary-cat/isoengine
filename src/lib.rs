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

#![warn(missing_docs)]

//! A 2D Isometric Game Engine.

#[cfg(feature = "client")] extern crate gl;
#[cfg(feature = "client")] extern crate png;
#[cfg(feature = "client")] extern crate sdl2;

extern crate flatbuffers;

extern crate num;

#[macro_use] extern crate log;

#[macro_use] extern crate mkprim;

#[macro_use] extern crate bitflags;

/// Vector math.
#[macro_use]
pub mod math;

/// Units used throughout the engine.
pub mod units;

/// High-level graphics abstractions built on top of OpenGL.
#[macro_use] pub mod grafix;

/// Code for managing assets between a server and clients.
pub mod asset;

/// Abstractions for dealing with time.
pub mod time;

/// The Entity Component System.
pub mod entity;

// Not quite ready for this yet.
// /// Systems which process entities, and tools for constructing them.
// pub mod system;

/// Code which is specific to game clients (as opposed to servers).
#[cfg(feature = "client")] pub mod client;
