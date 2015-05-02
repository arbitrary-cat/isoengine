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

use std::collections::BTreeMap;
use std::convert::AsRef;

#[allow(missing_docs)]
pub mod wire;

#[cfg(feature = "client")] mod client;

#[cfg(feature = "client")] pub use self::client::*;

/// A unique identifier for an asset.
pub type AssetID = usize;

/// Different types of game assets.
#[derive(Clone,Debug)]
pub enum Type {
    /// A Sprite Sheet, corresponding to a `sprite::Sheet` in the client.
    SpriteSheet,

    /// An Animation, corresponding to an `anim::Anim` in the client.
    Animation,
}

/// A database of `AssetID`s, it doesn't store any actual assets, just their types.
pub struct ServerDb {
    by_name: BTreeMap<String, AssetID>,
    by_id:   Vec<Type>,
}

impl ServerDb {
    /// Load a `ServerDb`
    pub fn from_manifest(w: &wire::AssetManifest) -> ServerDb {
        let mut db = ServerDb{
            by_name: BTreeMap::new(),
            by_id:   Vec::new(),
        };

        for wire_sheet_desc in w.sprite_sheets().unwrap().iter() {
            let id: AssetID = db.by_id.len();

            let name = From::from(wire_sheet_desc.name().unwrap().as_ref());

            db.by_name.insert(name, id);

            db.by_id.push(Type::SpriteSheet);
        }

        for wire_anim in w.anims().unwrap().iter() {
            let id: AssetID = db.by_id.len();

            let name = From::from(wire_anim.name().unwrap().as_ref());

            db.by_name.insert(name, id);

            db.by_id.push(Type::Animation);
        }

        db
    }

    /// Get the type of the asset referred to by a given ID, if such an asset exists.
    pub fn type_by_id(&self, id: AssetID) -> Option<Type> {
        self.by_id.get(id).cloned()
    }

    /// Get the ID of the asset referred to by a given name, if such an asset exists.
    pub fn id_by_name<S: AsRef<str>>(&self, name: &S) -> Option<AssetID> {
        self.by_name.get(name.as_ref()).cloned()
    }

    /// Get the type of the asset referred to by a given name, if such an asset exists.
    pub fn type_by_name<S: AsRef<str>>(&self, name: &S) -> Option<Type> {
        self.id_by_name(name).and_then(|id| self.type_by_id(id))
    }

    /// Print out the name, id, and type of every item in the database.
    pub fn dbg_print(&self) {
        for (name, &id) in self.by_name.iter() {
            if let Some(typ) = self.by_id.get(id) {
                println!("Resource `{}' has id #{} and type `{:?}'.", *name, id, *typ);
            } else {
                println!("Resource `{}' refers to dangling id #{}", *name, id);
            }
        }
    }
}
