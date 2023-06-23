use std::{fs::File, path::PathBuf};

use clap::{command, value_parser, Arg};

use nut::{ReadSchema_v0, Schema_v0, Schema_v1, WriteSchema_v1};

fn main() -> std::io::Result<()> {
    let args = command!()
        .arg(
            Arg::new("input_schema")
                .required(true)
                .help("input file that contains the ooa_0 schema")
                .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            Arg::new("output_schema")
                .required(true)
                .help("output file to write the ooa_1 schema")
                .value_parser(value_parser!(PathBuf)),
        )
        .get_matches();

    let path = args.get_one::<PathBuf>("input_schema").unwrap();

    let schema: Schema_v0 = File::open(path)?.from_json()?;
    let schema: Schema_v1 = schema.into();

    let path = args.get_one::<PathBuf>("output_schema").unwrap();
    File::create(path)?.to_json(&schema)?;

    Ok(())
}
