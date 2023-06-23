//! Generate ObjectStore for some domain
//!
//! This is where all of the instances from the domain model reside.
use std::{env, path::PathBuf};

use clap::{command, value_parser, Arg};

use nut::codegen::{Result, SarzakModel};
use nut::domain::generate_store;

fn main() -> Result<()> {
    let args = command!()
        .arg(
            Arg::new("input_model")
                .required(true)
                .help("domain model file")
                .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            Arg::new("output_file")
                .required(true)
                .help("generated code file")
                .value_parser(value_parser!(PathBuf)),
        )
        .get_matches();

    let model_path = args.get_one::<PathBuf>("input_model").unwrap();
    let output_path = args.get_one::<PathBuf>("output_file").unwrap();

    let model = SarzakModel::load_cuckoo_model(&model_path).unwrap();

    generate_store(&model, &output_path, "", false, true)
}
