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

use std::cell::{self, RefCell};
use std::collections::HashMap;
use std::convert::{From,AsRef};
use std::rc::Rc;

/// A database with interior mutability. It can hand out read-only `DatabaseHandle`s while still
/// allowing elements to be inserted (by using `RefCell` internally).
#[derive(Clone)]
pub struct SharedDb<T> {
    inner: Rc<RefCell<Db<T>>>,
}

struct Db<T> {
    db_name: String,
    name2id: HashMap<String, usize>,
    id2elem: Vec<T>,
}

impl<T> SharedDb<T> {
    /// Create a new empty `Database`.
    pub fn new<S>(name: S) -> SharedDb<T> where String: From<S> {
        SharedDb{
            inner: Rc::new( RefCell::new( Db {
                db_name: From::from(name),
                name2id: HashMap::new(),
                id2elem: Vec::new(),
            })),
        }
    }

    /// Insert a resource into the database.
    ///
    /// This method utilizes interior mutability. It cannot be called if there are presently any
    /// `Handle`s to this database.
    ///
    /// # Errors
    ///
    /// If there is already a resource by this name than an error will be logged and `self` will be
    /// unchanged.
    pub fn insert<S>(&self, name: S, elem: T) where String: From<S> {
        let owned_name = From::from(name);

        let mut inner = self.inner.borrow_mut();

        if inner.name2id.contains_key(&owned_name) {
            error!("Attempted to insert more than one resource named `{}' into the `{}' database.",
                owned_name, inner.db_name);
            return
        }

        let id = inner.id2elem.len();

        debug_assert_eq!(inner.name2id.insert(owned_name, id), None);
        inner.id2elem.push(elem);
    }

    /// A read-only view into the database. It is capable of handing out references to resources
    /// which live for as long as the `Handle` itself.
    ///
    /// The `insert` method may only be called if there are no active `Handle`s.
    pub fn get_handle<'x>(&'x self) -> Handle<'x, T> {
        Handle{ inner: self.inner.borrow() }
    }
}

/// A read-only reference to a `SharedDb`.
pub struct Handle<'x, T: 'x> {
    inner: cell::Ref<'x, Db<T>>,
}

impl<'x, T: 'x> Handle<'x, T> {
    /// If there is a `Sheet` stored under `name` in the database, return its id. Otherwise return
    /// None.
    pub fn get_id<S: AsRef<str>>(&self, name: S) -> Option<usize> {
        self.inner.name2id.get(name.as_ref()).cloned()
    }

    /// Get a sprite sheet from an id. If there is no sheet with that id, then None is returned. But
    /// that should never happen because you got the id by calling `self.get_id()`... right?
    pub fn get_resource(&self, id: usize) -> Option<&T> {
        self.inner.id2elem.get(id)
    }
}
