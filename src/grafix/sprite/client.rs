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

use super::common::*;

use std::convert::{AsRef, From};
use std::mem;

use gl;
use gl::types::*;
use png;

use grafix::opengl;
use grafix;

use grafix::camera::Camera;
use math;
use system::db;
use units::*;

// The maximum number of sprites that can be drawn on-screen at any given time.
const MAX_SPRITES: usize = 16 * 1024;

/// A descriptor which explains the properties of a sprite sheet and where to find the textures.
pub struct SheetDesc {
    /// Width of the texture, in texels.
    pub img_width:  u16,

    /// Height of the texture, in texels.
    pub img_height: u16,

    /// X-coordinate of origin pixel.
    pub origin_x: u16,

    /// Y-coordinate of origin pixel.
    pub origin_y: u16,

    /// Width of each sprite, in texels.
    pub spr_width: u16,

    /// Height of each sprite, in texels.
    pub spr_height: u16,

    /// Number of sprites in each row in the sheet.
    pub num_across: u16,

    /// Number of sprites in each column in the sheet.
    pub num_down: u16,

    /// Total number of sprites in the sheet.
    pub total: u16,

    /// Path to the color PNG for this sprite sheet.
    pub color_path: String,

    /// Path to the depth PNG for this sprite sheet.
    pub depth_path: String,
}

impl SheetDesc {
    /// Convert from FlatBuffer representation.
    pub fn from_wire(w: &grafix::sprite::wire::SpriteSheetDesc) -> SheetDesc {
        SheetDesc {
            img_width:  w.img_width(),
            img_height: w.img_height(),
            origin_x:   w.origin_x(),
            origin_y:   w.origin_y(),
            spr_width:  w.spr_width(),
            spr_height: w.spr_height(),
            num_across: w.num_across(),
            num_down:   w.num_down(),
            total:      w.total(),
            color_path: From::from(AsRef::as_ref(w.color_path().unwrap())),
            depth_path: From::from(AsRef::as_ref(w.depth_path().unwrap())),
        }
    }
}

/// A sprite sheet.
pub struct Sheet {
    // Position of a sprite's origin as a ratio of width and height.
    origin: math::Vec2<Pixels>,

    // Dimensions of a sprite in pixels.
    scr_dimens: math::Vec2<Pixels>,

    // Dimensions of a sprite in texture coordinates (i.e. as a ration of the whole image's size).
    tex_dimens: math::Vec2<TexCoord>,

    // Number of sprites in each row of the sheet. There may be 'slack' along the right side
    // or bottom of the texture, if the sprites don't fit the texture perfectly.
    num_across: usize,

    // RGBA texture which gives the sprite its color.
    color: opengl::Tex2D,

    // Red texture which gives each pixels distance from the camera, at render time.
    depth: opengl::Tex2D,
}

impl Sheet {
    /// Load a `Sheet` from a descriptor. This turns the paths in the `SheetDesc` into OpenGL
    /// textures.
    pub fn from_desc(desc: SheetDesc) -> Result<Sheet, Error> {
        let color_png = try!(png::load_png(&desc.color_path).map_err(Error::PngError));
        let depth_png = try!(png::load_png(&desc.depth_path).map_err(Error::PngError));

        Ok( Sheet {
            origin: vec2!(Pixels ; desc.origin_x as f32, desc.origin_y as f32),

            scr_dimens: vec2!(Pixels ; desc.spr_width as f32, desc.spr_height as f32),

            tex_dimens: vec2!(TexCoord ;
                (desc.spr_width as f32)  / (desc.img_width as f32),
                (desc.spr_height as f32) / (desc.img_height as f32),
            ),

            num_across: desc.num_across as usize,

            color: opengl::Tex2D::from_png(&color_png),
            depth: opengl::Tex2D::from_png(&depth_png),
        })
    }
}

/// This is the vertex type that is sent to the GPU
#[allow(non_snake_case)]
#[derive(Debug,Copy,Clone)]
pub struct SpriteVertex {
    /// The top-left of the sprite in screen-coordinates
    pub screen_TL: math::Vec2<NDU>,

    /// The bottom-right of the sprite in screen-coordinates
    pub screen_BR: math::Vec2<NDU>,

    /// The top-left texture coordinate.
    pub tex_TL: math::Vec2<TexCoord>,

    /// The bottom-right texture coordinate.
    pub tex_BR: math::Vec2<TexCoord>,

    /// Depth of the origin of the sprite from the camera. In `Meters`, since that's the unit used
    /// in the depth texture.
    pub depth: Meters,
}

impl SpriteVertex {
    /// Return a `SpriteVertex` with all fields set to 0.0
    pub fn zero() -> SpriteVertex {
        SpriteVertex {
            screen_TL: vec2!(NDU ; 0.0, 0.0),
            screen_BR: vec2!(NDU ; 0.0, 0.0),

            tex_TL: vec2!(TexCoord ; 0.0, 0.0),
            tex_BR: vec2!(TexCoord ; 0.0, 0.0),

            depth: Meters(0.0),
        }
    }
}

/// An abstraction around the process of rendering a sprite. The `Batcher` dispatches sprites to a
/// `Renderer` to be drawn, and the `Renderer` is free to accomplish that however it wishes.
pub trait Renderer {
    /// Send `verts` to the GPU and get ready to render sprites from it (i.e. bind buffers and use
    /// programs, etc...)
    fn prepare(&self, verts: &[SpriteVertex]);

    /// Render a `RenderGroup`.
    fn render<'x>(&mut self, grp: RenderGroup<'x>);
}

macro_rules! attrib_offset {
    ($attr:ident) => ( unsafe {
        let base: &SpriteVertex = mem::transmute(0usize);
        let offs: usize = mem::transmute(&base.$attr);

        offs
    })
}

/// A group of sprites to be rendered at the same time. This struct only exists to be passed to the
/// `Renderer::render` method, and references a range of sprites passed to that `Renderer` in the
/// most recent call to `Renderer::prepare`.
pub struct RenderGroup<'x> {
    /// The index of the first sprite to be drawn.
    pub first: usize,

    /// The number of sprites to be drawn.
    pub count: usize,

    /// The sprite sheet on which these sprites reside (this just provides the textures).
    pub sheet: &'x Sheet,
}

/// A `Renderer` which has no instrumentation, and is designed for performance alone.
pub struct ReleaseRenderer {
    prog: opengl::ShaderProgram,
    vao:  opengl::VertexArray,
    vbo:  opengl::VertexBuffer,
}

impl ReleaseRenderer {
    /// Create a new `sprite::Renderer`. This compiles and links a shader program, so it should only
    /// be called after OpenGL has been initialized.
    pub fn new() -> Result<ReleaseRenderer, Error> {
        #![allow(non_snake_case)]
        let vtx = try!(opengl::Shader::new_vertex(include_str!("../shaders/sprite.vtx")));
        let geo = try!(opengl::Shader::new_geometry(include_str!("../shaders/sprite.geo")));
        let frg = try!(opengl::Shader::new_fragment(include_str!("../shaders/sprite.frg")));

        let prog = try!(opengl::ShaderProgram::new(&[vtx, geo, frg]));
        prog.use_program();

        // Allow up to 16k sprites to be drawn simultaneously, this is far too many =P.
        let vbo = opengl::VertexBuffer::new(mem::size_of::<SpriteVertex>() * MAX_SPRITES);

        let vao = try!(setup_gl_attributes(&prog));

        let color_tex = try!(prog.get_uniform("color_tex"));
        let depth_tex = try!(prog.get_uniform("depth_tex"));

        color_tex.set1i(0);
        depth_tex.set1i(1);

        Ok(ReleaseRenderer {
            prog: prog,
            vao:  vao,
            vbo:  vbo,
        })
    }

}

impl Renderer for ReleaseRenderer {
    fn prepare(&self, verts: &[SpriteVertex]) {
        self.vbo.buffer_data(verts);

        self.prog.use_program();

        self.vao.bind();

        self.vbo.bind();
    }

    fn render<'x>(&mut self, grp: RenderGroup<'x>) {
        grp.sheet.color.bind_to_unit(0);
        grp.sheet.depth.bind_to_unit(1);

        unsafe {
            gl::DrawArrays(gl::POINTS, grp.first as GLint, grp.count as GLsizei);
        }
    }
}

/// An instrumented `Renderer` which prints the output of the vertex and geometry shaders to
/// standard out.
///
/// FIXME: This doesn't actually print the output of the geometry shader yet. No good reason
///        to implement it yet =].
pub struct DebugRenderer {
    // A shader program which only runs the vertex shader, for transform feedback.
    vtx_prog: opengl::ShaderProgram,
    vtx_vao:  opengl::VertexArray,

    // Transform feedback buffer that will capture the output of the vertex shader.
    vtx_xfb: opengl::TransformFeedback<SpriteVertex>,

    // A shader program which only runs the vertex and geometry shaders, for transform feedback.
    geo_prog: opengl::ShaderProgram,
    geo_vao:  opengl::VertexArray,

    // Transform feedback buffer that will capture the output of the geometry shader.
    geo_xfb: opengl::TransformFeedback<SpriteVertex>,

    // A shader program which does the whole render.
    full_prog: opengl::ShaderProgram,
    full_vao:  opengl::VertexArray,

    vbo: opengl::VertexBuffer,
}

impl DebugRenderer {
    /// Create a new `sprite::DebugRenderer`. This compiles and links a shader program, so it should
    /// only be called after OpenGL has been initialized.
    pub fn new() -> Result<DebugRenderer, Error> {
        #![allow(non_snake_case)]

        // Allow up to 16k sprites to be drawn simultaneously, this is far too many =P.
        let vbo = opengl::VertexBuffer::new(mem::size_of::<SpriteVertex>() * MAX_SPRITES);

        let vtx_names: &[&str] = &[
            "FromVert.screen_TL",
            "FromVert.screen_BR",
            "FromVert.tex_TL",
            "FromVert.tex_BR",
            "FromVert.depth",
        ];

        let vtx = try!(opengl::Shader::new_vertex(include_str!("../shaders/sprite.vtx")));

        let vtx_prog = try!(opengl::ShaderProgram::new_xfb(&[vtx], vtx_names));
        
        let vtx_xfb = opengl::TransformFeedback::new(MAX_SPRITES, SpriteVertex::zero());

        vbo.bind();
        let vtx_vao = try!(setup_gl_attributes(&vtx_prog));

        let vtx = try!(opengl::Shader::new_vertex(include_str!("../shaders/sprite.vtx")));
        let geo = try!(opengl::Shader::new_geometry(include_str!("../shaders/sprite.geo")));

        let geo_prog = try!(opengl::ShaderProgram::new(&[vtx, geo]));

        // Each input vertex gets turned into a rectangle consisting of two triangles, so there will
        // be a total of 6 vertices per-sprite output by the geometry shader.
        let geo_xfb = opengl::TransformFeedback::new(MAX_SPRITES * 6, SpriteVertex::zero());

        vbo.bind();
        let geo_vao = try!(setup_gl_attributes(&geo_prog));

        let vtx = try!(opengl::Shader::new_vertex(include_str!("../shaders/sprite.vtx")));
        let geo = try!(opengl::Shader::new_geometry(include_str!("../shaders/sprite.geo")));
        let frg = try!(opengl::Shader::new_fragment(include_str!("../shaders/sprite.frg")));

        let full_prog = try!(opengl::ShaderProgram::new(&[vtx, geo, frg]));

        vbo.bind();
        let full_vao = try!(setup_gl_attributes(&full_prog));

        let color_tex = try!(full_prog.get_uniform("color_tex"));
        let depth_tex = try!(full_prog.get_uniform("depth_tex"));

        color_tex.set1i(0);
        depth_tex.set1i(1);

        Ok(DebugRenderer {
            vtx_prog: vtx_prog,
            vtx_vao:  vtx_vao,

            vtx_xfb: vtx_xfb,

            geo_prog: geo_prog,
            geo_vao:  geo_vao,

            geo_xfb: geo_xfb,

            full_prog: full_prog,
            full_vao:  full_vao,

            vbo: vbo,
        })
    }

}

// This function will set up the OpenGL Vertex Attributes for the standard sprite shader program.
// It is here as a convenience function, since this is common to the Debug and Release renderers.
fn setup_gl_attributes(prog: &opengl::ShaderProgram) -> Result<opengl::VertexArray, Error> {
    #![allow(non_snake_case)]

    // All of the attribute state will be stored in this VAO.
    let vao = opengl::VertexArray::new();
    vao.bind();

    prog.use_program();

    let screen_TL = try!(prog.get_attrib("screen_TL"));
    screen_TL.enable();
    screen_TL.set_pointer(2, gl::FLOAT, false, mem::size_of::<SpriteVertex>(),
        attrib_offset!(screen_TL));

    let screen_BR = try!(prog.get_attrib("screen_BR"));
    screen_BR.enable();
    screen_BR.set_pointer(2, gl::FLOAT, false, mem::size_of::<SpriteVertex>(),
        attrib_offset!(screen_BR));

    let tex_TL = try!(prog.get_attrib("tex_TL"));
    tex_TL.enable();
    tex_TL.set_pointer(2, gl::FLOAT, false, mem::size_of::<SpriteVertex>(),
        attrib_offset!(tex_TL));

    let tex_BR = try!(prog.get_attrib("tex_BR"));
    tex_BR.enable();
    tex_BR.set_pointer(2, gl::FLOAT, false, mem::size_of::<SpriteVertex>(),
        attrib_offset!(tex_BR));

    let depth = try!(prog.get_attrib("depth"));
    depth.enable();
    depth.set_pointer(1, gl::FLOAT, false, mem::size_of::<SpriteVertex>(),
        attrib_offset!(depth));

    Ok(vao)
}

impl Renderer for DebugRenderer {
    fn prepare(&self, verts: &[SpriteVertex]) {
        println!("buffering data: {:?}", verts);
        self.vbo.buffer_data(verts);
        self.vbo.bind();
    }

    /// Render the sprites, as well as printing the output of the vertex and geometry shaders to
    /// stdout.
    fn render<'x>(&mut self, grp: RenderGroup<'x>) {
        grp.sheet.color.bind_to_unit(0);
        grp.sheet.depth.bind_to_unit(1);

        self.vtx_prog.use_program();
        self.vtx_vao.bind();
        self.vtx_xfb.bind();

        unsafe {
            gl::Enable(gl::RASTERIZER_DISCARD);
            gl::BeginTransformFeedback(gl::POINTS);
            gl::DrawArrays(gl::POINTS, grp.first as GLint, grp.count as GLsizei);
            gl::EndTransformFeedback();
            gl::Flush();
        }

        let xfb_slice = self.vtx_xfb.read();

        println!("slice len: {}", xfb_slice.len());
        println!("# vertex shader output ({} verts):", grp.count);
        for vtx in xfb_slice.iter().take(grp.count) {
            println!("{:?}", vtx);
        }

        self.geo_prog.use_program();
        self.geo_vao.bind();
        self.geo_xfb.bind();

        unsafe {
            gl::BeginTransformFeedback(gl::TRIANGLES);
            gl::DrawArrays(gl::POINTS, grp.first as GLint, grp.count as GLsizei);
            gl::EndTransformFeedback();
            gl::Flush();
        }

        println!("# geometry shader output ({} sprites):", grp.count);
        for prim in self.geo_xfb.read().chunks(6).take(grp.count) {
            println!("{:?} {:?} {:?} {:?}", prim[0], prim[1], prim[2], prim[5]);
        }

        self.full_prog.use_program();
        self.full_vao.bind();

        unsafe {
            gl::Disable(gl::RASTERIZER_DISCARD);
            gl::DrawArrays(gl::POINTS, grp.first as GLint, grp.count as GLsizei);
        }
    }
}

/// A `SharedDb` of `sprite::Sheet`s.
pub type Database = db::SharedDb<Sheet>;

/// A request for a sprite to be drawn. These are aggregated by the `Batcher` and turned into
/// efficient OpenGL calls.
#[derive(Copy,Clone)]
pub struct DrawReq {
    /// The id of the sprite-sheet where this sprite resides.
    pub sheet_id: SheetID,

    /// The index into that sheet of the sprite to be drawn.
    pub sprite_idx: usize,

    /// The location in the game world where that sprite's origin should be located.
    pub game_loc: math::Vec3<Meters>
}

impl DrawReq {
    fn to_vertex(&self, cam: &Camera, sheet: &Sheet) -> SpriteVertex {
        #![allow(non_snake_case)]

        let  cam_loc         = cam.game_to_camera(self.game_loc);
        let (scr_loc, depth) = cam.camera_to_screen(cam_loc);

        let row_coef = TexCoord((self.sprite_idx / sheet.num_across) as f32);
        let col_coef = TexCoord((self.sprite_idx % sheet.num_across) as f32);

        let tex_TL = vec2!(col_coef + TexCoord(1.0), row_coef) * sheet.tex_dimens;
        let tex_BR = vec2!(col_coef, row_coef + TexCoord(1.0)) * sheet.tex_dimens;

        let screen_TL_px = scr_loc - sheet.origin;
        let screen_BR_px = screen_TL_px + sheet.scr_dimens;

        SpriteVertex {
            screen_TL: cam.screen_to_ndu(screen_TL_px),
            screen_BR: cam.screen_to_ndu(screen_BR_px),

            tex_TL: vec2!(TexCoord(1.0) - tex_TL.x, TexCoord(1.0) - tex_TL.y),
            tex_BR: vec2!(TexCoord(1.0) - tex_BR.x, TexCoord(1.0) - tex_BR.y),

            depth: depth,
        }
    }
}

/// The `Batcher` gathers the set of sprites that need to be drawn each frame and aggregates them
/// into a smaller number of GL draw calls.
pub struct Batcher {
    by_sheet: Vec<Vec<DrawReq>>,
}

impl Batcher {
    /// Return a batcher which will use the given renderer.
    pub fn new() -> Batcher {
        Batcher {
            by_sheet: vec![],
        }
    }

    /// Register a `DrawReq` for this batch.
    pub fn register(&mut self, req: DrawReq) {
        if req.sheet_id >= self.by_sheet.len() {
            // Apparently `Vec::resize` is unstable, so here's a hacked version.
            let extra = (req.sheet_id + 1) - self.by_sheet.len();
            self.by_sheet.reserve(extra);
            for _ in 0..extra { self.by_sheet.push(vec![]) }
        }

        self.by_sheet[req.sheet_id].push(req)
    }

    /// Render all `DrawReq`s which have been passed to this `Batcher`. In addition to causing them
    /// to be rendered, this will also leave the `Batcher` clear for the next frame.
    pub fn render_batch<R: Renderer>(&mut self, r: &mut R, db: db::Handle<Sheet>, cam: &Camera) {

        let mut verts  = vec![];
        let mut groups = vec![];

        for (id, reqs) in self.by_sheet.iter().enumerate().filter(|&(_, v)| { !v.is_empty() }) {
            let sheet = match db.get_resource(id) {
                Some(sheet) => sheet,
                None        => continue,
            };

            groups.push(RenderGroup {
                first: verts.len(),
                count: reqs.len(),
                sheet: sheet,
            });

            for req in reqs.iter() {
                let vert = req.to_vertex(cam, sheet);
                verts.push(vert);
            }
        }

        r.prepare(&verts);

        for g in groups {
            r.render(g)
        }

        for v in self.by_sheet.iter_mut() {
            v.clear();
        }
    }
}

/// An error encountered when loading sprites or related resources.
#[derive(Debug)]
pub enum Error {
    /// Error loading a PNG.
    PngError(String),

    /// Error compiling a shader.
    CompileError(opengl::CompileError),

    /// Error linking a shader program.
    LinkError(opengl::LinkError),

    /// The engine and the shaders disagree about the name of a vertex attribute.
    NoSuchActiveAttrib(String),

    /// The engine and the shaders disagree about the name of a uniform.
    NoSuchActiveUniform(String),
}

impl From<opengl::CompileError> for Error {
    fn from(err: opengl::CompileError) -> Error {
        Error::CompileError(err)
    }
}

impl From<opengl::LinkError> for Error {
    fn from(err: opengl::LinkError) -> Error {
        Error::LinkError(err)
    }
}

impl From<opengl::NoSuchActiveAttrib> for Error {
    fn from(err: opengl::NoSuchActiveAttrib) -> Error {
        match err {
            opengl::NoSuchActiveAttrib(id) => Error::NoSuchActiveAttrib(id),
        }
    }
}

impl From<opengl::NoSuchActiveUniform> for Error {
    fn from(err: opengl::NoSuchActiveUniform) -> Error {
        match err {
            opengl::NoSuchActiveUniform(id) => Error::NoSuchActiveUniform(id),
        }
    }
}
