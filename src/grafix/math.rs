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

/// A 2D Vector type, with floating point elements.
#[derive(Copy,Clone,Debug)]
#[allow(missing_docs)]
pub struct Vec2<F: Float> {
    pub x: F,
    pub y: F,
}

macro_rules! vec2 {
    ($x:expr, $y:expr) => (Vec2 { x: $x, y: $y })
}

impl<F: Float> Vec2<F> {
    /// Return a zero vector.
    #[inline]
    pub fn zero() -> Vec2<F> {
        Vec2 { x: Float::zero(), y: Float::zero() }
    }

    /// Compute the dot product of two Vec2's.
    #[inline]
    pub fn dot(self, rhs: Vec2<F>) -> F {
        self.x * rhs.x + self.y * rhs.y
    }

    /// Return a vector whose components are equal to `self`, scaled by a factor of `s`.
    #[inline]
    pub fn scaled(self, s: F) -> Vec2<F> {
        Vec2 { x: self.x * s, y: self.y * s }
    }

    /// Compute the length of this vector.
    #[inline]
    pub fn length(self) -> F {
        self.dot(self).sqrt()
    }

    /// Return a unit length vector in the same direction as `self`.
    #[inline]
    pub fn normalized(self) -> Vec2<F> {
        self.scaled(self.dot(self).rsqrt())
    }
}

impl<F: Float> Add for Vec2<F> {
    type Output = Vec2<F>;

    /// Return the result of adding `self` to `rhs` component-wise.
    #[inline]
    fn add(self, rhs: Vec2<F>) -> Vec2<F> {
        Vec2 { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl<F: Float> Sub for Vec2<F> {
    type Output = Vec2<F>;

    /// Return the result of subtracting `rhs` from `self` component-wise.
    #[inline]
    fn sub(self, rhs: Vec2<F>) -> Vec2<F> {
        Vec2 { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl<F: Float> Mul for Vec2<F> {
    type Output = Vec2<F>;

    /// Return the result of multiplying `self` by `rhs` component-wise.
    #[inline]
    fn mul(self, rhs: Vec2<F>) -> Vec2<F> {
        Vec2 { x: self.x * rhs.x, y: self.y * rhs.y }
    }
}

impl<F: Float> Div for Vec2<F> {
    type Output = Vec2<F>;

    /// Return the result of dividing `self` by `rhs` component-wise.
    #[inline]
    fn div(self, rhs: Vec2<F>) -> Vec2<F> {
        Vec2 { x: self.x / rhs.x, y: self.y / rhs.y }
    }
}

impl<F: Float> Neg for Vec2<F> {
    type Output = Vec2<F>;

    /// Return a vector which is the additive inverse of self.
    #[inline]
    fn neg(self) -> Vec2<F> {
        Vec2 { x: -self.x, y: -self.y }
    }
}
