//! Types for instances of the "One To Many" domain
//! # Domain Description
//!
//! Domain to test 1-M relationships.
//!
//!
//! # Contents
//!
//! The following types are defined herein:
//!    * [`A`]
//!    * [`B`]
//!    * [`C`]
//!    * [`Referent`]
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
use uuid::Uuid;

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"imports"}}}
use crate::one_to_many::store::ObjectStore;
use crate::one_to_many::UUID_NS;
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"imports"}}}

/// This is the [`Referrer`] side of a 1-M relationship
///
// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"a-struct-definition"}}}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct A {
    /// pub id: `Uuid`,
    ///
    pub id: Uuid,
    /// pub name: `std::string::String`,
    ///
    pub name: std::string::String,
    /// pub ptr: `Referent`,
    ///
    pub ptr: Uuid,
}
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"a-struct-definition"}}}

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"a-new_impl"}}}
impl A {
    //     // {"magic":"","kind":{"CriticalBlockBegin":{"tag":"a-new_impl"}}} //⚡️
    /// Inter a new A and return it's `id`
    ///
    //     // {"magic":"","kind":{"IgnoreBlockBegin":{}}} //⚡️
    // {"magic":"","kind":{"IgnoreBlockBegin":{}}}
    /// # Example
    ///
    ///```
    /// # use test_models::one_to_many::Referent;
    /// # use test_models::one_to_many::A;
    /// # let mut store = test_models::one_to_many::ObjectStore::new();
    ///
    /// let opposite_sleep = "obtainable_health".to_owned();
    /// let referent_yob = Referent::new(&mut store, opposite_sleep);
    /// let voracious_chess = "cuddly_week".to_owned();
    ///
    /// let a = A::new(&mut store, &referent_yob, voracious_chess);
    ///```
    // {"magic":"","kind":"IgnoreBlockEnd"}
    pub fn new(store: &mut ObjectStore, ptr: &Referent, name: std::string::String) -> Self {
        let id = Uuid::new_v5(&UUID_NS, format!("{:?}::{}::", ptr, name,).as_bytes());
        let new = Self {
            id,
            ptr: ptr.id,
            name,
        };

        store.inter_a(new.clone());

        new
    }
    // {"magic":"","kind":{"CriticalBlockEnd":{"tag":"a-new_impl"}}}
}

/// Connected to TGT via _R2_.
///
/// This is for testing a 1c-M relationship.
///
// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"b-struct-definition"}}}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct B {
    /// pub baz: `std::string::String`,
    ///
    pub baz: std::string::String,
    /// pub id: `Uuid`,
    ///
    pub id: Uuid,
    //     /// pub ptr: `Option<Referent>`, //⚡️
    /// pub ptr: `Referent`,
    ///
    //     pub ptr: Option<Uuid>, //⚡️
    pub ptr: Uuid,
}
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"b-struct-definition"}}}

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"b-new_impl"}}}
impl B {
    //     // {"magic":"","kind":{"CriticalBlockBegin":{"tag":"b-new_impl"}}} //⚡️
    /// Inter a new B and return it's `id`
    ///
    //     // {"magic":"","kind":{"IgnoreBlockBegin":{}}} //⚡️
    // {"magic":"","kind":{"IgnoreBlockBegin":{}}}
    /// # Example
    ///
    ///```
    /// # use test_models::one_to_many::Referent;
    /// # use test_models::one_to_many::B;
    /// # let mut store = test_models::one_to_many::ObjectStore::new();
    ///
    /// let unequaled_aunt = "oceanic_land".to_owned();
    /// let referent = Referent::new(&mut store, unequaled_aunt);
    /// let hapless_balloon = "cowardly_cable".to_owned();
    ///
    /// let b = B::new(&mut store, Some(&referent), hapless_balloon);
    ///```
    // {"magic":"","kind":"IgnoreBlockEnd"}
    //     pub fn new(store: &mut ObjectStore, ptr: Option<&Referent>, baz: std::string::String) -> Self { //⚡️
    pub fn new(store: &mut ObjectStore, ptr: &Referent, baz: std::string::String) -> Self {
        let id = Uuid::new_v5(&UUID_NS, format!("{:?}::{}::", ptr, baz,).as_bytes());
        let new = Self {
            id,
            //             ptr: ptr.map(|o| o.id), //⚡️
            ptr: ptr.id,
            baz,
        };

        store.inter_b(new.clone());

        new
    }
    // {"magic":"","kind":{"CriticalBlockEnd":{"tag":"b-new_impl"}}}
}

/// This is the [`Referrent`] side of a 1-Mc
///
// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"c-struct-definition"}}}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct C {
    /// pub id: `Uuid`,
    ///
    pub id: Uuid,
    /// pub jackpot: `f64`,
    ///
    pub jackpot: f64,
    //     /// pub ptr: `Referent`, //⚡️
    /// pub ptr: `Option<Referent>`,
    ///
    //     pub ptr: Uuid, //⚡️
    pub ptr: Option<Uuid>,
}
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"c-struct-definition"}}}

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"c-new_impl"}}}
impl C {
    //     // {"magic":"","kind":{"CriticalBlockBegin":{"tag":"c-new_impl"}}} //⚡️
    /// Inter a new C and return it's `id`
    ///
    //     // {"magic":"","kind":{"IgnoreBlockBegin":{}}} //⚡️
    // {"magic":"","kind":{"IgnoreBlockBegin":{}}}
    /// # Example
    ///
    ///```
    /// # use test_models::one_to_many::Referent;
    /// # use test_models::one_to_many::C;
    /// # let mut store = test_models::one_to_many::ObjectStore::new();
    ///
    /// let ignorant_frame = "devilish_produce".to_owned();
    /// let referent_uap = Referent::new(&mut store, ignorant_frame);
    ///
    /// let c = C::new(&mut store, &referent_uap, 42.0);
    ///```
    // {"magic":"","kind":"IgnoreBlockEnd"}
    //     pub fn new(store: &mut ObjectStore, ptr: &Referent, jackpot: f64) -> Self { //⚡️
    pub fn new(store: &mut ObjectStore, ptr: Option<&Referent>, jackpot: f64) -> Self {
        let id = Uuid::new_v5(&UUID_NS, format!("{:?}::{}::", ptr, jackpot,).as_bytes());
        let new = Self {
            id,
            //             ptr: ptr.id, //⚡️
            ptr: ptr.map(|o| o.id),
            jackpot,
        };

        store.inter_c(new.clone());

        new
    }
    // {"magic":"","kind":{"CriticalBlockEnd":{"tag":"c-new_impl"}}}
}

/// The object of so many relationships
///
/// I’m related to stuff.
///
// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"referent-struct-definition"}}}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Referent {
    /// pub id: `Uuid`,
    ///
    pub id: Uuid,
    /// pub name: `std::string::String`,
    ///
    pub name: std::string::String,
}
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"referent-struct-definition"}}}

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"referent-new_impl"}}}
impl Referent {
    //     // {"magic":"","kind":{"CriticalBlockBegin":{"tag":"referent-new_impl"}}} //⚡️
    /// Inter a new Referent and return it's `id`
    ///
    //     // {"magic":"","kind":{"IgnoreBlockBegin":{}}} //⚡️
    // {"magic":"","kind":{"IgnoreBlockBegin":{}}}
    /// # Example
    ///
    ///```
    /// # use test_models::one_to_many::Referent;
    /// # let mut store = test_models::one_to_many::ObjectStore::new();
    ///
    /// let warm_riddle = "sore_achieve".to_owned();
    ///
    /// let referent = Referent::new(&mut store, warm_riddle);
    ///```
    // {"magic":"","kind":"IgnoreBlockEnd"}
    pub fn new(store: &mut ObjectStore, name: std::string::String) -> Self {
        let id = Uuid::new_v5(&UUID_NS, format!("{}::", name,).as_bytes());
        let new = Self { id, name };

        store.inter_referent(new.clone());

        new
    }
    // {"magic":"","kind":{"CriticalBlockEnd":{"tag":"referent-new_impl"}}}
}
