//! ObjectStore for the instances of the "Everything" domain
//!
//! An end user should have little need to use this directly.
//!
//! This store contains the following instances:
//!    * [`RandoObject`]
//!    * [`Everything`]
//!
//! # Generated Code -- edit _with care_.
//!
//! Don't mess with anything between `{"magic":"","kind":"CriticalBlockBegin"}`
//! and `{"magic":"","kind":"CriticalBlockEnd"}`. Otherwise, you should be free
//! to go wild. Happy hacking!
//!
//! Use the following invocation to reproduce:
// {"magic":"","kind":{"IgnoreBlockBegin":{}}}
// {"magic":"","kind":{"IgnoreBlockBegin":{}}}
//! ```shell
//!  /Users/uberfoo/projects/sarzak/nut/target/debug/deps/generate_test_domain-0ff2341248e740c2 --nocapture
//!  /Users/uberfoo/projects/sarzak/nut/target/debug/deps/generate_test_domain-145fdb9ab1f4b4be --nocapture
//! ```
// {"magic":"","kind":"IgnoreBlockEnd"}
// {"magic":"","version":"0.2.0"}
// {"magic":"","version":"0.5.0"}
// {"magic":"","version":"1.0.0"}
use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::everything::types::{Everything, RandoObject};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ObjectStore {
    rando_object: HashMap<Uuid, RandoObject>,
    everything: HashMap<Uuid, Everything>,
}

impl ObjectStore {
    pub fn new() -> Self {
        Self {
            rando_object: HashMap::new(),
            everything: HashMap::new(),
        }
    }

    /// Inter [`RandoObject`] into the [`ObjectStore`]
    ///
    pub fn inter_rando_object(&mut self, rando_object: RandoObject) {
        self.rando_object.insert(rando_object.id, rando_object);
    }

    /// Exhume [`Rando Object`] from the [`ObjectStore`]
    ///
    pub fn exhume_rando_object(&self, id: &Uuid) -> Option<&RandoObject> {
        self.rando_object.get(id)
    }

    /// Get an iterator over the internal `HashMap<(&Uuid, RandoObject)>` in the [`ObjectStore`]
    ///
    pub fn iter_rando_object(&self) -> impl Iterator<Item = (&Uuid, &RandoObject)> {
        self.rando_object.iter()
    }

    /// Inter [`Everything`] into the [`ObjectStore`]
    ///
    pub fn inter_everything(&mut self, everything: Everything) {
        self.everything.insert(everything.id, everything);
    }

    /// Exhume [`Everything`] from the [`ObjectStore`]
    ///
    pub fn exhume_everything(&self, id: &Uuid) -> Option<&Everything> {
        self.everything.get(id)
    }

    /// Get an iterator over the internal `HashMap<(&Uuid, Everything)>` in the [`ObjectStore`]
    ///
    pub fn iter_everything(&self) -> impl Iterator<Item = (&Uuid, &Everything)> {
        self.everything.iter()
    }
}
