use std::{fs::File, path::PathBuf};

use clap::Parser;
use snafu::{prelude::*, Whatever};

use nut::model::{jsformat::RelationshipUI, ReadModel, WriteModel};

#[derive(Parser)]
#[command(author, version, about)]
#[command(propagate_version = true)]
struct Args {
    /// Input File
    ///
    /// Cuckoo input file you would like to shift.
    input: PathBuf,

    /// Output File
    ///
    /// Where you want the results. It's better to not overwrite the original IMO.
    output: PathBuf,

    /// Amount to shift vertically
    ///
    #[clap(long, allow_hyphen_values(true))]
    vert: i32,

    /// Amount to shift horizontally
    ///
    #[clap(long, allow_hyphen_values(true))]
    horiz: i32,
}

pub fn main() -> Result<(), Whatever> {
    let args = Args::parse();
    let x = args.horiz;
    let y = args.vert;

    let mut model = File::open(args.input)
        .whatever_context("can't open input file")?
        .from_json()
        .whatever_context("can't parse json")?;

    let id = model.paper.ids[0];
    for (_, rect) in &mut model.paper.entities.get_mut(&id).unwrap().objects {
        rect.x += x;
        rect.y += y;
    }

    for (_, rel) in &mut model.paper.entities.get_mut(&id).unwrap().relationships {
        match rel {
            RelationshipUI::BinaryUI(ref mut b) => {
                b.from.x += x;
                b.from.y += y;
                b.to.x += x;
                b.to.y += y;
            }
            RelationshipUI::IsaUI(ref mut i) => {
                i.from.x += x;
                i.from.y += y;
                for s in &mut i.to {
                    s.x += x;
                    s.y += y;
                }
            }
            RelationshipUI::AssociativeUI(ref mut a) => {
                a.from.x += x;
                a.from.y += y;
                a.one.x += x;
                a.one.y += y;
                a.other.x += x;
                a.other.y += y;
                a.middle.x += x;
                a.middle.y += y;
            }
        }
    }

    File::create(args.output)
        .whatever_context("can't open file for writing")?
        .to_json(&model)
        .whatever_context("problem writing json")?;

    Ok(())
}
