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

#[macro_use]
mod macros;

/// Provides an entity with a location on the world map.
#[derive(Clone)]
pub struct WorldLocation {
    /// Bounding cube for this entity.
    pub bounds: math::BoundingCube,
}

/// Provides an entity with a visible image on the world map.
#[derive(Clone)]
pub struct WorldRender {
    /// Sprite sheet to draw this entity from.
    pub sheet_id: sprite::SheetID,

    /// Index within the sheet of the sprite to draw.
    pub sprite_idx: usize,
}

mod ecs {
    use super::WorldLocation;
    use super::WorldRender;

    make_ecs! {
        world_location: WorldLocation,
        world_render:   WorldRender
    }
}

pub use self::ecs::{EntityID, System, View, Manager};

#[macro_export]
macro_rules! create_entity {
    ($manager:expr, $($comp_name:ident : $comp_val:expr),+) => {
        create_entity!($manager, $($comp_name, $comp_val,)+)
    };
    ($manager:expr, $($comp_name:ident : $comp_val:expr,)+) => {
        {
            $( let mut $comp_name = $comp_val; )+

            let mut view = $crate::entity::View::empty();

            $( view.$comp_name = Some(&mut $comp_name); )+

            $manager.entity_from_view(view)
        }
    }
}
