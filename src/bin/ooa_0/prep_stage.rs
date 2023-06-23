use std::{env, fs::File, path::PathBuf};

use clap::{command, value_parser, Arg};

use nut::{ReadCuckooModel, WriteSchema_v0};

fn main() -> std::io::Result<()> {
    let args = command!()
        .arg(
            Arg::new("input_model")
                .required(true)
                .help("input file that contains Cuckoo Model")
                .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            Arg::new("output_schema")
                .required(true)
                .help("output file to write the ooa_0 schema")
                .value_parser(value_parser!(PathBuf)),
        )
        .get_matches();

    // Read the model from the JSON schema
    let path = args.get_one::<PathBuf>("input_model").unwrap();
    let ooa = File::open(path)?.from_json()?;

    let model = ooa.into();

    let path = args.get_one::<PathBuf>("output_schema").unwrap();
    File::create(path)?.to_json(&model)?;

    Ok(())
}
