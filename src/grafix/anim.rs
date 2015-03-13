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

use grafix::math;
use grafix::sprite;
use grafix::units::*;

use time;

/// An animation, which is just an ordered collection of sprites from a sprite-sheet.
pub struct Anim {
    /// The ID of the sheet where the sprites for this animation reside.
    pub sheet_id: sprite::SheetID,

    /// The indices of the frames of this animation, in order. This vector **must** be non-empty.
    pub indices:  Vec<usize>,
}

impl Anim {
    /// Create an instance of this anim which starts at time `t_start` and runs for Duration `dur`,
    /// beginning at `start_loc` and being displaced by `disp`.
    pub fn instance<'x>(
        &'x self,
        t_start: time::Duration,
        dur: time::Duration,
        loc_start: math::Vec3<Meters>,
        disp: math::Vec3<Meters>,
    ) -> Instance<'x> {

        Instance {
            anim:      self,
            t_start:   t_start,
            dur:       dur,
            loc_start: loc_start,
            disp:      disp,
        }
    }

}

/// An instance of an animation, which specifies how long it should take for the animation to
/// complete, when the animation began, where the first frame's sprite is located, and where the
/// animation should end up.
pub struct Instance<'x> {
    // The Anim being instantiated.
    anim: &'x Anim,

    // The time (expressed as a duration since startup) at which the animation began.
    t_start: time::Duration,

    // The duration of the animation.
    dur: time::Duration,

    // The location at which the animation begins.
    loc_start: math::Vec3<Meters>,

    // The displacement vector of the animation (i.e. how far and in what direction the animation
    // will move).
    disp: math::Vec3<Meters>
}

impl<'x> Instance<'x> {
    /// Return a `sprite::DrawReq` for this instance rendered at a particular time.
    pub fn draw_at(&self, t: time::Duration) -> sprite::DrawReq {
        let elapsed = t - self.t_start;
        let interp = if t < self.t_start {
            0.0
        } else if elapsed / self.dur > 0.999 {
            0.999
        } else {
            elapsed / self.dur
        };

        let frame = ((self.anim.indices.len() as f64) * interp) as usize;

        sprite::DrawReq {
            sheet_id:   self.anim.sheet_id,
            sprite_idx: self.anim.indices[frame],
            game_loc:   self.loc_start + self.disp.scaled(Meters(interp as f32)),
        }
    }

    /// Return the time at which this instance will end.
    pub fn end_time(&self) -> time::Duration {
        self.t_start + self.dur
    }
}
