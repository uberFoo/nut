//! ObjectStore for the instances of the "Imported Object" domain
//!
//! An end user should have little need to use this directly.
//!
//! This store contains the following instances:
//!    * [`AnotherObject`]
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

use crate::imported_object::types::AnotherObject;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ObjectStore {
    another_object: HashMap<Uuid, AnotherObject>,
}

impl ObjectStore {
    pub fn new() -> Self {
        Self {
            another_object: HashMap::new(),
        }
    }

    /// Inter [`AnotherObject`] into the [`ObjectStore`]
    ///
    pub fn inter_another_object(&mut self, another_object: AnotherObject) {
        self.another_object
            .insert(another_object.id, another_object);
    }

    /// Exhume [`Another Object`] from the [`ObjectStore`]
    ///
    pub fn exhume_another_object(&self, id: &Uuid) -> Option<&AnotherObject> {
        self.another_object.get(id)
    }

    /// Get an iterator over the internal `HashMap<(&Uuid, AnotherObject)>` in the [`ObjectStore`]
    ///
    pub fn iter_another_object(&self) -> impl Iterator<Item = (&Uuid, &AnotherObject)> {
        self.another_object.iter()
    }
}
