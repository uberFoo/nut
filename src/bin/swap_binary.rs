//! Fix my fuck up
//!
//! This program swaps the relationship phrases because I did them wrong in the
//! tool.
use std::{fs::File, path::PathBuf};

use clap::{command, value_parser, Arg};

use nut::{
    codegen::{ReadModel, WriteModel},
    Relationship_v0,
};

fn main() -> std::io::Result<()> {
    let args = command!()
        .arg(
            Arg::new("input_model")
                .required(true)
                .help("cuckoo model file to unfuck")
                .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            Arg::new("output_model")
                .required(true)
                .help("the unfucked model, hopefully")
                .value_parser(value_parser!(PathBuf)),
        )
        .get_matches();

    let input = args.get_one::<PathBuf>("input_model").unwrap();
    let mut model = File::open(input)?.from_json()?;

    // Find all the binary relationships and swap the relationship phrase.
    model
        .relationships
        .entities
        .iter_mut()
        .for_each(|(_, rel)| match rel {
            Relationship_v0::Binary(b) => {
                let to_phrase = b.to.description.clone();
                b.to.description = b.from.description.clone();
                b.from.description = to_phrase;
            }
            _ => {}
        });

    // Find all the binary relationships and swap the relationship conditionality.
    model
        .relationships
        .entities
        .iter_mut()
        .for_each(|(_, rel)| match rel {
            Relationship_v0::Binary(b) => {
                let to_cond = b.to.conditionality;
                b.to.conditionality = b.from.conditionality;
                b.from.conditionality = to_cond;
            }
            _ => {}
        });

    let output = args.get_one::<PathBuf>("output_model").unwrap();
    File::create(output)?.to_json(&model)?;

    Ok(())
}
