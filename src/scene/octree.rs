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

// Until I'm done w/ the design.
#![allow(dead_code)]

use std::num::Float;

use grafix::math;
use grafix::units::*;

/// An Octree is a data structure for maintaining the locations of objects in 3D space.
pub struct Octree<T> {
    // The cube which this octree subdivides.
    bcube: BoundingCube,

    // The octree will never be be partitioned into sections with a smaller side-length than this.
    min_dimen: Meters,

    // The root node of the octree.
    root: Box<Node>,

    // The nodes hold indices into this array.
    elems: Vec<Entry<T>>,
}

struct Entry<T> {
    bcube: BoundingCube,

    // The item at this location.
    val: T,
}

struct Node {
    // Which octant of the parent node contains this node. This will be O0 for the root.
    octant: Octant,

    // The nodes which are contained inside of this one.
    children: [Option<Box<Node>>; 8],

    // Indices into the `elems` field of the Octree.
    contents: Vec<usize>,
}


bitflags! {
    /// Flags for identifying octants of a cube.
    flags Octant: u8 {
        const O_0   = 0b000,
        const O_X   = 0b001,
        const O_Y   = 0b010,
        const O_Z   = 0b100,
        const O_XY  = O_X.bits | O_Y.bits,
        const O_XZ  = O_X.bits | O_Z.bits,
        const O_YZ  = O_Y.bits | O_Z.bits,
        const O_XYZ = O_X.bits | O_Y.bits | O_Z.bits,
    }
}

/// A cube in 3D space.
#[deriver(Copy,Debug)]
pub struct BoundingCube {
    /// The location of the corner of the cube with the lowest *x*, *y*, *z* values.
    pub position: math::Vec3<Meters>,

    /// The length of each side of the cube.
    pub size: Meters,
}

// This enum specifies how a cube is bounded by another cube. It is useful for inserting cubes into
// an octree.
enum Boundedness {
    // The "inner" cube is not bounded by the other.
    None,

    // The inner cube is bounded by the other, but none of it's sub-octants bound it.
    Minimal,

    // The inner cube is bounded by one of this cube's octants.
    Octant(Octant),
}

impl BoundingCube {
    // Return the `Octant` containing `v`, if any. If the point is at the exact center of the cube,
    // `O_0` will be returned.
    fn octant(&self, v: math::Vec3<Meters>) -> Option<Octant> {
        let half_edge = self.size / Meters(2.0);
        let center    = self.position + vec3!(half_edge, half_edge, half_edge);
        let diff      = v - center;

        let mut octant = if diff.x.abs() > half_edge {
            return None;
        } else if diff.x > Float::zero() { O_X } else { O_0 };

        octant = octant | if diff.y.abs() > half_edge {
            return None;
        } else if diff.y > Float::zero() { O_Y } else { O_0 };
        
        Some(octant | if diff.z.abs() > half_edge {
            return None;
        } else if diff.z > Float::zero() { O_Z } else { O_0 })
    }

    // Return true if `v` is within this cube.
    fn contains(&self, v: math::Vec3<Meters>) -> bool {
        let half_edge = self.size / Meters(2.0);
        let center    = self.position + vec3!(half_edge, half_edge, half_edge);
        let diff      = v - center;

        diff.x.abs() < half_edge && diff.y.abs() < half_edge && diff.z.abs() < half_edge
    }

    // Specify how (if at all) `other` is bounded by `self`
    fn boundedness(&self, other: BoundingCube) -> Boundedness {
    }
}
