//! Code Generation Doings
//!
//! This module is mostly re-exports of other interesting things for code generation,
//! pulled together from other modules. This will always contain the latest things,
//! which should make use of this crate easier, I hope.
use std::path::PathBuf;

use snafu::prelude::*;

pub mod config;
pub mod context;
#[macro_use]
pub mod macros;
pub mod template;
pub mod types;

pub use config::{Config, ConfigValue, ImportedObject, SingletonObject};
pub use context::{CachingContext, Context, Symbol};
pub use types::{Field, Ref};

// Macro re-exports
pub use begin_crit;
pub use emit;
pub use emitln;
pub use end_crit;
pub use func_name;
pub use function;

// Stuff from the Sarzak Domain
pub use crate::ooa_2::{
    get_referent, get_referrer, get_subtypes, get_supertype, sarzak_maybe_get_one_r_sub_across_r15,
    sarzak_maybe_get_one_r_sup_across_r14, Extrude, ObjectStore as SarzakObjectStore,
    WriteObjectStore,
};
pub use crate::sarzak::{ReadSarzakModel, SarzakModel, WriteSarzakModel, VERSION};

// Stuff from the Drawing Domain
pub use crate::drawing::{ObjectStore as DrawingObjectStore, UUID_NS as DRAWING_UUID_NS};

// Stuff from the "Model Domain", which is really just Cuckoo's output file.
pub use crate::model::{JSFormat, ReadModel, WriteModel};

// Templates for code generation
pub(crate) use template::{
    create_arg_string, emit_generated_code_comments,
    macros::{
        emit_assoc_many, emit_assoc_maybe_get_one, emit_associative_main, emit_binary_main,
        emit_many_conditional_lookup, emit_many_unconditional, emit_one_conditional,
        emit_one_conditional_lookup, emit_one_unconditional, emit_one_unconditional_lookup,
    },
    types::{emit_enum_main, emit_object_comments, emit_singleton, emit_struct_main},
};

#[derive(Debug, Snafu)]
pub struct Error(pub CodeGenError);

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Snafu)]
pub enum CodeGenError {
    #[snafu(display("😱 unable to load cuckoo model {}", path.display()))]
    LoadCuckooModel {
        path: PathBuf,
        source: std::io::Error,
    },
    #[snafu(display("😱 unable to write file {}", path.display()))]
    FileWrite {
        path: PathBuf,
        source: std::io::Error,
    },
    #[snafu(display("😱 unable to create file {}", path.display()))]
    FileCreate {
        path: PathBuf,
        source: std::io::Error,
    },
    #[snafu(display("😱 context missing output path"))]
    ContextNoPath,
    #[snafu(display("😱 context unable to open output path {}", path.display()))]
    ContextPathBeingStubborn {
        path: PathBuf,
        source: std::io::Error,
    },
    #[snafu(display("😱 something terrible is happening"))]
    Badness { source: std::io::Error },
    #[snafu(display("😱 serialization error -- somebody really hates us"))]
    SerdeJsonBombed { source: serde_json::Error },
    #[snafu(display("😱 unable to spawn rustfmt process"))]
    SpawnRustfmt { source: std::io::Error },
    #[snafu(display("😱 rustfmt error: {:?}", exit_code))]
    RustFmt { exit_code: Option<i32> },
}
