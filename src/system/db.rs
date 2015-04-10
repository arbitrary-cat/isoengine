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
use std::ops::Deref;
use std::rc::Rc;

/// A database with interior mutability. It can hand out read-only `DatabaseHandle`s while still
/// allowing elements to be inserted (by using `RefCell` internally).
pub struct SharedDb<T> {
    inner: Rc<Db<T>>,
}

struct Db<T> {
    db_name: String,
    name2id: RefCell<HashMap<String, usize>>,
    id2elem: RefCell<Vec<T>>,
}

impl<T> SharedDb<T> {
    /// Create a new empty `Database`.
    pub fn new<S>(name: S) -> SharedDb<T> where String: From<S> {
        SharedDb{
            inner: Rc::new( Db {
                db_name: From::from(name),
                name2id: RefCell::new(HashMap::new()),
                id2elem: RefCell::new(Vec::new()),
            }),
        }
    }

    /// Insert a resource into the database.
    ///
    /// # Errors
    ///
    /// If there is already a resource by this name than an error will be logged and `self` will be
    /// unchanged.
    pub fn insert<S>(&self, name: S, elem: T) where String: From<S> {
        let owned_name = From::from(name);

        if self.inner.name2id.borrow().contains_key(&owned_name) {
            error!("Attempted to insert more than one resource named `{}' into the `{}' database.",
                owned_name, self.inner.db_name);
            return
        }

        let id = self.inner.id2elem.borrow().len();

        debug_assert_eq!(self.inner.name2id.borrow_mut().insert(owned_name, id), None);
        self.inner.id2elem.borrow_mut().push(elem);
    }

    /// 
    pub fn get_handle(&self) -> Handle<T> {
        Handle{ inner: self.inner.clone() }
    }
}

/// A reference to a resource stored in a database. This reference is tied to an immutable borrow
/// from an internal `RefCell` of the database, mutable operations on that database (i.e.
/// `SharedDb::insert`) will panic if called while this `Ref` is alive.
pub struct Ref<'x, T: 'x> {
    inner: cell::Ref<'x, Vec<T>>,
    index: usize,
}

impl<'x, T> Deref for Ref<'x, T> {
    type Target = T;

    /// Return a reference to the underlying resource.
    fn deref<'a>(&'a self) -> &'a T {
        self.inner.get(self.index).unwrap()
    }
}

/// A read-only reference to a `SharedDb`.
pub struct Handle<T> {
    inner: Rc<Db<T>>,
}

impl<T> Handle<T> {
    /// If there is a `Sheet` stored under `name` in the database, return its id. Otherwise return
    /// None.
    pub fn get_id<S: AsRef<str>>(&self, name: S) -> Option<usize> {
        self.inner.name2id.borrow().get(name.as_ref()).cloned()
    }

    /// Get a sprite sheet from an id. If there is no sheet with that id, then None is returned. But
    /// that should never happen because you got the id by calling `self.get_id()`... right?
    pub fn get_resource<'x>(&'x self, id: usize) -> Option<Ref<'x, T>> {
        let inner = self.inner.id2elem.borrow();

        if inner.get(id).is_some() {
            Some(Ref {
                inner: inner,
                index: id,
            })
        } else {
            None
        }
    }
}
