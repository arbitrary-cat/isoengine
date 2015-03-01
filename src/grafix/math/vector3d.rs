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

use std::num::Float;
use std::ops::{Add, Sub, Mul, Div, Neg};

/// A 3D Vector type, with floating point elements.
#[derive(Copy,Clone,Debug)]
#[allow(missing_docs)]
pub struct Vec3<F: Float> {
    pub x: F,
    pub y: F,
    pub z: F,
}

impl<F: Float> Vec3<F> {
    /// Return a zero vector.
    #[inline]
    pub fn zero() -> Vec3<F> {
        Vec3 { x: Float::zero(), y: Float::zero(), z: Float::zero() }
    }

    /// Compute the dot product of two Vec3's.
    #[inline]
    pub fn dot(self, rhs: Vec3<F>) -> F {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    /// Return a vector whose components are equal to `self`, scaled by a factor of `s`.
    #[inline]
    pub fn scaled(self, s: F) -> Vec3<F> {
        Vec3 { x: self.x * s, y: self.y * s, z: self.z * s }
    }

    /// Compute the length of this vector.
    #[inline]
    pub fn length(self) -> F {
        self.dot(self).sqrt()
    }

    /// Return a unit length vector in the same direction as `self`.
    #[inline]
    pub fn normalized(self) -> Vec3<F> {
        self.scaled(self.dot(self).rsqrt())
    }
}

impl<F: Float> Add for Vec3<F> {
    type Output = Vec3<F>;

    /// Return the result of adding `self` to `rhs` component-wise.
    #[inline]
    fn add(self, rhs: Vec3<F>) -> Vec3<F> {
        Vec3 { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
    }
}

impl<F: Float> Sub for Vec3<F> {
    type Output = Vec3<F>;

    /// Return the result of subtracting `rhs` from `self` component-wise.
    #[inline]
    fn sub(self, rhs: Vec3<F>) -> Vec3<F> {
        Vec3 { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z }
    }
}

impl<F: Float> Mul for Vec3<F> {
    type Output = Vec3<F>;

    /// Return the result of multiplying `self` by `rhs` component-wise.
    #[inline]
    fn mul(self, rhs: Vec3<F>) -> Vec3<F> {
        Vec3 { x: self.x * rhs.x, y: self.y * rhs.y, z: self.z * rhs.z }
    }
}

impl<F: Float> Div for Vec3<F> {
    type Output = Vec3<F>;

    /// Return the result of dividing `self` by `rhs` component-wise.
    #[inline]
    fn div(self, rhs: Vec3<F>) -> Vec3<F> {
        Vec3 { x: self.x / rhs.x, y: self.y / rhs.y, z: self.z / rhs.z }
    }
}

impl<F: Float> Neg for Vec3<F> {
    type Output = Vec3<F>;

    /// Return a vector which is the additive inverse of self.
    #[inline]
    fn neg(self) -> Vec3<F> {
        Vec3 { x: -self.x, y: -self.y, z: -self.z }
    }
}


