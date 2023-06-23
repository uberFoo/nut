//! One to Many Domain
//!
//! This file will eventually be generated.
//!
//! The purpose of this domain is to help me develop and test domain code generation.
//! It contains the following model:
#![cfg_attr(feature = "doc-images",
cfg_attr(all(),
doc = ::embed_doc_image::embed_image!("imported_object", "models/imported_object.png")))]
#![cfg_attr(
    not(feature = "doc-images"),
    doc = "**Doc images not enabled**. Compile with feature `doc-images` and Rust version >= 1.54 \
           to enable."
)]
//!
//! ![Imported Object Test Model][imported_object]
use uuid::{uuid, Uuid};

pub mod macros;
pub mod store;
pub mod types;

pub use store::ObjectStore;
pub use types::*;
pub use macros::*;

// imported_object
pub const UUID_NS: Uuid = uuid!("f1b24300-39bc-5928-ab66-116fd36296b1");

#[cfg(test)]
mod tests {
    use super::*;

    use nut::sarzak::Object;

    #[test]
    fn test() {
        let mut store = ObjectStore::new();

        let obj = Object::default();
        let _ao = AnotherObject::new(&mut store, &obj);
    }
}
