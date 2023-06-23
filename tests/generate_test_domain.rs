//! Test to Generate Test Domains
//!
//! The idea is to run code gen on the test domains as part of the test. Maybe
//! get our code coverage number up!
//!
//! This is exactly what `sarzak gen` would do as of 1/16/23. In fact I cribbed
//! the code from there. But I wrote it, so I figure that's cool. 😎
use std::{fs, path::PathBuf};

use anyhow::{Context, Result};
use env_logger;
use log::debug;

use nut::codegen::SarzakModel;
use nut::domain::{generate_macros, generate_store, generate_types};

const TYPES: &str = "types";
const MACROS: &str = "macros";
const STORE: &str = "store";

const RS_EXT: &str = "rs";
const JSON_EXT: &str = "json";

#[test]
fn test_as_much_as_i_can() -> Result<()> {
    let _ = env_logger::builder().is_test(true).try_init();

    let package_root = PathBuf::from("crates/test_models");
    let model_dir = PathBuf::from("crates/test_models/models");

    // Iterate over all of the model files
    for entry in fs::read_dir(&model_dir)? {
        let path = &entry?.path();
        if let Some(ext) = path.extension() {
            if ext == "json" {
                generate_domain_code(&package_root, &path, false, false, true, true, false)?;
            }
        }
    }

    Ok(())
}

/// Generate types.rs, store.rs, and macros.rs
///
/// There is an assumption here that the model file is named the same as the
/// module, and all of it's files. This assumption holds true assuming it was
/// all setup with this program.
fn generate_domain_code(
    root: &PathBuf,
    model_file: &PathBuf,
    meta: bool,
    test_mode: bool,
    doc_tests: bool,
    new_impl: bool,
    extrude_impl: bool,
) -> Result<()> {
    // Check that the path exists, and that it's a file. From there we just
    // have to trust...
    anyhow::ensure!(
        model_file.exists(),
        format!("😱 Model file ({:?}) does not exist!", model_file)
    );
    anyhow::ensure!(
        model_file.is_file(),
        format!("😱 {:?} is not a model file!", model_file)
    );
    if let Some(extension) = model_file.extension() {
        anyhow::ensure!(
            extension == JSON_EXT,
            format!("😱 {:?} is not a json file!", model_file)
        );
    } else {
        anyhow::bail!(format!("😱 {:?} is not a json file!", model_file));
    }

    let module = if let Some(stem) = model_file.file_stem() {
        stem
    } else {
        anyhow::bail!(format!(
            "😱 Cannot extract the module name from the model file: {:?}!",
            model_file
        ));
    };

    let model = SarzakModel::load_cuckoo_model(&model_file)
        .context(format!("😱 reading model file {}", model_file.display()))?;

    println!(
        "Generating 🧬 code for domain ✨{:?}✨!",
        module.to_string_lossy()
    );
    debug!("Generating 🧬 code for domain, {:?}!", model_file);

    let mut module_path = root.clone();
    module_path.push("src");
    module_path.push(&module);
    module_path.push("fubar");

    let package = root
        .as_path()
        .components()
        .last()
        .unwrap()
        .as_os_str()
        .to_string_lossy();

    // generate types.rs
    //
    module_path.set_file_name(TYPES);
    module_path.set_extension(RS_EXT);
    debug!("Writing 🖍️ {:?}!", module_path);
    if !test_mode {
        generate_types(
            &model,
            &module_path,
            &package,
            meta,
            doc_tests,
            new_impl,
            extrude_impl,
            false,
        )?;
    }

    // generate store.rs
    //
    module_path.set_file_name(STORE);
    module_path.set_extension(RS_EXT);
    debug!("Writing ✏️ {:?}!", module_path);
    if !test_mode {
        generate_store(&model, &module_path, &package, meta, doc_tests)?;
    }

    // generate macros.rs
    //
    module_path.set_file_name(MACROS);
    module_path.set_extension(RS_EXT);
    debug!("Writing ✒️ {:?}!", module_path);
    if !test_mode {
        generate_macros(&model, &module_path, &package, meta, doc_tests, false)?;
    }

    Ok(())
}
