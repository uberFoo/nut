//! Types for instances of the "Singleton" domain
//! # Domain Description
//!
//! Domain to test the poor, lonely, Singleton type.
//!
//!
//! # Contents
//!
//! The following types are defined herein:
//!    * [`SINGLETON`]
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
use crate::singleton::store::ObjectStore;
use crate::singleton::UUID_NS;
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"imports"}}}

/// This is a singleton type
///
/// This means that there is only ever one instance of this type. There are restrictions on
/// what may constitute a singleton. It requires that there be only an `id` on the type.
///
/// ❗️{ "singleton_object": true, "imported_object": null }
///
//
pub const SINGLETON: Uuid = uuid!["852f152f-fbb2-5d5b-8ecb-99bceb2e940f"];
