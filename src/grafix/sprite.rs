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

use std::collections::HashMap;
use std::error::FromError;
use std::mem;
use std::old_path::Path;

use gl;
use png;

use grafix::math;
use grafix::opengl;
use grafix::units::*;

/// A descriptor which explains the properties of a sprite sheet and where to find the textures.
pub struct SheetDesc {
    /// Width of the texture, in texels.
    pub img_width:  u32,

    /// Height of the texture, in texels.
    pub img_height: u32,

    /// X-coordinate of origin pixel.
    pub origin_x: u32,

    /// Y-coordinate of origin pixel.
    pub origin_y: u32,

    /// Width of each sprite, in texels.
    pub spr_width: u32,

    /// Height of each sprite, in texels.
    pub spr_height: u32,

    /// Number of sprites in each row in the sheet.
    pub num_across: u32,

    /// Number of sprites in each column in the sheet.
    pub num_down: u32,

    /// Total number of sprites in the sheet.
    pub total: u32,

    /// Path to the color PNG for this sprite sheet.
    pub color_path: String,

    /// Path to the depth PNG for this sprite sheet.
    pub depth_path: String,
}

/// A sprite sheet.
pub struct Sheet {
    // Dimensions of the whole image.
    img_dimens: math::Vec2<Pixels>,

    // Position of a sprite's origin as a ratio of width and height.
    origin: math::Vec2<Pixels>,

    // Dimensions of a sprite in pixels.
    scr_dimens: math::Vec2<Pixels>,

    // Dimensions of a sprite in texture coordinates (i.e. as a ration of the whole image's size).
    tex_dimens: math::Vec2<TexCoord>,

    // Number of sprites in each row/column of the sheet. There may be 'slack' along the right side
    // or bottom of the texture, if `scr_dimens` doesn't evenly divide `img_dimens`.
    num_across: usize,
    num_down:   usize,

    // Number of sprites in the sheet (num_across * (num_down - 1) < total <= num_across * num_down)
    total: usize,

    // RGBA texture which gives the sprite its color.
    color: opengl::Tex2D,

    // Red texture which gives each pixels distance from the camera, at render time.
    depth: opengl::Tex2D,
}

impl Sheet {
    /// Load a `Sheet` from a descriptor. This turns the paths in the `SheetDesc` into OpenGL
    /// textures.
    pub fn from_desc(desc: SheetDesc) -> Result<Sheet, Error> {
        let color_path = Path::new(desc.color_path);
        let depth_path = Path::new(desc.depth_path);

        let color_png = try!(png::load_png(&color_path).map_err(Error::PngError));
        let depth_png = try!(png::load_png(&depth_path).map_err(Error::PngError));

        Ok( Sheet {
            img_dimens: vec2!(
                Pixels(desc.img_width as f32),
                Pixels(desc.img_height as f32),
            ),

            origin: vec2!(
                Pixels(desc.origin_x as f32),
                Pixels(desc.origin_y as f32),
            ),

            scr_dimens: vec2!(
                Pixels(desc.spr_width as f32),
                Pixels(desc.spr_height as f32),
            ),

            tex_dimens: vec2!(
                TexCoord((desc.spr_width as f32)  / (desc.img_width as f32)),
                TexCoord((desc.spr_height as f32) / (desc.img_height as f32)),
            ),

            num_across: desc.num_across as usize,
            num_down:   desc.num_down as usize,

            total: desc.total as usize,

            color: opengl::Tex2D::from_png(&color_png),
            depth: opengl::Tex2D::from_png(&depth_png),
        })
    }
}

/// This is the vertex type that is sent to the GPU
#[allow(non_snake_case)]
struct SpriteVertex {
    // Corners of the sprite
    screen_TL: math::Vec2<NDU>,
    screen_BR: math::Vec2<NDU>,

    // Corners of the texture
    tex_TL: math::Vec2<TexCoord>,
    tex_BR: math::Vec2<TexCoord>,

    // Depth of the origin of the sprite from the camera. In meters, since that's the unit used in
    // the depth texture.
    depth: Meters,
}

/// Struct which encapsulates the GL state needed to render sprites.
pub struct Renderer {
    prog: opengl::ShaderProgram,
    vao:  opengl::VertexArray,
    vbo:  opengl::VertexBuffer,
}

macro_rules! attrib_offset {
    ($attr:ident) => ( unsafe {
        let base: &SpriteVertex = mem::transmute(0usize);
        let offs: usize = mem::transmute(&base.$attr);

        offs
    })
}

impl Renderer {
    /// Create a new `sprite::Renderer`. This compiles and links a shader program, so it should only
    /// be called after OpenGL has been initialized.
    #[allow(non_snake_case)]
    pub fn new() -> Result<Renderer, Error> {
        let vtx = try!(opengl::Shader::new_vertex(include_str!("shaders/sprite.vtx")));
        let geo = try!(opengl::Shader::new_geometry(include_str!("shaders/sprite.geo")));
        let frg = try!(opengl::Shader::new_fragment(include_str!("shaders/sprite.frg")));

        let prog = try!(opengl::ShaderProgram::from_shaders(&[vtx, geo, frg]));

        // All of the attribute state will be stored in this VAO.
        let vao = opengl::VertexArray::new();
        vao.bind();

        let screen_TL = try!(prog.get_attrib("screen_TL"));
        let screen_BR = try!(prog.get_attrib("screen_BR"));

        let tex_TL = try!(prog.get_attrib("tex_TL"));
        let tex_BR = try!(prog.get_attrib("tex_BR"));

        let depth = try!(prog.get_attrib("depth"));

        screen_TL.set_pointer(2, gl::FLOAT, false, mem::size_of::<SpriteVertex>(),
            attrib_offset!(screen_TL));

        screen_BR.set_pointer(2, gl::FLOAT, false, mem::size_of::<SpriteVertex>(),
            attrib_offset!(screen_BR));

        tex_TL.set_pointer(2, gl::FLOAT, false, mem::size_of::<SpriteVertex>(),
            attrib_offset!(tex_TL));

        tex_BR.set_pointer(2, gl::FLOAT, false, mem::size_of::<SpriteVertex>(),
            attrib_offset!(tex_BR));

        depth.set_pointer(1, gl::FLOAT, false, mem::size_of::<SpriteVertex>(),
            attrib_offset!(depth));

        Ok(Renderer {
            prog: prog,
            vao:  vao,
            vbo:  opengl::VertexBuffer::new(),
        })
    }
}

/// An ID that refers to a particular `Sheet` in a `Database`.
pub type SheetID = usize;

/// Central storage for Sheets, it provides two mappings: `String -> ID` and `ID -> Sheet`.
pub struct Database {
    name2id:  HashMap<String, SheetID>,
    id2sheet: Vec<Sheet>,
}

impl Database {
    /// Create a new empty `Database`.
    pub fn new() -> Database {
        Database {
            name2id:  HashMap::new(),
            id2sheet: Vec::new(),
        }
    }

    /// Insert a sprite sheet into the `Database`.
    ///
    /// # Errors
    ///
    /// If there is already a sheet by this name than an error will be logged and `self` will be
    /// unchanged.
    pub fn insert(&mut self, name: String, sheet: Sheet) {
        if self.name2id.contains_key(&name) {
            println!("Attempt to load additional sprite sheet named `{}' ignored.", name);
            return
        }

        let id = self.id2sheet.len();

        assert_eq!(self.name2id.insert(name, id), None);
        self.id2sheet.push(sheet);
    }

    /// If there is a `Sheet` stored under `name` in the database, return its id. Otherwise return
    /// None.
    pub fn get_id(&self, name: &str) -> Option<SheetID> {
        self.name2id.get(name).cloned()
    }

    /// Get a sprite sheet from an id. If there is no sheet with that id, then None is returned. But
    /// that should never happen because you got the id by calling `self.get_id()`... right?
    pub fn get_sheet(&self, id: SheetID) -> Option<&Sheet> {
        self.id2sheet.get(id)
    }
}

/// An error encountered when loading sprites or related resources.
pub enum Error {
    /// Error loading a PNG.
    PngError(String),

    /// Error compiling a shader.
    CompileError(opengl::CompileError),

    /// Error linking a shader program.
    LinkError(opengl::LinkError),

    /// The engine and the shaders disagree about the name of an attribute.
    NoSuchActiveAttrib(String),
}

impl FromError<opengl::CompileError> for Error {
    fn from_error(err: opengl::CompileError) -> Error {
        Error::CompileError(err)
    }
}

impl FromError<opengl::LinkError> for Error {
    fn from_error(err: opengl::LinkError) -> Error {
        Error::LinkError(err)
    }
}

impl FromError<opengl::NoSuchActiveAttrib> for Error {
    fn from_error(err: opengl::NoSuchActiveAttrib) -> Error {
        match err {
            opengl::NoSuchActiveAttrib(id) => Error::NoSuchActiveAttrib(id),
        }
    }
}
