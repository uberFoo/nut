//! Macros for traversing the ObjectStore Database
//!

/// Macro to get the Referent side of a Binary relationship
///
/// We are given a reference to the store, a reference to an Object, and the
/// name of the relationship to traverse. E.g.,
///
/// ```ignore
/// # use std::fs::File;
/// # use nut::codegen::{SarzakObjectStore};
/// # use nut::{ReadCuckooModel, Schema_v0, Schema_v1};
/// # use nut::codegen::get_referent;
/// # let cuckoo = File::open("schemas/cat_dog.json")
/// #     .unwrap()
/// #     .from_json()
/// #     .unwrap();
/// # let ooa_0: Schema_v0 = cuckoo.into();
/// # let ooa_1: Schema_v1 = ooa_0.into();
/// # let store: SarzakObjectStore = ooa_1.into();
/// # let dog = store.get_obj("Dog").unwrap();
/// let cat = get_referent!(dog("R2"), store);
/// ```
///
/// We also support the form wherein a UUID is passed in:
///
/// ```ignore
/// # use std::fs::File;
/// # use nut::codegen::{SarzakObjectStore};
/// # use nut::{ReadCuckooModel, Schema_v0, Schema_v1};
/// # use nut::codegen::get_referent;
/// # let cuckoo = File::open("schemas/cat_dog.json")
/// #     .unwrap()
/// #     .from_json()
/// #     .unwrap();
/// # let ooa_0: Schema_v0 = cuckoo.into();
/// # let ooa_1: Schema_v1 = ooa_0.into();
/// # let store: SarzakObjectStore = ooa_1.into();
/// # let dog = store.get_obj("Dog").unwrap();
/// # let r_ptr = dog.rels.get("R1").unwrap();
/// let rel = store.exhume_binary(&r_ptr.value).unwrap();
/// let referent = get_referent!(r_ptr, store);
/// ```
#[macro_export]
macro_rules! get_referent {
    ($obj:ident($rel:expr), $store:expr) => {
        $store
            .exhume_object(
                &$store
                    .exhume_referent(
                        &$store
                            .exhume_binary(&$obj.rels.get($rel).unwrap().value)
                            .unwrap()
                            .to,
                    )
                    .unwrap()
                    .obj_id,
            )
            .unwrap()
    };
    ($rel:expr, $store:expr) => {
        $store
            .exhume_object(
                &$store
                    .exhume_referent(&$store.exhume_binary(&$rel.value).unwrap().to)
                    .unwrap()
                    .obj_id,
            )
            .unwrap()
    };
}

/// Macro to get the Referrer side of a Binary relationship
///
/// We are given a reference to the store, a reference to an Object, and the
/// name of the relationship to traverse. E.g.,
///
/// ```ignore
/// let dog = get_referrer!(cat("R2"), store);
/// ```
///
/// We also support the form wherein a UUID is passed in:
///
/// ```ignore
///     let cat = store.get_obj("Dog").unwrap();
///     let r1 = cat.rels.get("R2").unwrap();
///     let dog = get_subtypes!(r1, store);
/// ```
///
/// The second form makes the most sense whilst generating code and you don't necessarily know
/// the name of the relationship. In fact the second form doesn't make much sense from
/// the perspective of code generation, because you probably won't be looking for a "Dog", or "R2".
/// It's still the preferred form when iterating over anonymous objects and relationships.
#[macro_export]
macro_rules! get_referrer {
    ($obj:ident($rel:expr), $store:expr) => {
        $store
            .exhume_object(
                &$store
                    .exhume_referrer(
                        &$store
                            .exhume_binary(&$obj.rels.get($rel).unwrap().value)
                            .unwrap()
                            .from,
                    )
                    .unwrap()
                    .obj_id,
            )
            .unwrap();
    };
    ($rel:expr, $store:expr) => {
        $store
            .exhume_object(
                &$store
                    .exhume_referrer(&$store.exhume_binary(&$rel.value).unwrap().from)
                    .unwrap()
                    .obj_id,
            )
            .unwrap();
    };
}

/// Macro to get the Supertype from an Isa relationship
///
/// We are given a reference to the store, a reference to an Object, and the
/// name of the relationship to traverse. E.g.,
///
/// ```ignore
/// let animal = get_supertype!(cat("R1"), store);
/// ```
#[macro_export]
macro_rules! get_supertype {
    ($obj:ident($rel:expr), $store:expr) => {
        $store
            .exhume_object(
                &$store
                    .exhume_supertype(
                        &$store
                            .exhume_isa(&$obj.rels.get($rel).unwrap().value)
                            .unwrap()
                            .supertype,
                    )
                    .unwrap()
                    .obj_id,
            )
            .unwrap();
    };
}

/// Macro to get the Subtypes from an Isa relationship
///
/// We are given a reference to the store, a reference to an Object, and the
/// name of the relationship to traverse. E.g.,
///
/// ```ignore
/// let animal = get_supertype!(cat("R1"), store);
/// ```
///
/// We also support the form wherein a UUID is passed in:
///
/// ```ignore
///     let animal = store.get_obj("Animal").unwrap();
///     let r1 = animal.rels.get("R1").unwrap();
///     let animals = get_subtypes!(r1.value, store);
/// ```
///
/// I tried using a [RelPointer][rp] instead of a UUID, but the macro won't compile
/// that way. I have no idea why, it's just supposed to be a stupid macro.
///
/// [rp]: crate::ooa_1::object::RelPointer
#[macro_export]
macro_rules! get_subtypes {
    ($obj:ident($rel:expr), $store:expr) => {
        &$store
            .exhume_isa(&$obj.rels.get($rel).unwrap().value)
            .unwrap()
            .subtypes
            .iter()
            .map(|sub| $store.exhume_subtype(&sub).unwrap().obj_id)
            .map(|obj| $store.exhume_object(&obj).unwrap())
            .collect::<Vec<&nut::sarzak::Object>>()
    };
    ($rel:expr, $store:expr) => {{
        $store
            .exhume_isa(&$rel)
            .unwrap()
            .subtypes
            .iter()
            .map(|sub| $store.exhume_subtype(&sub).unwrap().obj_id)
            .map(|obj| $store.exhume_object(&obj).unwrap())
            .collect::<Vec<&Object>>()
    }};
}

/// Macro to traverse [`Object`][🦀] ➡ [`Supertype`][🦞], via _R14(c)_
///
/// This macro expects a &[`Object`][🦀], and returns an Option<&[`Supertype`][🦞]>.
///
/// Generated by `nut::domain::generate_macros::generate_binary_macro_referent_to_referrer_imp`
///
/// [🦀]: crate::sarzak::types::Object
/// [🦞]: crate::sarzak::types::Supertype
#[macro_export]
macro_rules! sarzak_maybe_get_one_r_sup_across_r14 {
    ($input:expr, $store:expr) => {{
        // nut::codegen::template::macros::emit_one_conditional_lookup
        $store
            .sarzak
            .iter_supertype()
            .find(|(_, z)| z.obj_id == $input.id)
            .map(|(_, z)| z)
    }};
}

/// Macro to traverse [`Object`][🦀] ➡ [`Subtype`][🦞], via _R15(c)_
///
/// This macro expects a &[`Object`][🦀], and returns an Option<&[`Subtype`][🦞]>.
///
/// Generated by `nut::domain::generate_macros::generate_binary_macro_referent_to_referrer_imp`
///
/// [🦀]: crate::sarzak::types::Object
/// [🦞]: crate::sarzak::types::Subtype
#[macro_export]
macro_rules! sarzak_maybe_get_one_r_sub_across_r15 {
    ($input:expr, $store:expr) => {{
        // nut::codegen::template::macros::emit_one_conditional_lookup
        $store
            .sarzak
            .iter_subtype()
            .find(|z| z.1.obj_id == $input.id)
            .map(|(_, z)| z)
    }};
}
