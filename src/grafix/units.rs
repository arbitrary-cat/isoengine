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

// Workaround since we can't put doc comments inside of the mkprim! macro.
#![allow(missing_docs)]

mkprim! {
    /// The basic unit of game space.
    pub float Meters(pub f32);

    /// A "logical" pixel, which may correspond to some different number of device pixels.
    pub float Pixels(pub f32);

    /// The smallest unit of color that OpenGL is working with. One DevicePixel corresponds to one
    /// fragment in the shader pipeline. It might not actually be a single pixel on the final output
    /// device, for example if the game is running full-screen at less than native resolution.
    pub float DevicePixels(pub f32);

    /// Normalized Device Units, This is the unit that we actually send to OpenGL, and in which the
    /// shaders think.
    pub float NDU(pub f32);

    /// A texture coordinate, in the range [0.0, 1.0].
    pub float TexCoord(pub f32);
}
