//! ObjectStore for the instances of the "One To One" domain
//!
//! An end user should have little need to use this directly.
//!
//! This store contains the following instances:
//!    * [`C`]
//!    * [`A`]
//!    * [`Referent`]
//!    * [`B`]
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

use crate::one_to_one::types::{Referent, A, B, C};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ObjectStore {
    c: HashMap<Uuid, C>,
    a: HashMap<Uuid, A>,
    referent: HashMap<Uuid, Referent>,
    b: HashMap<Uuid, B>,
}

impl ObjectStore {
    pub fn new() -> Self {
        Self {
            c: HashMap::new(),
            a: HashMap::new(),
            referent: HashMap::new(),
            b: HashMap::new(),
        }
    }

    /// Inter [`C`] into the [`ObjectStore`]
    ///
    pub fn inter_c(&mut self, c: C) {
        self.c.insert(c.id, c);
    }

    /// Exhume [`C`] from the [`ObjectStore`]
    ///
    pub fn exhume_c(&self, id: &Uuid) -> Option<&C> {
        self.c.get(id)
    }

    /// Get an iterator over the internal `HashMap<(&Uuid, C)>` in the [`ObjectStore`]
    ///
    pub fn iter_c(&self) -> impl Iterator<Item = (&Uuid, &C)> {
        self.c.iter()
    }

    /// Inter [`A`] into the [`ObjectStore`]
    ///
    pub fn inter_a(&mut self, a: A) {
        self.a.insert(a.id, a);
    }

    /// Exhume [`A`] from the [`ObjectStore`]
    ///
    pub fn exhume_a(&self, id: &Uuid) -> Option<&A> {
        self.a.get(id)
    }

    /// Get an iterator over the internal `HashMap<(&Uuid, A)>` in the [`ObjectStore`]
    ///
    pub fn iter_a(&self) -> impl Iterator<Item = (&Uuid, &A)> {
        self.a.iter()
    }

    /// Inter [`Referent`] into the [`ObjectStore`]
    ///
    pub fn inter_referent(&mut self, referent: Referent) {
        self.referent.insert(referent.id, referent);
    }

    /// Exhume [`Referent`] from the [`ObjectStore`]
    ///
    pub fn exhume_referent(&self, id: &Uuid) -> Option<&Referent> {
        self.referent.get(id)
    }

    /// Get an iterator over the internal `HashMap<(&Uuid, Referent)>` in the [`ObjectStore`]
    ///
    pub fn iter_referent(&self) -> impl Iterator<Item = (&Uuid, &Referent)> {
        self.referent.iter()
    }

    /// Inter [`B`] into the [`ObjectStore`]
    ///
    pub fn inter_b(&mut self, b: B) {
        self.b.insert(b.id, b);
    }

    /// Exhume [`B`] from the [`ObjectStore`]
    ///
    pub fn exhume_b(&self, id: &Uuid) -> Option<&B> {
        self.b.get(id)
    }

    /// Get an iterator over the internal `HashMap<(&Uuid, B)>` in the [`ObjectStore`]
    ///
    pub fn iter_b(&self) -> impl Iterator<Item = (&Uuid, &B)> {
        self.b.iter()
    }
}
