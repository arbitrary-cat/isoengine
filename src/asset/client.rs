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
use std::collections::BTreeMap;
use std::convert::{AsRef, From};
use std::mem;
use std::rc::Rc;

use grafix::anim;
use grafix::sprite;
use asset;

enum Asset {
    PlaceHolder,

    SpriteSheetAbsent(sprite::SheetDesc),
    SpriteSheet(sprite::Sheet),

    Animation(anim::Anim),
}

/// A database containing assets which can be retreived by name or ID.
pub struct AssetDb {
    inner: Rc<RefCell<AssetDbInner>>,
}

impl AssetDb {
    /// Load an `AssetDb` from a manifest file.
    pub fn from_manifest(w: &asset::wire::AssetManifest) -> AssetDb {
        let db = AssetDb {
            inner: Rc::new(RefCell::new(AssetDbInner::empty())),
        };

        for wire_sheet_desc in w.sprite_sheets().unwrap().iter() {
            let id: asset::AssetID = db.inner.borrow().by_id.len();

            let name = From::from(wire_sheet_desc.name().unwrap().as_ref());

            db.inner.borrow_mut().by_name.insert(name, id);

            let sheet_desc = sprite::SheetDesc::from_wire(wire_sheet_desc);

            db.inner.borrow_mut().by_id.push(Asset::SpriteSheetAbsent(sheet_desc));
        }

        for wire_anim in w.anims().unwrap().iter() {
            let id: asset::AssetID = db.inner.borrow().by_id.len();

            let name = From::from(wire_anim.name().unwrap().as_ref());

            db.inner.borrow_mut().by_name.insert(name, id);

            let anim = anim::Anim::from_wire(wire_anim, db.get_handle());

            db.inner.borrow_mut().by_id.push(Asset::Animation(anim));
        }

        db
    }

    /// Load a given asset by its ID. Some assets (e.g. sprite sheets) only have a descriptor loaded
    /// by `AssetDb::from_manifest`, and require this function to be called in order to load the
    /// associated resource into memory.
    pub fn load(&self, id: asset::AssetID) {
        use self::Asset::*;

        let mut mref = self.inner.borrow_mut();

        if let Some(x @ &mut SpriteSheetAbsent(..)) = mref.by_id.get_mut(id) {
            if let SpriteSheetAbsent(desc) = mem::replace(x, PlaceHolder) {
                match sprite::Sheet::from_desc(desc) {
                    Ok(sheet) => { mem::replace(x, SpriteSheet(sheet)); }
                    Err(err)  => debug!("couldn't load sprite: {:?}", err),
                }
            } else { unreachable!() }
        }
    }

    /// A read-only view into the database. It is capable of handing out references to resources
    /// which live for as long as the `Handle` itself.
    pub fn get_handle<'x>(&'x self) -> Handle<'x> {
        Handle { inner: self.inner.borrow() }
    }
}

struct AssetDbInner {
    by_name: BTreeMap<String, asset::AssetID>,
    by_id:   Vec<Asset>,
}

impl AssetDbInner {
    fn empty() -> AssetDbInner {
        AssetDbInner {
            by_name: BTreeMap::new(),
            by_id:   Vec::new(),
        }
    }
}

/// A read-only reference to an `AssetDb`.
pub struct Handle<'x> {
    inner: cell::Ref<'x, AssetDbInner>,
}

impl<'x> Handle<'x> {
    /// If there is an asset stored under `name` in the database, return its id. Otherwise return
    /// `None`.
    pub fn get_id<S: AsRef<str>>(&self, name: S) -> Option<asset::AssetID> {
        self.inner.by_name.get(name.as_ref()).cloned()
    }

    /// Get an `anim::Anim` from an `asset::AssetID`.
    pub fn get_anim(&self, id: asset::AssetID) -> Option<&anim::Anim> {
        use self::Asset::*;
        if let Some(&Animation(ref anim)) = self.inner.by_id.get(id) {
            Some(anim)
        } else {
            None
        }
    }

    /// Get a `sprite::Sheet` from an `asset::AssetID`.
    pub fn get_sprite_sheet(&self, id: asset::AssetID) -> Option<&sprite::Sheet> {
        use self::Asset::*;
        if let Some(&SpriteSheet(ref sheet)) = self.inner.by_id.get(id) {
            Some(sheet)
        } else {
            None
        }
    }
}
