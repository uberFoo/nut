//! Types for instances of the "Everything" domain
//! # Domain Description
//!
//! Domain to test an Object with attributes of all types.
//!
//!
//! # Contents
//!
//! The following types are defined herein:
//!    * [`Everything`]
//!    * [`RandoObject`]
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
use crate::everything::store::ObjectStore;
use crate::everything::UUID_NS;
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"imports"}}}

/// An object, with everything on it!
///
// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"everything-struct-definition"}}}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Everything {
    /// pub bool: `bool`,
    ///
    pub bool: bool,
    /// pub float: `f64`,
    ///
    pub float: f64,
    /// pub id: `Uuid`,
    ///
    pub id: Uuid,
    /// pub int: `i64`,
    ///
    pub int: i64,
    /// pub string: `std::string::String`,
    ///
    pub string: std::string::String,
    /// pub rando: `Rando Object`,
    ///
    pub rando: Uuid,
}
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"everything-struct-definition"}}}

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"everything-new_impl"}}}
impl Everything {
    //     // {"magic":"","kind":{"CriticalBlockBegin":{"tag":"everything-new_impl"}}} //⚡️
    /// Inter a new Everything and return it's `id`
    ///
    //     // {"magic":"","kind":{"IgnoreBlockBegin":{}}} //⚡️
    // {"magic":"","kind":{"IgnoreBlockBegin":{}}}
    /// # Example
    ///
    ///```
    /// # use test_models::everything::Everything;
    /// # use test_models::everything::RandoObject;
    /// # let mut store = test_models::everything::ObjectStore::new();
    ///
    /// let rando_object_wtd = RandoObject::new(&mut store);
    /// let stiff_point = "spiffy_payment".to_owned();
    ///
    /// let everything = Everything::new(&mut store, &rando_object_wtd, 42, true, 42.0, stiff_point);
    ///```
    // {"magic":"","kind":"IgnoreBlockEnd"}
    pub fn new(
        store: &mut ObjectStore,
        rando: &RandoObject,
        //         float: f64, //⚡️
        int: i64,
        //         string: std::string::String, //⚡️
        bool: bool,
        float: f64,
        string: std::string::String,
    ) -> Self {
        let id = Uuid::new_v5(
            &UUID_NS,
            //             format!("{:?}::{}::{}::{}::{}::", rando, float, int, string, bool,).as_bytes(), //⚡️
            format!("{:?}::{}::{}::{}::{}::", rando, int, bool, float, string,).as_bytes(),
        );
        let new = Self {
            id,
            rando: rando.id,
            //             float, //⚡️
            int,
            //             string, //⚡️
            bool,
            float,
            string,
        };

        store.inter_everything(new.clone());

        new
    }
    // {"magic":"","kind":{"CriticalBlockEnd":{"tag":"everything-new_impl"}}}
}

/// Just some random object with which we wish to relate
///
/// How tawdry.
///
// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"rando_object-struct-definition"}}}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct RandoObject {
    /// pub id: `Uuid`,
    ///
    pub id: Uuid,
}
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"rando_object-struct-definition"}}}

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"rando_object-new_impl"}}}
impl RandoObject {
    //     // {"magic":"","kind":{"CriticalBlockBegin":{"tag":"rando_object-new_impl"}}} //⚡️
    /// Inter a new RandoObject and return it's `id`
    ///
    //     // {"magic":"","kind":{"IgnoreBlockBegin":{}}} //⚡️
    // {"magic":"","kind":{"IgnoreBlockBegin":{}}}
    /// # Example
    ///
    ///```
    /// # use test_models::everything::RandoObject;
    /// # let mut store = test_models::everything::ObjectStore::new();
    ///
    ///
    /// let rando_object = RandoObject::new(&mut store);
    ///```
    // {"magic":"","kind":"IgnoreBlockEnd"}
    pub fn new(store: &mut ObjectStore) -> Self {
        let id = Uuid::new_v5(&UUID_NS, format!("",).as_bytes());
        let new = Self { id };

        store.inter_rando_object(new.clone());

        new
    }
    // {"magic":"","kind":{"CriticalBlockEnd":{"tag":"rando_object-new_impl"}}}
}
