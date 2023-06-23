//! ObjectStore for the instances of the "Isa Relationship" domain
//!
//! An end user should have little need to use this directly.
//!
//! This store contains the following instances:
//!    * [`SubtypeA`]
//!    * [`SimpleSupertype`]
//!    * [`SuperT`]
//!    * [`SubtypeB`]
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

use crate::isa_relationship::types::{SimpleSupertype, SubtypeA, SubtypeB, SuperT};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ObjectStore {
    subtype_a: HashMap<Uuid, SubtypeA>,
    simple_supertype: HashMap<Uuid, SimpleSupertype>,
    super_t: HashMap<Uuid, SuperT>,
    subtype_b: HashMap<Uuid, SubtypeB>,
}

impl ObjectStore {
    pub fn new() -> Self {
        Self {
            subtype_a: HashMap::new(),
            simple_supertype: HashMap::new(),
            super_t: HashMap::new(),
            subtype_b: HashMap::new(),
        }
    }

    /// Inter [`SubtypeA`] into the [`ObjectStore`]
    ///
    pub fn inter_subtype_a(&mut self, subtype_a: SubtypeA) {
        self.subtype_a.insert(subtype_a.id, subtype_a);
    }

    /// Exhume [`Subtype A`] from the [`ObjectStore`]
    ///
    pub fn exhume_subtype_a(&self, id: &Uuid) -> Option<&SubtypeA> {
        self.subtype_a.get(id)
    }

    /// Get an iterator over the internal `HashMap<(&Uuid, SubtypeA)>` in the [`ObjectStore`]
    ///
    pub fn iter_subtype_a(&self) -> impl Iterator<Item = (&Uuid, &SubtypeA)> {
        self.subtype_a.iter()
    }

    /// Inter [`SimpleSupertype`] into the [`ObjectStore`]
    ///
    pub fn inter_simple_supertype(&mut self, simple_supertype: SimpleSupertype) {
        self.simple_supertype
            .insert(simple_supertype.get_id(), simple_supertype);
    }

    /// Exhume [`Simple Supertype`] from the [`ObjectStore`]
    ///
    pub fn exhume_simple_supertype(&self, id: &Uuid) -> Option<&SimpleSupertype> {
        self.simple_supertype.get(id)
    }

    /// Get an iterator over the internal `HashMap<(&Uuid, SimpleSupertype)>` in the [`ObjectStore`]
    ///
    pub fn iter_simple_supertype(&self) -> impl Iterator<Item = (&Uuid, &SimpleSupertype)> {
        self.simple_supertype.iter()
    }

    /// Inter [`SuperT`] into the [`ObjectStore`]
    ///
    pub fn inter_super_t(&mut self, super_t: SuperT) {
        self.super_t.insert(super_t.get_id(), super_t);
    }

    /// Exhume [`Super T`] from the [`ObjectStore`]
    ///
    pub fn exhume_super_t(&self, id: &Uuid) -> Option<&SuperT> {
        self.super_t.get(id)
    }

    /// Get an iterator over the internal `HashMap<(&Uuid, SuperT)>` in the [`ObjectStore`]
    ///
    pub fn iter_super_t(&self) -> impl Iterator<Item = (&Uuid, &SuperT)> {
        self.super_t.iter()
    }

    /// Inter [`SubtypeB`] into the [`ObjectStore`]
    ///
    pub fn inter_subtype_b(&mut self, subtype_b: SubtypeB) {
        self.subtype_b.insert(subtype_b.id, subtype_b);
    }

    /// Exhume [`Subtype B`] from the [`ObjectStore`]
    ///
    pub fn exhume_subtype_b(&self, id: &Uuid) -> Option<&SubtypeB> {
        self.subtype_b.get(id)
    }

    /// Get an iterator over the internal `HashMap<(&Uuid, SubtypeB)>` in the [`ObjectStore`]
    ///
    pub fn iter_subtype_b(&self) -> impl Iterator<Item = (&Uuid, &SubtypeB)> {
        self.subtype_b.iter()
    }
}
