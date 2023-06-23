//! Types for instances of the "Isa Relationship" domain
//! # Domain Description
//!
//! Domain to test the supertype/subtype relationship.
//!
//!
//! # Contents
//!
//! The following types are defined herein:
//!    * [`SIMPLE_SUBTYPE_A`]
//!    * [`SIMPLE_SUBTYPE_B`]
//!    * [`SimpleSupertype`]
//!    * [`SubtypeA`]
//!    * [`SubtypeB`]
//!    * [`SuperT`]
//!
//! # Generated Code -- edit _with care_.
//!
//! Don't mess with anything between `{"magic":"","kind":"CriticalBlockBegin"}`
//! and `{"magic":"","kind":"CriticalBlockEnd"}`. Otherwise, you should be free
//! to go wild. Happy hacking!
//!
//! Use the following invocation to reproduce:
// {"magic":"","kind":{"IgnoreBlockBegin":{}}}
//! ```shell
//!  /Users/uberfoo/projects/sarzak/nut/target/debug/deps/generate_test_domain-145fdb9ab1f4b4be --nocapture
//! ```
// {"magic":"","kind":"IgnoreBlockEnd"}
// {"magic":"","version":"0.5.0"}
// {"magic":"","version":"1.0.0"}
use serde::{Deserialize, Serialize};
use uuid::{uuid, Uuid};

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"imports"}}}
use crate::isa_relationship::store::ObjectStore;
use crate::isa_relationship::UUID_NS;
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"imports"}}}

/// Simple [`Subtype`] A
///
/// This is represented as a singleton.
///
/// ❗️{ "singleton_object": true }
///
//
pub const SIMPLE_SUBTYPE_A: Uuid = uuid!["63a0306b-73d0-5cd5-8908-7360ee3dd9ed"];

/// Simple [`Subtype`] B
///
/// This is represented as a singleton.
///
/// ❗️{ "singleton_object": true }
///
//
pub const SIMPLE_SUBTYPE_B: Uuid = uuid!["13815980-a56d-5f1e-a4e2-57d9d2907c29"];

/// This [`Supertype`] is Simple
///
/// By that I mean that it's [`Subtypes`] consist only of singletons.
///
// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"simple_supertype-enum-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum SimpleSupertype {
    /// `SimpleSubtypeB(SimpleSubtypeB)`,
    ///
    SimpleSubtypeB(Uuid),
    /// `SimpleSubtypeA(SimpleSubtypeA)`,
    ///
    SimpleSubtypeA(Uuid),
}
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"{}-enum-definition"}}}

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"SimpleSupertype-enum-get-id-impl"}}}
impl SimpleSupertype {
    pub fn get_id(&self) -> Uuid {
        match *self {
            Self::SimpleSubtypeB(z) => z,
            Self::SimpleSubtypeA(z) => z,
        }
    }
}

// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"SimpleSupertype-enum-get-id-impl"}}}

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"simple_supertype-test_default"}}}
impl SimpleSupertype {
    pub fn test_default(store: &mut ObjectStore) -> Self {
        let test = Self::SimpleSubtypeB(SIMPLE_SUBTYPE_B);

        store.inter_simple_supertype(test.clone());

        test
    }
}
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"simple_supertype-test_default"}}}

/// This [`Subtype`][s] has [`Attribute`][a]s
///
/// [a]: nut::sarzak::Attribute
/// [s]: nut::sarzak::Subtype
///
// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"subtype_a-struct-definition"}}}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct SubtypeA {
    /// pub id: `Uuid`,
    ///
    pub id: Uuid,
    /// pub name: `std::string::String`,
    ///
    pub name: std::string::String,
}
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"subtype_a-struct-definition"}}}

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"subtype_a-new_impl"}}}
impl SubtypeA {
    //     // {"magic":"","kind":{"CriticalBlockBegin":{"tag":"subtype_a-new_impl"}}} //⚡️
    /// Inter a new SubtypeA and return it's `id`
    ///
    //     // {"magic":"","kind":{"IgnoreBlockBegin":{}}} //⚡️
    // {"magic":"","kind":{"IgnoreBlockBegin":{}}}
    /// # Example
    ///
    ///```
    /// # use test_models::isa_relationship::SubtypeA;
    /// # let mut store = test_models::isa_relationship::ObjectStore::new();
    ///
    /// let real_alley = "sleepy_pleasure".to_owned();
    ///
    /// let subtype_a = SubtypeA::new(&mut store, real_alley);
    ///```
    // {"magic":"","kind":"IgnoreBlockEnd"}
    pub fn new(store: &mut ObjectStore, name: std::string::String) -> Self {
        let id = Uuid::new_v5(&UUID_NS, format!("{}::", name,).as_bytes());
        let new = Self { id, name };

        store.inter_subtype_a(new.clone());

        new
    }
    // {"magic":"","kind":{"CriticalBlockEnd":{"tag":"subtype_a-new_impl"}}}
}

/// This [`Subtype`][s] has a number
///
/// [s]: nut::sarzak::Subtype
///
// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"subtype_b-struct-definition"}}}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct SubtypeB {
    /// pub id: `Uuid`,
    ///
    pub id: Uuid,
    /// pub number: `i64`,
    ///
    pub number: i64,
}
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"subtype_b-struct-definition"}}}

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"subtype_b-new_impl"}}}
impl SubtypeB {
    //     // {"magic":"","kind":{"CriticalBlockBegin":{"tag":"subtype_b-new_impl"}}} //⚡️
    /// Inter a new SubtypeB and return it's `id`
    ///
    //     // {"magic":"","kind":{"IgnoreBlockBegin":{}}} //⚡️
    // {"magic":"","kind":{"IgnoreBlockBegin":{}}}
    /// # Example
    ///
    ///```
    /// # use test_models::isa_relationship::SubtypeB;
    /// # let mut store = test_models::isa_relationship::ObjectStore::new();
    ///
    ///
    /// let subtype_b = SubtypeB::new(&mut store, 42);
    ///```
    // {"magic":"","kind":"IgnoreBlockEnd"}
    pub fn new(store: &mut ObjectStore, number: i64) -> Self {
        let id = Uuid::new_v5(&UUID_NS, format!("{}::", number,).as_bytes());
        let new = Self { id, number };

        store.inter_subtype_b(new.clone());

        new
    }
    // {"magic":"","kind":{"CriticalBlockEnd":{"tag":"subtype_b-new_impl"}}}
}

/// A [`Supertype`] with normal [`Subtype`]s
///
/// This was called "Super". Rust didn't like it when it became "super". There needs to be
///a way of fixing keywords.
///
// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"super_t-enum-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum SuperT {
    /// `SubtypeA(SubtypeA)`,
    ///
    SubtypeA(Uuid),
    /// `SubtypeB(SubtypeB)`,
    ///
    SubtypeB(Uuid),
}
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"{}-enum-definition"}}}

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"SuperT-enum-get-id-impl"}}}
impl SuperT {
    pub fn get_id(&self) -> Uuid {
        match *self {
            Self::SubtypeA(z) => z,
            Self::SubtypeB(z) => z,
        }
    }
}

// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"SuperT-enum-get-id-impl"}}}

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"super_t-test_default"}}}
impl SuperT {
    pub fn test_default(store: &mut ObjectStore) -> Self {
        //         // {"magic":"","kind":{"IgnoreBlockBegin":{}}} //⚡️
        //         let male_cakes = "kindly_achiever".to_owned(); //⚡️
        //         let test = Self::SubtypeA(SubtypeA::new(store, male_cakes).id); //⚡️
        // {"magic":"","kind":{"IgnoreBlockBegin":{}}}
        let loose_manager = "dry_feast".to_owned();
        let test = Self::SubtypeA(SubtypeA::new(store, loose_manager).id);
        // {"magic":"","kind":"IgnoreBlockEnd"}

        store.inter_super_t(test.clone());

        test
    }
}
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"super_t-test_default"}}}
