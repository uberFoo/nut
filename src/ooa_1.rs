use std::{collections::HashMap, io};

use serde::{Deserialize, Serialize};
use uuid::{uuid, Uuid};

pub mod extrude;
pub mod object;
pub mod relationship;

pub use crate::ooa_0::{
    relationship::{Associative, Cardinality, Conditionality},
    Attribute,
};
pub use object::Object;
pub use relationship::Relationship;

use extrude::Extrude;

// ISO OID for "sarzak_ooa_1"
pub const UUID_NS: Uuid = uuid!("039ebc15-0ca8-5f17-b437-cd8937a3864a");

pub trait BaseClass {
    fn id(&self) -> Uuid;
}

/// Context for Object Extrusion
///
/// We don't need context for this round of extrusion.
struct ObjectContext<'a> {
    relationships: &'a HashMap<Uuid, Relationship>,
}

/// OOA_1 Schema
///
/// This is what a "Schema" looks like in ooa_1. We still load ooa_0 schema
/// files though, at least for now, as that's all that cuckoo exports.
/// So, I've stolen the definition of a schema from the `Model` definition
/// that I'm now writing out, so what's the point of this?
///
/// I'd like to see these indexed by String, where the value is the name of the
/// thing.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Schema {
    pub version: String,
    pub obj: HashMap<String, Uuid>,
    pub rel: HashMap<String, Uuid>,
    pub objects_: HashMap<Uuid, Object>,
    pub relationships_: HashMap<Uuid, Relationship>,
}

use crate::ooa_0;
impl From<ooa_0::Schema> for Schema {
    fn from(model: ooa_0::Schema) -> Self {
        // Relationship changed, so we need to convert.
        // Note that Relationship itself has no id, so the UUID we are indexing by is
        // actually the id of the enum variant.
        let relationships = model
            .clone()
            .relationships
            .into_iter()
            .map(|(id, rel)| (id, rel.into()))
            .collect::<Vec<(Uuid, Relationship)>>()
            .into_iter()
            .collect::<HashMap<Uuid, Relationship>>();

        // These are keyed by UUID for the relationship context.
        let objects = model
            .objects
            .into_iter()
            .map(|(id, o)| {
                let o_prime: Object = Object::extrude(
                    o,
                    &mut ObjectContext {
                        relationships: &relationships,
                    },
                );
                (id, o_prime)
            })
            .collect::<Vec<(Uuid, Object)>>()
            .into_iter()
            .collect::<HashMap<Uuid, Object>>();

        let obj = objects
            .clone()
            .into_iter()
            .map(|(_, o)| {
                let name = o.name.inner().clone();
                (name, o.id)
            })
            .collect::<Vec<(String, Uuid)>>()
            .into_iter()
            .collect::<HashMap<String, Uuid>>();

        let rel = relationships
            .clone()
            .into_iter()
            .map(|(_, r)| {
                let name = format!("R{}", r.get_number());
                (name, r.get_id())
            })
            .collect::<Vec<(String, Uuid)>>()
            .into_iter()
            .collect::<HashMap<String, Uuid>>();

        Schema {
            version: "ooa_1".to_owned(),
            obj,
            rel,
            objects_: objects,
            relationships_: relationships,
        }
    }
}

pub trait ReadSchema {
    fn from_json(&mut self) -> io::Result<Schema>;
}

impl<R: io::Read> ReadSchema for R {
    fn from_json(&mut self) -> io::Result<Schema> {
        let mut deserializer = serde_json::Deserializer::from_reader(self);
        Ok(Schema::deserialize(&mut deserializer)?)
    }
}

pub trait WriteSchema
where
    Self: io::Write,
{
    fn to_json(&mut self, schema: &Schema) -> io::Result<()>;
}

impl<W: io::Write> WriteSchema for W {
    fn to_json(&mut self, schema: &Schema) -> io::Result<()> {
        let mut serializer = serde_json::Serializer::new(self);
        schema.serialize(&mut serializer)?;
        Ok(())
    }
}
