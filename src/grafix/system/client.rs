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

use entity::client as entity;
use grafix::sprite;
use grafix::anim;
use grafix::camera::Camera;
use time;

/// An implementation of `entity::System` which is responsible for rendering sprites.
pub struct WorldRender<R: sprite::Renderer> {
    sprite_db: sprite::Database,
    anim_db:   anim::Database,
    batcher:   sprite::Batcher,
    renderer:  R,
    camera:    Camera,
}

impl<R: sprite::Renderer> entity::System for WorldRender<R> {
    /// Render last frame's entity batch.
    fn update(&mut self, _now: time::Duration) {
        self.batcher.render_batch(&mut self.renderer, self.sprite_db.get_handle(), &self.camera);
    }

    /// Add this entity to the batch to be rendered.
    fn process_entity<'x>(&mut self, now: time::Duration, entity: &mut entity::View<'x>) {
       if let &mut entity::View{
           world_location: Some(ref mut loc),
           world_render:   Some(ref mut ren),
           ..
       } = entity {
            if let Some(req) = ren.anim.draw_at(self.anim_db.get_handle(), loc.bounds.center, now) {
                self.batcher.register(req)
            }
       }
    }
}

impl <R: sprite::Renderer> WorldRender<R> {
    /// Create a new world rendering system with the given components.
    ///
    /// At the moment there is no way to update the database or camera. I'll work on that later.
    pub fn new(spr: sprite::Database, anm: anim::Database, renderer: R, cam: Camera) -> WorldRender<R> {
        WorldRender {
            sprite_db: spr,
            anim_db:   anm,
            batcher:   sprite::Batcher::new(),
            renderer:  renderer,
            camera:    cam,
        }
    }

}
