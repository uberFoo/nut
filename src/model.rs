//! Model Extrusion
//!
//! The purpose of this module is to process the UI bits that are used by Cuckoo to render
//! boxen and lines. The underlying structure was ad hoc, and rushed. I've already created
//! a new model to represent the UI bits.
//!
//! There will likely be several submodules here that extrude the data into the new model.
//!
//! It just occurred to me that I should be able to generate the raw structure that I'm
//! extruding into! Now, that's exciting!
//!
pub mod jsformat;

pub use jsformat::{JSFormat, ReadModel, WriteModel};

/// Extract the ooa model information from the overarching Cuckoo Model.
///
use crate::codegen::SarzakObjectStore;
pub fn extract_ooa2(input: &JSFormat) -> SarzakObjectStore {
    let schema = crate::ooa_0::Schema {
        version: "ooa_1".to_owned(),
        objects: input.objects.entities.clone(),
        relationships: input.relationships.entities.clone(),
    };

    let ooa_0: crate::ooa_0::Schema = schema.into();
    let ooa_1: crate::ooa_1::Schema = ooa_0.into();
    let store: crate::ooa_2::ObjectStore = ooa_1.into();

    store
}
