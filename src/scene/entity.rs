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

use std::collections::VecDeque;

/// An `EventBus` is the central nervous system of the entity. It allows communication between
/// `Component`s. Functionally it is just a queue for events.
pub struct EventBus<E> {
    queue: VecDeque<E>,
}

impl<E> EventBus<E> {
    /// Fire an event on the bus.
    pub fn fire(&mut self, event: E) { self.queue.push_back(event) }

    /// Get the next event off of the bus.
    pub fn next(&mut self) -> Option<E> { self.queue.pop_front() }
}

/// A `Component` is an abstraction around some functionality posessed by an `Entity`. They
/// communicate with one another over the `Entity`'s central `EventBus`.
pub trait Component<E> {
    /// Called at the beginning of each update cycle in order to allow components to add any events
    /// to the queue.
    fn stage(&mut self, &mut EventBus<E>);

    /// Called once per event, to allow components to modify them before they are acted upon. Keep
    /// in mind when adding events to `bus` as part of a `react` call, the event is not in its final
    /// final form.
    fn react(&mut self, event: &mut E, bus: &mut EventBus<E>);

    /// Called after all components have had an opportunity to mess with the event.
    fn commit(&mut self, event: &E, bus: &mut EventBus<E>);
}

/// The generic entity type. In general all entities should be implementation using `GenericEntity`,
/// however for performance reasons it might be useful to create specializations with unboxed
/// components.
pub trait Entity {
    /// Process this entity, updating all components. In each frame, systems should access the
    /// components *before* `update` is called on the entity.
    fn update(&mut self);
}

/// An implementation of the `Entity` trait where all components are generic (a boxed
/// `Component<E>`) and events are transmitted via a central `EventBus<E>`.
pub struct GenericEntity<E> {
    bus:        EventBus<E>,
    components: Vec<Box<Component<E>>>
}

impl<E> Entity for GenericEntity<E> {
    /// Process all components of this entity.
    fn update(&mut self) {

        for c in self.components.iter_mut() {
            c.stage(&mut self.bus);
        }

        while let Some(mut event) = self.bus.next() {

            for c in self.components.iter_mut() {
                c.react(&mut event, &mut self.bus);
            }

            for c in self.components.iter_mut() {
                c.commit(&event, &mut self.bus);
            }
        }
    }
}
