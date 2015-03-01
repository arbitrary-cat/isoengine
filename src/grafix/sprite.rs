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
use std::old_path::Path;

use png;

use grafix::opengl;

/// A descriptor which explains the properties of a sprite sheet and where to find the textures.
pub struct SheetDesc {
    /// Width of the texture, in texels.
    pub tex_width:  u32,

    /// Height of the texture, in texels.
    pub tex_height: u32,

    /// Width of each sprite, in texels.
    pub spr_width:  u32,

    /// Height of each sprite, in texels.
    pub spr_height: u32,

    /// Number of sprites in each row in the sheet.
    pub num_across: u32,

    /// Number of sprites in each column in the sheet.
    pub num_down:   u32,

    /// Path to the color PNG for this sprite sheet.
    pub color_path: String,

    /// Path to the depth PNG for this sprite sheet.
    pub depth_path: String,
}

/// A sprite sheet.
pub struct Sheet {
    tex_width:  u32,
    tex_height: u32,

    spr_width:  u32,
    spr_height: u32,

    num_across: u32,
    num_down:   u32,

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
            tex_width:  desc.tex_width,
            tex_height: desc.tex_height,

            spr_width:  desc.spr_width,
            spr_height: desc.spr_height,

            num_across: desc.num_across,
            num_down:   desc.num_down,

            color: opengl::Tex2D::from_png(&color_png),
            depth: opengl::Tex2D::from_png(&depth_png),
        })
    }
}



/// Struct which encapsulates the GL state needed to render sprites.
pub struct Renderer {
    prog: opengl::ShaderProgram,
}

impl Renderer {
    /// Create a new `sprite::Renderer`. This compiles and links a shader program, so it should only
    /// be called after OpenGL has been initialized.
    pub fn new() -> Result<Renderer, Error> {
        let vtx = try!(opengl::Shader::new_vertex(include_str!("shaders/sprite.vtx")));
        let geo = try!(opengl::Shader::new_geometry(include_str!("shaders/sprite.geo")));
        let frg = try!(opengl::Shader::new_fragment(include_str!("shaders/sprite.frg")));

        let prog = try!(opengl::ShaderProgram::from_shaders(&[vtx, geo, frg]));

        Ok(Renderer { prog: prog })
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

    /// Insert a sprite sheet into the `Database`. If there is already a sheet by this name than an
    /// error will be logged and `self` will be unchanged.
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
