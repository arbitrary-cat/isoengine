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

use grafix::anim;
use math;
use units::*;

#[macro_use]
mod macros;

#[allow(missing_docs)]
pub mod wire;

/// Provides an entity with a location on the world map.
#[derive(Clone)]
pub struct WorldLocation {
    /// Bounding cube for this entity.
    pub bounds: math::BoundingCube,
}

impl WorldLocation {
    /// Convert from FlatBuffer representation.
    pub fn from_wire(w: &wire::WorldLocation) -> WorldLocation {
        WorldLocation {
            bounds: math::BoundingCube {
                center: vec3!(Meters ;
                    w.bounds().center_x(),
                    w.bounds().center_y(),
                    w.bounds().center_z(),
                ),
                half_edge: Meters(w.bounds().half_edge()),
            }
        }
    }

    /// Convert to FlatBuffer representation.
    pub fn to_wire(&self) -> wire::WorldLocation {
        wire::WorldLocation::new(
            &wire::BoundingCube::new(
                self.bounds.center.x.0,
                self.bounds.center.y.0,
                self.bounds.center.z.0,
                self.bounds.half_edge.0,
            )
        )
    }
}

/// Provides an entity with a visible image on the world map.
#[derive(Clone)]
pub struct WorldRender {
    /// The animation that this entity is currently running (possibly a single-frame static
    /// animation).
    pub anim: anim::Instance,
}

impl WorldRender {
    /// Convert from FlatBuffer representation.
    pub fn from_wire(w: &wire::WorldRender) -> WorldRender {
        WorldRender { anim: anim::Instance::from_wire(w.anim()) }
    }

    /// Convert to FlatBuffer representation.
    pub fn to_wire(&self) -> wire::WorldRender {
        wire::WorldRender::new(&self.anim.to_wire())
    }
}

/// The client-side entity system.
#[cfg(feature = "client")] pub mod client {
    use super::WorldLocation;
    use super::WorldRender;

    make_ecs! {
        world_location: WorldLocation,
        world_render:   WorldRender
    }
}

/// The server-side entity system.
#[cfg(feature = "server")] pub mod server {
    use super::WorldLocation;
    use super::WorldRender;

    make_ecs! {
        world_location: WorldLocation,
        world_render:   WorldRender
    }
}

#[macro_export]
macro_rules! client_entity {

    ($manager:expr, $($comp_name:ident : $comp_val:expr),+) => {
        create_entity!($module, $manager, $($comp_name : $comp_val,)+)
    };

    ($manager:expr, $($comp_name:ident : $comp_val:expr,)+) => {
        {
            $( let mut $comp_name = $comp_val; )+

            let mut __view = $crate::entity::client::View::empty();

            $( __view.$comp_name = Some(&mut $comp_name); )+

            $manager.entity_from_view(__view)
        }
    }

}

#[macro_export]
macro_rules! server_entity {

    ($manager:expr, $($comp_name:ident : $comp_val:expr),+) => {
        create_entity!($module, $manager, $($comp_name : $comp_val,)+)
    };

    ($manager:expr, $($comp_name:ident : $comp_val:expr,)+) => {
        {
            $( let mut $comp_name = $comp_val; )+

            let mut __view = $crate::entity::server::View::empty();

            $( __view.$comp_name = Some(&mut $comp_name); )+

            $manager.entity_from_view(__view)
        }
    }

}
