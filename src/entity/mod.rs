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


#[macro_use]
mod macros;

#[allow(missing_docs)]
pub mod wire;

/// Components which can make up client- or server-side entities.
pub mod component;

/// The client-side entity system.
#[cfg(feature = "client")] pub mod client {
    use entity::component;

    make_ecs! {
        world_location: component::WorldLocation,
        world_render:   component::WorldRender,
    }
}

/// The server-side entity system.
#[cfg(feature = "server")] pub mod server {
    use entity::component;

    make_ecs! {
        world_location: component::WorldLocation,
        world_render:   component::WorldRender,
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
