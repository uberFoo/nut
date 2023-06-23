//! Types for instances of the "One To One" domain
//! # Domain Description
//!
//! Domain to test the many flavors of 1-1 relationships.
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
use crate::one_to_one::store::ObjectStore;
use crate::one_to_one::UUID_NS;
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"imports"}}}

/// A: Referrer with Conditional [`Referent`]
///
/// This type is related to the [`Referent`] across a conditional relationship. This is 1-1c
///, and given that I am the referrer, I have the referential attribute/I am formalizing the
/// relationship. I think I prefer the latter language, but the former is very descriptive.
///..
///
// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"a-struct-definition"}}}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct A {
    /// pub id: `Uuid`,
    ///
    pub id: Uuid,
    /// pub number: `i64`,
    ///
    pub number: i64,
    //     /// pub ptr: `Referent`, //⚡️
    /// pub ptr: `Option<Referent>`,
    ///
    //     pub ptr: Uuid, //⚡️
    pub ptr: Option<Uuid>,
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
    /// # use test_models::one_to_one::A;
    /// # use test_models::one_to_one::Referent;
    /// # let mut store = test_models::one_to_one::ObjectStore::new();
    ///
    /// let festive_truck = "racial_meat".to_owned();
    /// let referent_skb = Referent::new(&mut store, festive_truck);
    ///
    /// let a = A::new(&mut store, &referent_skb, 42);
    ///```
    // {"magic":"","kind":"IgnoreBlockEnd"}
    //     pub fn new(store: &mut ObjectStore, ptr: &Referent, number: i64) -> Self { //⚡️
    pub fn new(store: &mut ObjectStore, ptr: Option<&Referent>, number: i64) -> Self {
        let id = Uuid::new_v5(&UUID_NS, format!("{:?}::{}::", ptr, number,).as_bytes());
        let new = Self {
            id,
            //             ptr: ptr.id, //⚡️
            ptr: ptr.map(|o| o.id),
            number,
        };

        store.inter_a(new.clone());

        new
    }
    // {"magic":"","kind":{"CriticalBlockEnd":{"tag":"a-new_impl"}}}
}

/// B: Referrer Unconditional to Referent
///
/// This is a plain Jayne 😉 1-1 relationship, where this guy is formalizing.
///
// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"b-struct-definition"}}}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct B {
    /// pub bit: `bool`,
    ///
    pub bit: bool,
    /// pub id: `Uuid`,
    ///
    pub id: Uuid,
    /// pub ptr: `Referent`,
    ///
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
    /// # use test_models::one_to_one::B;
    /// # use test_models::one_to_one::Referent;
    /// # let mut store = test_models::one_to_one::ObjectStore::new();
    ///
    /// let agonizing_boat = "complex_dock".to_owned();
    /// let referent_jbd = Referent::new(&mut store, agonizing_boat);
    ///
    /// let b = B::new(&mut store, &referent_jbd, true);
    ///```
    // {"magic":"","kind":"IgnoreBlockEnd"}
    pub fn new(store: &mut ObjectStore, ptr: &Referent, bit: bool) -> Self {
        let id = Uuid::new_v5(&UUID_NS, format!("{:?}::{}::", ptr, bit,).as_bytes());
        let new = Self {
            id,
            ptr: ptr.id,
            bit,
        };

        store.inter_b(new.clone());

        new
    }
    // {"magic":"","kind":{"CriticalBlockEnd":{"tag":"b-new_impl"}}}
}

/// C: Referrer to [`Referent`] Bi-Conditional
///
/// This will be an interesting one to translate. Hopefully not too gnarly.🤘
///
// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"c-struct-definition"}}}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct C {
    /// pub id: `Uuid`,
    ///
    pub id: Uuid,
    /// pub like_water: `f64`,
    ///
    pub like_water: f64,
    /// pub ptr: `Option<Referent>`,
    ///
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
    /// # use test_models::one_to_one::Referent;
    /// # use test_models::one_to_one::C;
    /// # let mut store = test_models::one_to_one::ObjectStore::new();
    ///
    /// let jaded_earthquake = "damp_gate".to_owned();
    /// let referent = Referent::new(&mut store, jaded_earthquake);
    ///
    /// let c = C::new(&mut store, Some(&referent), 42.0);
    ///```
    // {"magic":"","kind":"IgnoreBlockEnd"}
    pub fn new(store: &mut ObjectStore, ptr: Option<&Referent>, like_water: f64) -> Self {
        let id = Uuid::new_v5(&UUID_NS, format!("{:?}::{}::", ptr, like_water,).as_bytes());
        let new = Self {
            id,
            ptr: ptr.map(|o| o.id),
            like_water,
        };

        store.inter_c(new.clone());

        new
    }
    // {"magic":"","kind":{"CriticalBlockEnd":{"tag":"c-new_impl"}}}
}

/// The target of our relationship tests.
///
/// It is conditionally related to [`OneToOneConditional`] across _R2_, and it is unconditionally
/// related to [`OneToOneUnconditional`] across _R1_.
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
    /// # use test_models::one_to_one::Referent;
    /// # let mut store = test_models::one_to_one::ObjectStore::new();
    ///
    /// let uptight_body = "purring_rice".to_owned();
    ///
    /// let referent = Referent::new(&mut store, uptight_body);
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
