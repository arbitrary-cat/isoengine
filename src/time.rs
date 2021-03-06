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

use std::ops::{Add, Sub, Mul, Div, Rem};

/// A period of time, measured at microsecond granularity. Duratons are unsigned, there is no such
/// thing as a negative duration. It is implemented as a 64-bit number of microseconds, the struct
/// is cheap to pass around by value.
///
/// The mathematical operations defined on `Duration` are a bit peculiar in that they will take the
/// absolute value of any negative input or result, so that the output is always positive. This is
/// convenient **as long as you realize it's happening**, so be careful! All of the methods are
/// commented to describe how the arguments are transformed, so there will be no surprises *provided
/// that you read the docs*.
///
/// A Duration has a maximum value of around 35 million years.
#[derive(Copy,Clone,Debug,PartialEq,Eq,PartialOrd,Ord)]
pub struct Duration {
    us: u64,
}

impl Duration {
    /// Create a duration from a given number of seconds.
    pub fn sec(s: u64) -> Duration {
        Duration { us: s * 1_000_000 }
    }

    /// Create a duration from a given number of milliseconds.
    pub fn msec(ms: u64) -> Duration {
        Duration { us: ms * 1_000 }
    }

    /// Create a duration from a given number of microseconds.
    pub fn usec(us: u64) -> Duration {
        Duration { us: us }
    }

    /// Return the duration as an integer number of seconds.
    pub fn as_sec(self) -> u64 { self.us / 1_000_000 }

    /// Return the duration as an integer number of milliseconds.
    pub fn as_msec(self) -> u64 { self.us / 1_000 }

    /// Return the duration as an integer number of microseconds.
    pub fn as_usec(self) -> u64 { self.us }
}

impl Add for Duration {
    type Output = Duration;

    /// Add two durations together, producing another longer duration.
    fn add(self, rhs: Duration) -> Duration {
        Duration { us: self.us + rhs.us }
    }
}

impl Sub for Duration {
    type Output = Duration;

    /// Return the difference between two durations. This computes the absolute value of the
    /// difference, and is therefore commutative.
    fn sub(self, rhs: Duration) -> Duration {
        Duration { us: if self > rhs { self.us - rhs.us } else { rhs.us - self.us } }
    }
}

impl Div for Duration {
    type Output = f64;

    /// Compute the ratio between two durations. How many times would `rhs` have to elapse in order
    /// to equal `self`?
    fn div(self, rhs: Duration) -> f64 {
        (self.us as f64) / (rhs.us as f64)
    }
}

impl Mul<f64> for Duration {
    type Output = Duration;

    /// Scale `self` by the absolute value of `rhs`.
    fn mul(self, rhs: f64) -> Duration {
        Duration { us: ((self.us as f64) * rhs.abs()) as u64 }
    }
}

impl Mul<Duration> for f64 {
    type Output = Duration;

    /// Scale `rhs` by the absolute value of `self`.
    fn mul(self, rhs: Duration) -> Duration {
        // Defer to the preceding implementation.
        rhs * self
    }
}

impl Rem for Duration {
    type Output = Duration;

    /// Return the remainder of `self` beyond the nearest integer multiple (possibly 0) of `rhs`
    /// which is less than `self`.
    fn rem(self, rhs: Duration) -> Duration {
        Duration { us: self.us % rhs.us }
    }
}
