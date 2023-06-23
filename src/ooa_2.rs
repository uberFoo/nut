use std::collections::HashMap;

use uuid::Uuid;

#[macro_use]
pub mod macros;
pub mod object;
mod object_store;
pub mod relationship;

pub use crate::ooa_0::{
    relationship::{Cardinality, Conditionality},
    Attribute, AttributeName,
};
pub use crate::ooa_1::{
    extrude::Extrude,
    relationship::{Referent, Referrer, Subtype, Supertype},
};

// Macro re-exports
pub use get_referent;
pub use get_referrer;
pub use get_subtypes;
pub use get_supertype;
pub use sarzak_maybe_get_one_r_sub_across_r15;
pub use sarzak_maybe_get_one_r_sup_across_r14;

pub use object::Object;
pub use object_store::{ObjectStore, WriteObjectStore};
pub use relationship::{Associative, AssociativeReferent, Binary, Isa, Relationship};

struct Context<'a>(&'a mut ObjectStore);

use crate::ooa_1;
impl From<ooa_1::Schema> for ObjectStore {
    fn from(schema: ooa_1::Schema) -> Self {
        let mut store = ObjectStore::new();
        let mut context = Context(&mut store);

        // This is first so that the ObjectStore has it's relationship tables filled
        // for the object interring.
        inter_relationships(schema.relationships_, &mut context);
        inter_objects(schema.objects_, &mut context);

        // This needs to be last for the borrow checker to be happy.
        let _ = schema.rel.into_iter().for_each(|(k, r)| {
            store.inter_rel(k, r);
        });

        store
    }
}

fn inter_objects(objs: HashMap<Uuid, ooa_1::Object>, mut context: &mut Context) {
    // Two passes, because one pass borrows mutably twice.
    objs.into_iter()
        .map(|(_, o)| Object::extrude(o, &mut context))
        .collect::<Vec<Object>>()
        .into_iter()
        .for_each(|o| context.0.inter_object(o));
}

fn inter_relationships(rels: HashMap<Uuid, ooa_1::Relationship>, mut context: &mut Context) {
    rels.into_iter()
        .map(|(_, r)| Relationship::extrude(r, &mut context))
        .collect::<Vec<Relationship>>()
        .into_iter()
        .for_each(|r| context.0.inter_relationship(r));
}
