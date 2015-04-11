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

use grafix::sprite;
use math;
use system::db;
use time;
use units::*;

/// An ID that refers to a particular `Anim` in a `Database`.
pub type AnimID = usize;

/// A `SharedDb` of `anim::Anim`s.
pub type Database = db::SharedDb<Anim>;

/// An animation, which is just an ordered collection of sprites from a sprite-sheet.
pub struct Anim {
    /// The ID of the sheet where the sprites for this animation reside.
    pub sheet_id: sprite::SheetID,

    /// The indices of the frames of this animation, in order. This vector **must** be non-empty.
    pub indices:  Vec<usize>,
}

/// An instance of an animation, which specifies how long it should take for the animation to
/// complete, when the animation began, where the first frame's sprite is located, and where the
/// animation should end up.
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
    /// Return a `sprite::DrawReq` for this instance rendered at a particular time.
    pub fn draw_at(&self, db: db::Handle<Anim>, loc: math::Vec3<Meters>, t: time::Duration)
        -> Option<sprite::DrawReq> {

        let anim = if let Some(anim) = db.get_resource(self.anim_id) {
            anim
        } else {
            return None
        };

        if t < self.t_start {
            return None
        }

        let elapsed = if self.repeat {
            (t - self.t_start) % self.duration
        } else {
            t - self.t_start
        };

        let interp = elapsed / self.duration;

        if interp >= 1.0 {
            return None
        }

        let frame = ((anim.indices.len() as f64) * interp).floor() as usize;

        Some(sprite::DrawReq {
            sheet_id:   anim.sheet_id,
            sprite_idx: anim.indices[frame],
            game_loc:   loc,
        })
    }

    /// Return the time at which this instance will end.
    pub fn end_time(&self) -> time::Duration {
        self.t_start + self.dur
    }
}
