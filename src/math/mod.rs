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

mod vector2d;
mod vector3d;

pub use math::vector2d::Vec2;
pub use math::vector3d::Vec3;

#[macro_export]
/// Create a `Vec2` from components. This macro takes an optional conversion parameter which must be
/// a function. It will be applied to each element of the vector before construction.
///
/// ```rust
///    #[macro_use]
///    extern crate isoengine;
///
///    use std::num::Float;
///    use isoengine::math::Vec2;
///
///    fn main() {
///        let v:     Vec2<f32> = vec2!(1.0, 1.0);
///        let sqrts: Vec2<f32> = vec2!(|x: f32| x.sqrt() ; 4.0, 5.0);
///    }
/// ```
macro_rules! vec2 {
    ( $x:expr , $y:expr , ) => (vec2!($x, $y));


    ( $x:expr , $y:expr ) => ($crate::math::Vec2{ x: $x, y: $y });

    ( $conv:expr ; $x:expr , $y:expr , ) => (vec2!($conv ; $x, $y));

    ( $conv:expr ; $x:expr , $y:expr ) => ( $crate::math::Vec2 {
        x: $conv($x),
        y: $conv($y),
    });
}

#[macro_export]
/// Create a `Vec3` from components. This macro takes an optional conversion parameter which must be
/// a function. It will be applied to each element of the vector before construction.
///
/// ```rust
///    #[macro_use]
///    extern crate isoengine;
///
///    use std::num::Float;
///    use isoengine::math::Vec3;
///
///    fn main() {
///        let v:     Vec3<f32> = vec3!(1.0, 2.0, 3.0);
///        let sqrts: Vec3<f32> = vec3!(|x: f32| x.sqrt() ; 4.0, 5.0, 6.0);
///    }
/// ```
macro_rules! vec3 {
    ( $x:expr , $y:expr , $z:expr , ) => (vec3!($x, $y, $z));

    ( $x:expr , $y:expr , $z:expr ) => ($crate::math::Vec3{ x: $x, y: $y, z: $z });

    ( $conv:expr ; $x:expr , $y:expr , $z:expr , ) => (vec3!($conv ; $x, $y, $z));

    ( $conv:expr ; $x:expr , $y:expr , $z:expr ) => ( $crate::math::Vec3 {
        x: $conv($x),
        y: $conv($y),
        z: $conv($z),
    });
}
