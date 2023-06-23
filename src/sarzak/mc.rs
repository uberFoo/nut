//! Model Compiler Primitives
//!
//! This should live in sarzak, but needs to live here to avoid Cargo dependency
//! cycles.
//!
use std::{any::Any, path::PathBuf};

use snafu::prelude::*;

use crate::{
    codegen::{CodeGenError, Error},
    sarzak::SarzakModel,
};

#[derive(Debug, Snafu)]
pub enum ModelCompilerError {
    #[snafu(display("ModelError: {}", description))]
    ModelError { description: String },
    #[snafu(display("I/O Error: {}, caused by {}", path.display(), source))]
    IOError {
        path: PathBuf,
        source: std::io::Error,
    },
    #[snafu(display("Compiler Error: {}", description))]
    CompilerError { description: String },
}

impl From<Error> for ModelCompilerError {
    fn from(error: Error) -> Self {
        match error.0 {
            CodeGenError::LoadCuckooModel { path, source } => Self::IOError { path, source },
            CodeGenError::FileWrite { path, source } => Self::IOError { path, source },
            CodeGenError::FileCreate { path, source } => Self::IOError { path, source },
            CodeGenError::ContextNoPath => Self::CompilerError {
                description: "Context is missing an output path.".to_owned(),
            },
            CodeGenError::ContextPathBeingStubborn { path, source } => {
                Self::IOError { path, source }
            }
            CodeGenError::Badness { source } => Self::CompilerError {
                description: source.to_string(),
            },
            CodeGenError::SerdeJsonBombed { source } => Self::CompilerError {
                description: format!("Serde failed: {}", source),
            },
            CodeGenError::SpawnRustfmt { source } => Self::CompilerError {
                description: source.to_string(),
            },
            CodeGenError::RustFmt { exit_code } => Self::CompilerError {
                description: format!("RustFmt failed with exit {:?}", exit_code),
            },
        }
    }
}

pub trait ModelCompilerOptions {
    fn as_any(&self) -> &dyn Any;
}

pub trait SarzakModelCompiler {
    fn compile(
        &self,
        model: &SarzakModel,
        package: &str,
        output: &PathBuf,
        options: Box<&dyn ModelCompilerOptions>,
        test: bool,
    ) -> Result<(), ModelCompilerError>;
}
