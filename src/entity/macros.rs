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

macro_rules! make_ecs {
    { $($comp_name:ident : $comp_type:ty),+ } => { make_ecs! { $($comp_name: $comp_type, )+ } };
    { $($comp_name:ident : $comp_type:ty),+ , } => {

        use ::std::mem;
        use ::std::collections::{btree_map, BTreeMap};

        /// An entity is just a unique identifier which is used to locate associated components.
        pub type EntityID = u64;

        /// Whereas components represent the data of an entity, a `System` represents the logic.
        /// Components select and drive the behaviours of an entity, but `System`s are
        /// responsible for enacting that behaviour.
        pub trait System {
            /// Do general processing. This is called once per simulation step, before
            /// `process_entity` is called on any entities.
            fn update(&mut self);

            /// Process an entity. This will be called once per entity, per simulation step.
            fn process_entity<'x>(&mut self, entity: &mut View<'x>);
        }

        /// A view of an entity. This struct is passed to the `System`s for each entity they
        /// process.
        #[allow(missing_docs)] pub struct View<'x> {
            pub id: EntityID,

            $(pub $comp_name: Option<&'x mut $comp_type>,)+
        }

        impl<'x> View<'x> {
            /// Create a new view which doesn't reference any components.
            pub fn empty() -> View<'x> {
                View {
                    id: 0,
                    $($comp_name: None,)+
                }
            }
        }

        struct ComponentIter<'x, C> where C: 'x {
            next: Option<(&'x EntityID, &'x mut C)>,
            iter: btree_map::IterMut<'x, EntityID, C>,
        }

        /// A structure which holds all of the Components and Systems in the game, and processes
        /// them each frame.
        pub struct Manager {
            next_id: EntityID,

            systems: Vec<Box<System>>,

            $($comp_name: BTreeMap<EntityID, $comp_type>,)+
        }

        impl Manager {
            /// Create a new manager with no entities and no systems.
            pub fn new() -> Manager {
                Manager {
                    next_id: 1,
                    systems: vec![],

                    $($comp_name: BTreeMap::new(),)+
                }
            }

            /// Add a system to the manager. Each simulation step, systems are processed in the
            /// order that they were added to the manager. Similarly, entities are passed to the
            /// systems in the order they were added.
            pub fn add_system<S: System + 'static>(&mut self, system: S) {
                self.systems.push(box system)
            }

            /// Run a single frame of processing for all entities and systems.
            pub fn update(&mut self) {
                for system in self.systems.iter_mut() {
                    system.update();
                }

                $(
                    let mut $comp_name = ComponentIter {
                        next: None,
                        iter: self.$comp_name.iter_mut(),
                    };

                    $comp_name.next = $comp_name.iter.next();
                )+

                let mut next_entity = None;

                $(
                    next_entity = match (next_entity, &$comp_name.next) {
                        (Some(cur_id), &Some((new_id, _))) => if *new_id < cur_id {
                            Some(*new_id)
                        } else {
                            Some(cur_id)
                        },
                        (None, &Some((new_id, _))) => Some(*new_id),
                        (any,  &None)              => any,
                    };
                )+

                while let Some(cur_id) = next_entity {
                    let mut view = View {
                        id: cur_id,

                        $($comp_name: match $comp_name.next {
                            Some((id, _)) if *id == cur_id =>
                                match mem::replace(&mut $comp_name.next, $comp_name.iter.next()) {
                                    Some((_, comp)) => Some(comp),
                                    _               => unreachable!(),
                            },
                            _ => None,
                        },)+
                    };

                    for system in self.systems.iter_mut() {
                        system.process_entity(&mut view);
                    }

                    $(
                        next_entity = match (next_entity, &$comp_name.next) {
                            (Some(cur_id), &Some((new_id, _))) => if *new_id < cur_id {
                                Some(*new_id)
                            } else {
                                Some(cur_id)
                            },
                            (None, &Some((new_id, _))) => Some(*new_id),
                            (any,  &None)              => any,
                        };
                    )+
                }
            }

            /// Create an entity from a `View`. This will clone all of the components referred to by
            /// the `View`. This method is intended to be used via the `create_entity!` macro,
            /// though it can be called directly.
            ///
            /// Since the entity being viewed doesn't actually exist yet, it's `id` field is
            /// ignored.
            pub fn entity_from_view<'x>(&mut self, view: View<'x>) -> EntityID {
                let id = self.next_id;
                self.next_id = id + 1;

                $(
                    if let Some(comp_ref) = view.$comp_name {
                        self.$comp_name.insert(id, comp_ref.clone());
                    }
                )+

                id
            }

            /// Remove an entity from the `Manager`. If that entity didn't exist, this is a no-op.
            pub fn remove_entity(&mut self, id: EntityID) {
                $(self.$comp_name.remove(&id);)+
            }

            /// Get a view of an entity.
            pub fn view_entity<'x>(&'x mut self, id: EntityID) -> View<'x> {
                View {
                    id: id,
                    $($comp_name: self.$comp_name.get_mut(&id),)+
                }
            }
        }
    }
}
