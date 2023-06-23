use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::codegen::Config;
use crate::ooa_0::{Object, Relationship};

#[derive(Debug, Deserialize, Serialize)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AnchorPoint {
    pub id: Uuid,
    pub dir: Direction,
    pub x: i32,
    pub y: i32,
    pub offset: Point,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: u16,
    pub height: u16,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum RelationshipUI {
    BinaryUI(PointTuple),
    IsaUI(Isa),
    AssociativeUI(Associative),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Isa {
    pub from: AnchorPoint,
    pub to: Vec<AnchorPoint>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Associative {
    pub from: Point,
    pub middle: AnchorPoint,
    pub one: AnchorPoint,
    pub other: AnchorPoint,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PointTuple {
    pub from: AnchorPoint,
    pub to: AnchorPoint,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Paper {
    pub id: Uuid,
    pub description: String,
    pub domain_name: String,
    pub domain_ns: String,
    pub width: u16,
    pub height: u16,
    pub offset: Point,
    pub objects: HashMap<Uuid, Rect>,
    pub relationships: HashMap<Uuid, RelationshipUI>,
    pub config: Option<Config>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JSFormat {
    pub paper: EntityFormat<Paper>,
    pub objects: EntityFormat<Object>,
    pub relationships: EntityFormat<Relationship>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EntityFormat<T> {
    pub ids: Vec<Uuid>,
    pub entities: HashMap<Uuid, T>,
}

/// Read a Cuckoo Model file
///
/// That's the file that contains both the model, and the ui elements.
pub trait ReadModel {
    fn from_json(&mut self) -> std::io::Result<JSFormat>;
}

impl<R: std::io::Read> ReadModel for R {
    fn from_json(&mut self) -> std::io::Result<JSFormat> {
        let mut deserializer = serde_json::Deserializer::from_reader(self);

        Ok(JSFormat::deserialize(&mut deserializer)?)
    }
}

pub trait WriteModel {
    fn to_json(&mut self, model: &JSFormat) -> std::io::Result<()>;
}

impl<W: std::io::Write> WriteModel for W {
    fn to_json(&mut self, model: &JSFormat) -> std::io::Result<()> {
        let mut serializer = serde_json::Serializer::new(self);
        model.serialize(&mut serializer)?;
        Ok(())
    }
}
