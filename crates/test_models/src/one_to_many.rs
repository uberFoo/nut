//! One to Many Domain
//!
//! This file will eventually be generated.
//!
//! The purpose of this domain is to help me develop and test domain code generation.
//! It contains the following model:
#![cfg_attr(feature = "doc-images",
cfg_attr(all(),
doc = ::embed_doc_image::embed_image!("one_to_many", "models/one_to_many.png")))]
#![cfg_attr(
    not(feature = "doc-images"),
    doc = "**Doc images not enabled**. Compile with feature `doc-images` and Rust version >= 1.54 \
           to enable."
)]
//!
//! ![One to Many Test Model][one_to_many]
use uuid::{uuid, Uuid};

pub mod macros;
pub mod store;
pub mod types;

pub use store::ObjectStore;
pub use types::*;
pub use macros::*;

// one_to_many
pub const UUID_NS: Uuid = uuid!("f2633df4-8cb2-5d43-b4aa-0ec7150bd928");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_r1() {
        let mut store = ObjectStore::new();

        let tgt_0 = Referent::new(&mut store, "Woogie".to_owned());
        let a_0 = A::new(&mut store, &tgt_0, "Iniya".to_owned());

        let tgt_1 = Referent::new(&mut store, "blubber".to_owned());
        let a_1 = A::new(&mut store, &tgt_1, "foo".to_owned());
        let a_2 = A::new(&mut store, &tgt_1, "bar".to_owned());

        // Test in the one direction.
        let tgt = one_to_many_get_one_tgt_across_r1!(a_0, store);
        assert_eq!(&tgt_0, tgt);

        let tgt = one_to_many_get_one_tgt_across_r1!(a_1, store);
        assert_eq!(&tgt_1, tgt);

        let tgt = one_to_many_get_one_tgt_across_r1!(a_2, store);
        assert_eq!(&tgt_1, tgt);

        // Test in the many direction
        let a_vec = one_to_many_get_many_as_across_r1!(tgt_0, store);
        assert_eq!(1, a_vec.len());

        // result contains a_0
        let an_a = a_vec.iter().find(|a| a.id == a_0.id);
        assert!(an_a.is_some());
        assert_eq!(Some(&&a_0), an_a);

        let a_vec = one_to_many_get_many_as_across_r1!(tgt_1, store);
        assert_eq!(2, a_vec.len());

        // result contains a_1
        let an_a = a_vec.iter().find(|a| a.id == a_1.id);
        assert!(an_a.is_some());
        assert_eq!(Some(&&a_1), an_a);

        // result contains a_2
        let an_a = a_vec.iter().find(|a| a.id == a_2.id);
        assert!(an_a.is_some());
        assert_eq!(Some(&&a_2), an_a);
    }

    #[test]
    fn test_r2() {
        let mut store = ObjectStore::new();

        let b_0 = B::new(&mut store, None, "oh no".to_owned());

        let tgt_1 = Referent::new(&mut store, "not".to_owned());
        let b_1 = B::new(&mut store, Some(&tgt_1), "more".to_owned());
        let b_2 = B::new(&mut store, Some(&tgt_1), "strings".to_owned());

        // Test in the one direction.
        let tgt = one_to_many_maybe_get_one_tgt_across_r2!(b_0, store);
        assert!(tgt.is_none());

        let tgt = one_to_many_maybe_get_one_tgt_across_r2!(b_1, store);
        assert!(tgt.is_some());
        assert_eq!(Some(&tgt_1), tgt, "tgt_1 is related to b_1");

        let tgt = one_to_many_maybe_get_one_tgt_across_r2!(b_2, store);
        assert!(tgt.is_some());
        assert_eq!(Some(&tgt_1), tgt, "tgt_1 is related to b_2");

        let b_vec = one_to_many_get_many_bs_across_r2!(tgt_1, store);
        assert_eq!(2, b_vec.len());

        // result contains b_1
        let a_b = b_vec.iter().find(|b| b.id == b_1.id);
        assert!(a_b.is_some());
        assert_eq!(Some(&&b_1), a_b, "b_1 is related to tgt_1");

        // result contains b_2
        let a_b = b_vec.iter().find(|b| b.id == b_2.id);
        assert!(a_b.is_some());
        assert_eq!(Some(&&b_2), a_b, "b_2 is related to tgt_1");
    }

    #[test]
    fn test_r3() {
        let mut store = ObjectStore::new();

        let tgt_0 = Referent::new(&mut store, "last".to_owned());

        let tgt_1 = Referent::new(&mut store, "few".to_owned());
        let c_1 = C::new(&mut store, &tgt_1, 3.141);
        let c_2 = C::new(&mut store, &tgt_1, 1.618);

        // Test in the one direction.
        let tgt = one_to_many_get_one_tgt_across_r3!(c_1, store);
        assert_eq!(&tgt_1, tgt);

        let tgt = one_to_many_get_one_tgt_across_r3!(c_2, store);
        assert_eq!(&tgt_1, tgt);

        // Test in the many direction
        let c_vec = one_to_many_maybe_get_many_cs_across_r3!(tgt_0, store);
        assert_eq!(0, c_vec.len());

        let c_vec = one_to_many_maybe_get_many_cs_across_r3!(tgt_1, store);
        assert_eq!(2, c_vec.len());

        // result contains c_1
        let a_c = c_vec.iter().find(|c| c.id == c_1.id);
        assert!(a_c.is_some());
        assert_eq!(Some(&&c_1), a_c);

        // result contains c_2
        let a_c = c_vec.iter().find(|a| a.id == c_2.id);
        assert!(a_c.is_some());
        assert_eq!(Some(&&c_2), a_c);
    }
}
