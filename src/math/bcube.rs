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

// The bitflags! macro generates missing docs.
#![allow(missing_docs)]

use num::Float;

use math;
use units::*;

bitflags! {
    /// Flags for identifying octants of a cube. Why do they have the `S` prefix? Because `O0` was
    /// weird looking, and `S` could totally stand for "segment" or "section" or something.
    flags Octant: u8 {
        const S0   = 0b000,
        const SX   = 0b001,
        const SY   = 0b010,
        const SZ   = 0b100,
        const SXY  = SX.bits | SY.bits,
        const SXZ  = SX.bits | SZ.bits,
        const SYZ  = SY.bits | SZ.bits,
        const SXYZ = SX.bits | SY.bits | SZ.bits,
    }
}

impl Octant {
    /// Convert an octant into a vector. For a `BoundingCube` centered at the origin with a
    /// half-edge of 1 meter, this vector will point to the corner of the given octant (i.e.
    /// `SX.as_vector() == vec3!(Meters ; 1.0, -1.0, -1.0)`).
    pub fn as_vector(self) -> math::Vec3<Meters> {
        match self.bits {
            0b000 => vec3!(Meters ; -1.0, -1.0, -1.0),
            0b100 => vec3!(Meters ;  1.0, -1.0, -1.0),
            0b010 => vec3!(Meters ; -1.0,  1.0, -1.0),
            0b001 => vec3!(Meters ; -1.0, -1.0,  1.0),
            0b110 => vec3!(Meters ;  1.0,  1.0, -1.0),
            0b101 => vec3!(Meters ;  1.0, -1.0,  1.0),
            0b011 => vec3!(Meters ; -1.0,  1.0,  1.0),
            0b111 => vec3!(Meters ;  1.0,  1.0,  1.0),
            _     => unreachable!(),
        }
    }
}

/// A cube in 3D space.
#[derive(Copy,Clone,Debug)]
pub struct BoundingCube {
    /// The location of center of the cube.
    pub center: math::Vec3<Meters>,

    /// half of the length of a side of the cube.
    pub half_edge: Meters,
}

/// This enum specifies how a cube is bounded by another cube. It is useful for inserting cubes into
/// an octree.
///
/// Since I'm using loose octrees, however, idk if this will actually see use.
pub enum Boundedness {
    /// The inner cube is not bounded by the other.
    None,

    /// The inner cube is bounded by the other, but none of it's sub-octants bound it.
    Minimal,

    /// The inner cube is bounded by one of this cube's octants.
    Octant(Octant),
}

impl BoundingCube {
    // All of these methods are inlined because they tend to be used together and common
    // subexpression elimination can go a long way.

    /// Return the `Octant` containing `v`, if any. If a point is on the boundary between two
    /// octants, it will err towards S0.
    #[inline] pub fn octant(&self, v: math::Vec3<Meters>) -> Option<Octant> {
        let diff = v - self.center;

        let mut octant = if diff.x.abs() > self.half_edge {
            return None;
        } else if diff.x > Meters(0.0) { SX } else { S0 };

        octant = octant | if diff.y.abs() > self.half_edge {
            return None;
        } else if diff.y > Meters(0.0) { SY } else { S0 };
        
        Some(octant | if diff.z.abs() > self.half_edge {
            return None;
        } else if diff.z > Meters(0.0) { SZ } else { S0 })
    }

    /// Return true if `v` is within this cube.
    #[inline] pub fn contains(&self, v: math::Vec3<Meters>) -> bool {
        let diff = v - self.center;

        diff.x.abs() < self.half_edge && diff.y.abs() < self.half_edge &&
            diff.z.abs() < self.half_edge
    }

    /// Specify how (if at all) `other` is bounded by `self`
    #[inline] pub fn boundedness(&self, other: &BoundingCube) -> Boundedness {
        use self::Boundedness as B;

        let half_diag = vec3!(other.half_edge, other.half_edge, other.half_edge);

        if let Some(near) = self.octant(other.center - half_diag) {
            match self.octant(other.center + half_diag) {
                Some(far) if near == far => B::Octant(far),
                Some(..)                 => B::Minimal,
                None                     => B::None,
            }
        } else {
            B::None
        }
    }

    /// Specify whether or not `other` is contained within `self`.
    #[inline] pub fn contains_bcube(&self, other: &BoundingCube) -> bool {
        let half_diag = vec3!(other.half_edge, other.half_edge, other.half_edge);

        self.contains(other.center + half_diag) && self.contains(other.center - half_diag)
    }
}
