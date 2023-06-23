//! Isa Relationship Domain
//!
//! This file was generated by: `sarzak new "Isa Relationship"`.
//! It contains the following model:
#![cfg_attr(feature = "doc-images",
cfg_attr(all(),
doc = ::embed_doc_image::embed_image!("isa_relationship", "models/isa_relationship.png")))]
#![cfg_attr(
    not(feature = "doc-images"),
    doc = "**Doc images not enabled**. Compile with feature `doc-images` and Rust version >= 1.54 \
           to enable."
)]
//!
//! ![Isa Relationship Test Model][isa_relationship]
use uuid::{uuid, Uuid};

pub mod macros;
pub mod store;
pub mod types;

pub use store::ObjectStore;
pub use types::*;
pub use macros::*;

// Isa Relationship
pub const UUID_NS: Uuid = uuid!("fdd6c56b-f3fb-59ba-b387-31dd1ff762ea");

#[cfg(test)]
mod tests {
    //! I'm not sure what to test here. Making macros to traverse the Isa
    //! relationship is just weird. I don't think it makes any sense, but
    //! if it does in the future, I'll come back to it. For now, this
    //! half-baked attempt at a test. I hate deleting stuff. Bits are cheap.
    use super::*;

    #[test]
    fn test_r1() {
        // let mut store = ObjectStore::new();

        // What's the point of navigating this? So, let's agree that supertype
        // to subtype navigation is trivial. Let's go the other way. This is
        // opposite to the "owned property" of e.g., objects and attributes.
        // let sup = SimpleSupertype::SimpleSubtypeA(SimpleSubtypeA);
        // eprintln!("sup: {:?}", sup);

        // let sub = isa_relationship_get_one_SS_across_r1_from_SSA!(&SimpleSubtypeA, store);
        // This can't work, there aren't any instances in the store. No new
        // WTF?
        // let sub = store.exhume_simple_supertype(&SimpleSubtypeA);
        // eprintln!("sub: {:?}", sub);

        // assert_eq!(sub, Some(&sup));
    }
}