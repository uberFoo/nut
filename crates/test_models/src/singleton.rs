//! One to Many Domain
//!
//! This file will eventually be generated.
//!
//! The purpose of this domain is to help me develop and test domain code generation.
//! It contains the following model:
#![cfg_attr(feature = "doc-images",
cfg_attr(all(),
doc = ::embed_doc_image::embed_image!("singleton", "models/singleton.png")))]
#![cfg_attr(
    not(feature = "doc-images"),
    doc = "**Doc images not enabled**. Compile with feature `doc-images` and Rust version >= 1.54 \
           to enable."
)]
//!
//! ![Singleton Test Model][singleton]
use uuid::{uuid, Uuid};

#[macro_use]
pub mod macros;
pub mod store;
pub mod types;

pub use store::ObjectStore;
pub use types::*;

// singleton
pub const UUID_NS: Uuid = uuid!("2407bf3d-a3b5-50da-936d-3c70ce883400");
