use std::any::Any;

use clap::Args;
use log::debug;
use serde::{Deserialize, Serialize};

use nut::domain::{generate_macros, generate_store, generate_types};
use nut::sarzak::mc::{ModelCompilerError, ModelCompilerOptions, SarzakModelCompiler};

const TYPES: &str = "types";
const MACROS: &str = "macros";
const STORE: &str = "store";

const RS_EXT: &str = "rs";

const DEFAULT_META: bool = false;
const DEFAULT_DOC_TESTS: bool = true;
const DEFAULT_NEW: bool = true;
const DEFAULT_EXTRUDE: bool = false;
const DEFAULT_IGNORE_IGNORE: bool = false;

macro_rules! extract_options {
    ($options:ident; $(($option:ident, $default:expr)),+ ) => {
        $(
            let $option = match $options.$option {
                Some(b) => b,
                None => $default
            };
        )+
    };
}

#[derive(Args, Clone, Debug, Deserialize, Serialize)]
pub struct SarzakCompilerOptions {
    /// Enable output for domains sarzak and drawing
    ///
    /// Specifically this flag affects how objects are imported across domains.
    #[arg(long, short)]
    pub meta: Option<bool>,
    /// Generate documentation tests
    ///
    /// Currently this includes tests for the `new` associated function on generated
    /// structs. A function `test_default` is generated for enums, that creates
    /// instances, in a manner similar to `new` for structs.
    ///
    /// Tests are also generated for the relationship navigation macros.
    #[arg(long, short)]
    pub doc_tests: Option<bool>,
    /// Control emitting new implementations
    ///
    /// This is orthogonal to `doc_tests`. While the latter relies on this,
    /// this does not rely on it.
    #[arg(long, short)]
    pub new: Option<bool>,
    /// Generate Extrude Implementations
    ///
    /// This will generate an Extrude implementation for each Object for the
    /// transition to sarzak 1.0.  This is almost definitely useless otherwise.
    #[arg(long, short)]
    pub extrude: Option<bool>,
    /// Force Output Ignored Blocks
    ///
    /// Ignored blocks are used to keep random variable names and values from
    /// triggering differences with git. However, when the model changes, most
    /// likely these will need to be regenerated as well. This is how you do it.
    ///
    /// Maybe it would have been better to call this force_docs, or something?
    #[arg(long, short)]
    pub ignore_ignore: Option<bool>,
}

impl Default for SarzakCompilerOptions {
    fn default() -> Self {
        Self {
            meta: Some(DEFAULT_META),
            doc_tests: Some(DEFAULT_DOC_TESTS),
            new: Some(DEFAULT_NEW),
            extrude: Some(DEFAULT_EXTRUDE),
            ignore_ignore: Some(DEFAULT_IGNORE_IGNORE),
        }
    }
}

impl ModelCompilerOptions for SarzakCompilerOptions {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct ModelCompiler {}

impl ModelCompiler {
    pub fn new() -> Self {
        Self {}
    }
}

impl SarzakModelCompiler for ModelCompiler {
    fn compile(
        &self,
        model: &nut::sarzak::SarzakModel,
        package: &str,
        output: &std::path::PathBuf,
        options: Box<&dyn ModelCompilerOptions>,
        test: bool,
    ) -> Result<(), ModelCompilerError> {
        if let Some(options) = options.as_any().downcast_ref::<SarzakCompilerOptions>() {
            extract_options!(options;
                (meta, DEFAULT_META),
                (doc_tests, DEFAULT_DOC_TESTS),
                (new, DEFAULT_NEW),
                (extrude, DEFAULT_NEW),
                (ignore_ignore, DEFAULT_IGNORE_IGNORE));

            let mut module_path = output.clone();

            module_path.set_file_name(TYPES);
            module_path.set_extension(RS_EXT);

            debug!("Writing 🖍️ {:?}!", module_path);
            if !test {
                generate_types(
                    &model,
                    &module_path,
                    &package,
                    meta,
                    doc_tests,
                    new,
                    extrude,
                    ignore_ignore,
                )?;
            } else {
                debug!("Psych! 🙈");
            }

            // generate store.rs
            //
            module_path.set_file_name(STORE);
            module_path.set_extension(RS_EXT);
            debug!("Writing ✏️ {:?}!", module_path);
            if !test {
                generate_store(&model, &module_path, &package, meta, doc_tests)?;
            } else {
                debug!("Psych! 🙉");
            }

            // generate macros.rs
            //
            module_path.set_file_name(MACROS);
            module_path.set_extension(RS_EXT);
            debug!("Writing ✒️ {:?}!", module_path);
            if !test {
                generate_macros(
                    &model,
                    &module_path,
                    &package,
                    meta,
                    doc_tests,
                    ignore_ignore,
                )?;
            } else {
                debug!("Psych! 🙊");
            }

            Ok(())
        } else {
            Err(ModelCompilerError::CompilerError {
                description: "Passed incorrect compiler options.".to_owned(),
            })
        }
    }
}
