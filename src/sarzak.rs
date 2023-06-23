//! Sarzak OOA of OOA
//!
//! This surfaces the latest bit's of sarzak, rather than having to delve deeply into one of
//! the ooa_* crates.
//!
//! I'm trying to be careful to not use re-exports from the ooa_* crates.
use std::{
    cmp::{Ord, PartialOrd},
    fs::File,
    io,
    path::Path,
};

use log::{error, trace};
use random_string;
use serde::{Deserialize, Serialize};
use uuid::{uuid, Uuid};

use crate::codegen::{Config, ConfigValue};

#[macro_use]
mod macros;
pub mod mc;

pub use mc::*;

pub use get_obj_across_r17;
pub use get_r_from_across_r6;

pub use crate::ooa_0::{
    relationship::{AssociativeReferrer, Cardinality, Conditionality},
    Attribute, AttributeName, Type,
};
pub use crate::ooa_1::{
    object::{RelPointer, RelSide},
    relationship::{Referent, Referrer, Subtype, Supertype},
};
pub use crate::ooa_2::{Associative, AssociativeReferent, Binary, Isa, Object, Relationship};

use crate::codegen::{DrawingObjectStore, SarzakObjectStore};
use crate::model::{extract_ooa2, ReadModel};

// sarzak
pub const UUID_NS: Uuid = uuid!("daccabb9-eb3a-5cde-ba7c-19a3f22ab649");
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// A model from the tool
///
/// Which tool will be the question eventually. At that point we'll spin up another
/// version.
///
/// This contains the analysis as well as the bits necessary to render the model to
/// the screen.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SarzakModel {
    /// Model Serialization Version Number
    ///
    pub version: String,
    pub domain: String,
    pub id: Uuid,
    pub description: String,
    pub config: Config,
    pub sarzak: SarzakObjectStore,
    pub drawing: DrawingObjectStore,
    pub extents: [u16; 2],
    pub view: [i32; 2],
}

// TODO: I don't want to think about error handling just yet, but this needs
// a custom error type.
impl SarzakModel {
    pub fn load_cuckoo_model<P: AsRef<Path>>(path: P) -> std::io::Result<SarzakModel> {
        let cuckoo = ReadModel::from_json(&mut File::open(path.as_ref())?)?;

        let paper_id = cuckoo.paper.ids[0];
        let paper = cuckoo.paper.entities.get(&paper_id).unwrap();

        let mut config = match &paper.config {
            Some(c) => c.clone(),
            None => Config::new(),
        };

        // Load the compiler options from object descriptions.
        // This is until we have a new tool. Until then, and maybe afterwards too,
        // parse the object descriptions for config options.
        let sarzak = extract_ooa2(&cuckoo);
        sarzak.iter_object().for_each(|(id, obj)| {
            if obj.description.contains("❗️") {
                let mut iter = obj.description.split("❗️");
                // skip what's before the ❗️
                iter.next();
                if let Some(str) = iter.next() {
                    if let Some(opt) = str.split('\n').nth(0) {
                        if let Ok(c) = serde_json::from_str::<ConfigValue>(&opt) {
                            config.insert(*id, c);
                        } else {
                            // Good place to log an error, or at least a warning.
                            error!("😱 Unrecognized config option in {}: {}", obj.name, opt);
                        }
                    }
                }
            }
        });

        let mut model = SarzakModel {
            version: VERSION.to_owned(),
            domain: paper.domain_name.clone(),
            id: paper.id,
            description: paper.description.clone(),
            config,
            sarzak: SarzakObjectStore::new(),
            drawing: DrawingObjectStore::new(),
            extents: [paper.width, paper.height],
            view: [paper.offset.x, paper.offset.y],
        };

        // Damn, I'm doing some swizzling here...
        // I guess I'm doing this because of the borrow checker? Not really sure.
        model.sarzak = sarzak;
        model.drawing = cuckoo.into();

        Ok(model)
    }
}

pub trait ReadSarzakModel {
    fn from_json(&mut self) -> io::Result<SarzakModel>;
}

pub trait WriteSarzakModel
where
    Self: io::Write,
{
    fn to_json(&mut self, model: &SarzakModel) -> io::Result<()>;
}

impl<R: io::Read> ReadSarzakModel for R {
    fn from_json(&mut self) -> io::Result<SarzakModel> {
        let mut deserializer = serde_json::Deserializer::from_reader(self);
        Ok(SarzakModel::deserialize(&mut deserializer)?)
    }
}

impl<W: io::Write> WriteSarzakModel for W {
    fn to_json(&mut self, model: &SarzakModel) -> io::Result<()> {
        let mut serializer = serde_json::Serializer::new(self);
        model.serialize(&mut serializer)?;
        Ok(())
    }
}

// I'm not sure where these really belong...
// And I need to figure it out. This is messy.
use heck::{ToShoutySnakeCase, ToSnakeCase, ToUpperCamelCase};
pub const AS_TYPE: &str = "type";
pub const AS_IDENT: &str = "ident";
pub const AS_RIDENT: &str = "rando";
pub const AS_CONST: &str = "const";

pub const VAR_CHARS: &str = "abcdefghijklmnopqrstuvwxyz";

impl Object {
    pub fn render(&self, which: &str) -> String {
        self.name.render(which)
    }
}

impl Attribute {
    pub fn render(&self, which: &str) -> String {
        self.name.render(which)
    }
}

impl AttributeName {
    pub fn render(&self, which: &str) -> String {
        // This is a bit of a hack. I want identifiers to be changed from 'type'
        // to 'ty'.
        let name = if which == AS_IDENT {
            if let Some(n) = self.swizzle_name() {
                n
            } else {
                self.inner()
            }
        } else {
            self.inner()
        };

        match which {
            AS_TYPE => name.to_upper_camel_case(),
            AS_IDENT => name.to_snake_case(),
            AS_RIDENT => format!(
                "{}_{}",
                name.to_snake_case(),
                random_string::generate(3, VAR_CHARS)
            ),
            AS_CONST => name.to_shouty_snake_case(),
            _ => format!("unknown render type requested '{}'", which),
        }
    }

    pub fn swizzle_name(&self) -> Option<&str> {
        match self.inner().as_str() {
            "type" => {
                trace!("Changing attribute name from 'type' to 'ty'.");
                Some("ty")
            }
            "super" => {
                trace!("Changing attribute name from 'super' to 'uber'.");
                Some("uber")
            }
            "Type" => {
                // This isn't a type. It's necessary when rendering Type as an
                // identifier.
                trace!("Changing attribute name from 'Type' to 'ty'.");
                Some("ty")
            }
            _ => None,
        }
    }
}

// I tried creating a trait and then doing a blanket implementation, but I ran
// afoul of that rule that the type or trait must be local...
//
// It wouldn't have worked for Relationship anyway -- no id.
//
// These are needed to sort Objects.
impl Eq for Object {}
impl Ord for Object {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}
impl PartialOrd for Object {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// These are needed to sort Relationships.
impl Eq for Relationship {}
impl Ord for Relationship {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.get_id().cmp(&other.get_id())
    }
}
impl PartialOrd for Relationship {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// Attribute
impl Eq for Attribute {}
impl Ord for Attribute {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}
impl PartialOrd for Attribute {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
