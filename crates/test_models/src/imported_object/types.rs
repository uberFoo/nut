//! Types for instances of the "Imported Object" domain
//! # Domain Description
//!
//! Domain to test importing an Object.
//!
//! We are importing an object from the sarzak domain. We do some sick stuff importing objects
//!...
//!
//!
//! # Contents
//!
//! The following types are defined herein:
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
//! ```shell
//!  /Users/uberfoo/projects/sarzak/nut/target/debug/deps/generate_test_domain-145fdb9ab1f4b4be --nocapture
//! ```
// {"magic":"","kind":"IgnoreBlockEnd"}
// {"magic":"","version":"0.5.0"}
// {"magic":"","version":"1.0.0"}
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"imports"}}}
use crate::imported_object::store::ObjectStore;
use crate::imported_object::UUID_NS;
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"imports"}}}

// Imported Objects
// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"imported-objects"}}}
use nut::sarzak::Object;
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"imported-objects"}}}

/// This is another object, but different.
///
/// As a side effect, this is going to test being able to collapse a type with a space. It will
/// break, and I’ll have a new feature.
///
///
// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"another_object-struct-definition"}}}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AnotherObject {
    /// pub id: `Uuid`,
    ///
    pub id: Uuid,
    /// Imported from the sarzak domain.
    /// [`nut::sarzak::Object`]
    ///
    pub ptr: Uuid,
}
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"another_object-struct-definition"}}}

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"another_object-new_impl"}}}
impl AnotherObject {
    //     // {"magic":"","kind":{"CriticalBlockBegin":{"tag":"another_object-new_impl"}}} //⚡️
    /// Inter a new AnotherObject and return it's `id`
    ///
    //     // {"magic":"","kind":{"IgnoreBlockBegin":{}}} //⚡️
    // {"magic":"","kind":{"IgnoreBlockBegin":{}}}
    /// # Example
    ///
    ///```
    /// # use nut::sarzak::Object;
    /// # use test_models::imported_object::AnotherObject;
    /// # let mut store = test_models::imported_object::ObjectStore::new();
    ///
    /// let object_rtb = Object::default();
    ///
    ///
    /// let another_object = AnotherObject::new(&mut store, &object_rtb);
    ///```
    // {"magic":"","kind":"IgnoreBlockEnd"}
    pub fn new(store: &mut ObjectStore, ptr: &Object) -> Self {
        let id = Uuid::new_v5(&UUID_NS, format!("{:?}::", ptr,).as_bytes());
        let new = Self { id, ptr: ptr.id };

        store.inter_another_object(new.clone());

        new
    }
    // {"magic":"","kind":{"CriticalBlockEnd":{"tag":"another_object-new_impl"}}}
}
