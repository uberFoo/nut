use std::{collections::HashMap, io};

use serde::{Deserialize, Serialize};
use uuid::{uuid, Uuid};

pub mod attribute;
pub mod object;
pub mod relationship;

pub use attribute::{Attribute, AttributeName, Type};
pub use object::Object;
pub use relationship::{Cardinality, Conditionality, Relationship};

// ISO OID for "sarzak_ooa_0"
pub const UUID_NS: Uuid = uuid!("b49d6fe1-e5e9-5896-bd42-b72012429e52");

pub trait BaseClass {
    fn id(&self) -> Uuid;
}

/// "Type erase" our objects
///
/// This enum stores one of our objects. This allows us to pass around an array
/// of 'stuff'.
#[derive(Debug, Deserialize, Serialize)]
pub enum ObjectType {
    Object(object::Object),
    Relationship(relationship::Relationship),
}

/// An Cuckoo Model
///
/// This structure is what is output by cuckoo as a schema file. It's a very nice JSON
/// representation: it's a json object with keys "Relationship" and "Object". Each of
/// those fields contain an array of [ObjectType]. They are actually homogeneous in the
/// input JSON. So, why then can't we just use a `Vec<Object>`? Probably could have?
pub type CuckooModel = HashMap<String, Vec<ObjectType>>;

/// Read the Cuckoo
pub trait ReadCuckooModel {
    fn from_json(&mut self) -> io::Result<CuckooModel>;
}

impl<R: io::Read> ReadCuckooModel for R {
    fn from_json(&mut self) -> io::Result<CuckooModel> {
        let mut deserializer = serde_json::Deserializer::from_reader(self);
        Ok(CuckooModel::deserialize(&mut deserializer)?)
    }
}

pub trait WriteCuckooModel
where
    Self: io::Write,
{
    fn to_json(&mut self, schema: &CuckooModel) -> io::Result<()>;
}

impl<W: io::Write> WriteCuckooModel for W {
    fn to_json(&mut self, schema: &CuckooModel) -> io::Result<()> {
        let mut serializer = serde_json::Serializer::new(self);
        schema.serialize(&mut serializer)?;
        Ok(())
    }
}

/// ooa_0 Schema
///
/// This is a reorganized version of the model that we load from Cuckoo. Instead of
/// lists of things, we store them in hashmaps by UUID. That's the only difference.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Schema {
    pub version: String,
    pub objects: HashMap<Uuid, Object>,
    pub relationships: HashMap<Uuid, Relationship>,
}

impl From<CuckooModel> for Schema {
    fn from(value: CuckooModel) -> Self {
        // Unwrap the Vec<ObjectType::Object> into Vec<Object> to make access more pleasant.
        let objects = value
            .get("Objects")
            .expect("Invalid Schema")
            .iter()
            .map(|x| {
                if let ObjectType::Object(o) = x {
                    (o.id, o.clone())
                } else {
                    panic!("Invalid Schema")
                }
            })
            .collect::<Vec<(Uuid, Object)>>()
            .into_iter()
            .collect::<HashMap<Uuid, Object>>();

        // Here we put Relationships into a hashmap where the key is the relationship id.
        let relationships = value
            .get("Relationships")
            .expect("Invalid Schema")
            .iter()
            .map(|x| {
                if let ObjectType::Relationship(r) = x {
                    (r.get_id(), r.clone())
                } else {
                    panic!("Invalid Schema")
                }
            })
            .collect::<Vec<(Uuid, Relationship)>>()
            .into_iter()
            .collect::<HashMap<Uuid, Relationship>>();

        Self {
            version: "ooa_0".to_owned(),
            objects,
            relationships,
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
    fn to_json(&mut self, model: &Schema) -> io::Result<()>;
}

impl<W: io::Write> WriteSchema for W {
    fn to_json(&mut self, model: &Schema) -> io::Result<()> {
        let mut serializer = serde_json::Serializer::new(self);
        model.serialize(&mut serializer)?;
        Ok(())
    }
}
