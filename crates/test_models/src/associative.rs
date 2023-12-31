//! Associative Domain
//!
//! This file was generated by: `sarzak new "associative"`.
//! The purpose of this domain is to help me develop and test domain code generation.
//! It contains the following model:
#![cfg_attr(feature = "doc-images",
cfg_attr(all(),
doc = ::embed_doc_image::embed_image!("associative", "models/associative.png")))]
#![cfg_attr(
    not(feature = "doc-images"),
    doc = "**Doc images not enabled**. Compile with feature `doc-images` and Rust version >= 1.54 \
           to enable."
)]
//!
//! ![Associative Relationship/Object Test Model][associative]
use uuid::{uuid, Uuid};

pub mod macros;
pub mod store;
pub mod types;

pub use store::ObjectStore;
pub use types::*;
pub use macros::*;

// associative
pub const UUID_NS: Uuid = uuid!("78411374-4d65-54a9-a68a-cecf90597189");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_r10() {
        let mut store = ObjectStore::new();

        let ui0 = IsaUi::new(&mut store, 0);
        let a0 = Anchor::new(&mut store, 0);
        let a1 = Anchor::new(&mut store, 1);
        let a2 = Anchor::new(&mut store, 2);
        let _ = Anchor::new(&mut store, 3);
        let _ = SubtypeAnchor::new(&mut store, &a0, &ui0);
        let _ = SubtypeAnchor::new(&mut store, &a1, &ui0);

        let sa_set = associative_get_many_anch_across_r10!(ui0, store);
        assert_eq!(sa_set.len(), 2);
        assert!(sa_set.iter().find(|&x| **x == a0).is_some());
        assert!(sa_set.iter().find(|&x| **x == a1).is_some());
        assert!(sa_set.iter().find(|&x| **x == a2).is_none());

        let ui = associative_maybe_get_one_iui_across_r10!(a0, store);
        assert_eq!(Some(&ui0), ui);

        let ui = associative_maybe_get_one_iui_across_r10!(a2, store);
        assert!(ui.is_none());
    }

    #[test]
    fn test_r20() {
        let mut store = ObjectStore::new();

        let s0 = State::new(&mut store, "foo".to_owned());
        let s1 = State::new(&mut store, "bar".to_owned());
        let s2 = State::new(&mut store, "baz".to_owned());

        let e0 = Event::new(&mut store, "one".to_owned());
        let e1 = Event::new(&mut store, "two".to_owned());
        let e2 = Event::new(&mut store, "three".to_owned());

        let _ = AcknowledgedEvent::new(&mut store, &s0, &e1);
        let _ = AcknowledgedEvent::new(&mut store, &s0, &e2);

        let _ = AcknowledgedEvent::new(&mut store, &s1, &e2);

        let many = associative_get_many_s_across_r20!(e0, store);
        assert_eq!(many.len(), 0);

        let many = associative_get_many_s_across_r20!(e1, store);
        assert_eq!(many.len(), 1);
        assert!(many.iter().find(|&&x| *x == s0).is_some());

        let many = associative_get_many_s_across_r20!(e2, store);
        assert_eq!(many.len(), 2);
        assert!(many.iter().find(|&&x| *x == s0).is_some());
        assert!(many.iter().find(|&&x| *x == s1).is_some());

        let more = associative_get_many_e_across_r20!(s0, store);
        assert_eq!(more.len(), 2);
        assert!(more.iter().find(|&&x| *x == e1).is_some());
        assert!(more.iter().find(|&&x| *x == e2).is_some());

        let more = associative_get_many_e_across_r20!(s1, store);
        assert!(more.iter().find(|&&x| *x == e2).is_some());
        assert_eq!(more.len(), 1);

        let more = associative_get_many_e_across_r20!(s2, store);
        assert_eq!(more.len(), 0);
    }
}
