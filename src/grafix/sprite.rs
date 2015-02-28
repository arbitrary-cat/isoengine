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

use std::old_path::{Path, BytesContainer};

use png;

use grafix::opengl;

/// A descriptor which explains the properties of a sprite sheet and where to find the textures.
pub struct SheetDesc {
    pub tex_width:  u32,
    pub tex_height: u32,

    pub spr_width:  u32,
    pub spr_height: u32,

    pub num_across: u32,
    pub num_down:   u32,

    pub color_path: String,
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
    /// Load a Sprite from a descriptor. This turns the paths in the `Sheet
    pub fn from_desc(desc: SheetDesc) -> Result<Sheet, String> {
        let color_path = Path::new(desc.color_path);
        let depth_path = Path::new(desc.depth_path);

        Ok( Sheet {
            tex_width:  desc.tex_width,
            tex_height: desc.tex_height,

            spr_width:  desc.spr_width,
            spr_height: desc.spr_height,

            num_across: desc.num_across,
            num_down:   desc.num_down,

            color: opengl::Tex2D::from_png(&try!(png::load_png(&color_path))),
            depth: opengl::Tex2D::from_png(&try!(png::load_png(&depth_path))),
        })
    }
}
