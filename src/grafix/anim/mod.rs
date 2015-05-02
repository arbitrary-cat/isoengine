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

#[allow(missing_docs)]
pub mod wire;

#[cfg(feature = "client")] mod client;
#[cfg(feature = "client")] pub use self::client::*;

use time;

/// An ID that refers to a particular `Anim` in a `Database`.
pub type AnimID = usize;

/// An instance of an animation, which specifies how long it should take for the animation to
/// complete, when the animation began, where the first frame's sprite is located, and where the
/// animation should end up.
#[derive(Clone)]
pub struct Instance {
    /// The Anim being instantiated.
    pub anim_id: AnimID,

    /// The time (expressed as a duration since startup) at which the animation began.
    pub t_start: time::Duration,

    /// The duration of the animation.
    pub duration: time::Duration,

    /// True if this animation should repeat indefinitely.
    pub repeat: bool,
}

impl Instance {
    /// Create a struct from its FlatBuffer representation.
    pub fn from_wire(w: &wire::AnimInstance) -> Instance {
        Instance {
            anim_id:  w.id() as AnimID,
            t_start:  time::Duration::usec(w.t_start()),
            duration: time::Duration::usec(w.duration()),
            repeat:   w.repeat(),
        }
    }

    /// Get the FlatBuffer representation of this struct.
    pub fn to_wire(&self) -> wire::AnimInstance {
        wire::AnimInstance::new(
            self.t_start.as_usec(),
            self.duration.as_usec(),
            self.anim_id as u32,
            self.repeat,
        )
    }
}
