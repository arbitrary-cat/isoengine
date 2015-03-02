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

use std::num::Float;

use grafix::math;

mkprim! {
    pub Meters(f32);

    pub Pixels(f32);

    pub DevicePixels(f32);
}

/// A camera for a world with an isometric orthogonal projection. The camera knows how to translate
/// from coordinates in the game world to OpenGL's Normalized Device Units.
pub struct Camera {
    /// Size of a meter, in pixels.
    pub scale: f32,

    /// The number of pixels in the game's viewport. These may not correspond to actual pixels.
    pub resolution: math::Vec2<Pixels>,

    /// The number of actual device pixels that make up the screen. (Well... they might not really
    /// be device pixels, but whatever the smallest discreet unit of color on the screen is).
    pub true_resolution: math::Vec2<DevicePixels>,

    /// The position of the camera in space.
    pub position: math::Vec3<Meters>,

    // Note that the orientation of the camera is always the same, the euler angles are
    //
    //     60° x, 0° y, 45° z 
}

impl Camera {
    /// Convert game-space coordinates to camera-space coordinates. The z-component is the depth in
    /// meters of the camera coordinate.
    ///
    /// Both are measured in meters, since camera space is still "in the game world".
    pub fn game_to_camera(&self, game: math::Vec3<Meters>) -> math::Vec3<Meters> {
        // Here we count on LLVM to reduce a lot of this stuff. Hopefully computing sin_cos on
        // constants is free, and it won't actually build the matrix below.

        // These are the opposite of the Euler Angles used to orient the camera.
        let x_rot: f32 = (-60.0f32).to_radians();
        let y_rot: f32 = 00.0f32;
        let z_rot: f32 = (-45.0f32).to_radians();

        let (c1, s1) = x_rot.sin_cos();
        let (c2, s2) = y_rot.sin_cos();
        let (c3, s3) = z_rot.sin_cos();

        // This is the formula given by Wikipedia for turning XYZ Euler Angles into a 3D rotation
        // matrix.
        let m: [[f32; 3]; 3] = [
            [c2*c3,            -c2*s3,           s2],
            [c1*s3 + c3*s1*s2, c1*c3 - s1*s2*s3, -c2*s1],
            [s1*s3 - c1*c3*s2, c3*s1 + c1*s2*s3, c1*c2],
        ];

        let tr = game - self.position;

        math::Vec3 {
            x: tr.x*Meters(m[0][0]) + tr.y*Meters(m[0][1]) + tr.z*Meters(m[0][2]),
            y: tr.x*Meters(m[1][0]) + tr.y*Meters(m[1][1]) + tr.z*Meters(m[1][2]),
            z: tr.x*Meters(m[2][0]) + tr.y*Meters(m[2][1]) + tr.z*Meters(m[2][2]),
        }
    }
}
