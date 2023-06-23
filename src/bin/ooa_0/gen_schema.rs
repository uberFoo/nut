use rand::random;
use std::{collections::HashMap, fs::File, io::prelude::*, path::PathBuf};

use clap::{command, value_parser, Arg};
use serde::{Deserialize, Serialize};
use uuid::{self, Uuid};

use nut::sarzak::{Attribute, Cardinality, Conditionality, Type};
use nut::{
    ObjectType, Object_v0 as Object, Relationship_v0 as Relationship, UUID_NS_v0 as UUID_NS,
};

#[derive(Debug, Deserialize, Serialize)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Deserialize, Serialize)]
struct AnchorPoint {
    id: Uuid,
    dir: Direction,
    card: Cardinality,
    x: i32,
    y: i32,
    offset: Point,
}

impl AnchorPoint {
    fn new(id: &Uuid) -> Self {
        Self {
            id: id.clone(),
            dir: Direction::West,
            card: Cardinality::One,
            x: 300 + random::<i8>() as i32,
            y: 300 + random::<i8>() as i32,
            offset: Point { x: 40, y: 40 },
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Deserialize, Serialize)]
struct Rect {
    x: i32,
    y: i32,
    width: u16,
    height: u16,
}

impl Default for Rect {
    fn default() -> Self {
        Self {
            x: 100 + random::<i8>() as i32,
            y: 100 + random::<i8>() as i32,
            width: 250,
            height: 125,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
enum RelationshipUI {
    BinaryUI(PointTuple),
    IsaUI(Isa),
}

#[derive(Debug, Deserialize, Serialize)]
struct Isa {
    from: AnchorPoint,
    to: Vec<AnchorPoint>,
}

impl Isa {
    fn new(from: &Uuid, to: &Vec<Uuid>) -> Self {
        let to_a = to.iter().map(|i| AnchorPoint::new(i)).collect();
        Self {
            from: AnchorPoint::new(from),
            to: to_a,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct PointTuple {
    from: AnchorPoint,
    to: AnchorPoint,
}

impl PointTuple {
    fn new(from: &Uuid, to: &Uuid) -> Self {
        Self {
            from: AnchorPoint::new(from),
            to: AnchorPoint::new(to),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct Paper {
    id: Uuid,
    domain_name: String,
    domain_ns: String,
    width: u16,
    height: u16,
    offset: Point,
    objects: HashMap<Uuid, Rect>,
    relationships: HashMap<Uuid, RelationshipUI>,
}

impl Default for Paper {
    fn default() -> Self {
        let name = "Paper::sarzak_ooa_0".to_owned();
        let id = Uuid::new_v5(&UUID_NS, name.as_bytes());

        Self {
            id,
            domain_name: name,
            domain_ns: UUID_NS.to_string(),
            width: 3200,
            height: 1600,
            offset: Point { x: 0, y: 0 },
            objects: HashMap::new(),
            relationships: HashMap::new(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct JSFormat {
    paper: EntityFormat<Paper>,
    objects: EntityFormat<Object>,
    relationships: EntityFormat<Relationship>,
}

#[derive(Debug, Deserialize, Serialize)]
struct EntityFormat<T> {
    ids: Vec<Uuid>,
    entities: HashMap<Uuid, T>,
}

impl<T> EntityFormat<T> {
    fn new(map: HashMap<Uuid, T>) -> Self {
        Self {
            ids: map.keys().cloned().collect(),
            entities: map,
        }
    }
}

fn main() -> std::io::Result<()> {
    let args = command!()
        .arg(
            Arg::new("cuckoo")
                .help("output in a format for consumption by Cuckoo")
                .short('c')
                .long("cuckoo")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("out_file")
                .help("output file for the schema")
                .value_parser(value_parser!(PathBuf)),
        )
        .after_help("Output is written to stdout, or the provided file.")
        .get_matches();

    let mut objs = Vec::new();
    // let mut attrs = Vec::new();
    let mut rels = Vec::new();

    let object = Object::new("Object", "O")
        .add_attribute(Attribute::new("id", Type::Uuid))
        .add_attribute(Attribute::new("name", Type::String));
    objs.push(object.clone());

    let attribute = Object::new("Attribute", "A")
        .add_attribute(Attribute::new("id", Type::Uuid))
        .add_attribute(Attribute::new("name", Type::String));
    objs.push(attribute.clone());

    let ty = Object::new("Type", "T").add_attribute(Attribute::new("id", Type::Uuid));
    objs.push(ty.clone());

    let integer = Object::new("Integer", "T_INT").add_attribute(Attribute::new("id", Type::Uuid));
    objs.push(integer.clone());

    let float = Object::new("Float", "T_FLOAT").add_attribute(Attribute::new("id", Type::Uuid));
    objs.push(float.clone());

    let boolean = Object::new("Boolean", "T_BOOL").add_attribute(Attribute::new("id", Type::Uuid));
    objs.push(boolean.clone());

    let string = Object::new("String", "T_STR").add_attribute(Attribute::new("id", Type::Uuid));
    objs.push(string.clone());

    let relationship = Object::new("Relationship", "R")
        .add_attribute(Attribute::new("id", Type::Uuid))
        .add_attribute(Attribute::new("number", Type::Integer));
    objs.push(relationship.clone());

    // // Object <-R1->> Attribute
    let r1 = Relationship::new_binary(
        1,
        attribute.id,
        Cardinality::Many,
        Conditionality::Unconditional,
        "lives in an",
        "obj_id",
        object.id,
        Cardinality::One,
        Conditionality::Unconditional,
        "contains, and is defined by it's",
    );
    rels.push(r1);

    // Attribute <-R2-> Type
    let r2 = Relationship::new_binary(
        2,
        attribute.id,
        Cardinality::One,
        Conditionality::Unconditional,
        "has a",
        "type",
        ty.id,
        Cardinality::One,
        Conditionality::Unconditional,
        "describes the memory representation of",
    );
    rels.push(r2);

    // Type <-- String, Integer, etc.
    let r3 = Relationship::new_isa(3, ty.id, vec![integer.id, float.id, boolean.id, string.id]);
    rels.push(r3);

    let obj_vec: Vec<ObjectType> = objs.iter().map(|x| ObjectType::Object(x.clone())).collect();
    let rel_vec: Vec<ObjectType> = rels
        .iter()
        .map(|x| ObjectType::Relationship(x.clone()))
        .collect();

    let cuckoo = args.get_one::<bool>("cuckoo").unwrap();

    let schema = if *cuckoo {
        // ...and this is what I did for the JS app. I don't know if I want cannon to reflect the needs
        // of JS. OTOH, This is something like how I initially did it. Probably I'll just have an
        // option to select output type.
        let mut obj_hash = HashMap::new();
        let mut rel_hash = HashMap::new();

        objs.iter().cloned().for_each(|o| {
            obj_hash.insert(o.id, o);
        });

        rels.iter().cloned().for_each(|r| {
            rel_hash.insert(r.get_id(), r);
        });

        let mut paper = Paper::default();
        for k in obj_hash.keys() {
            paper.objects.insert(k.clone(), Rect::default());
        }

        for (k, rel) in rel_hash.iter() {
            match rel {
                Relationship::Binary(b) => {
                    let to = b.to.obj_id;
                    let from = b.from.obj_id;
                    paper.relationships.insert(
                        k.clone(),
                        RelationshipUI::BinaryUI(PointTuple::new(&from, &to)),
                    );
                }
                Relationship::Isa(i) => {
                    let from = i.obj_id;
                    paper.relationships.insert(
                        k.clone(),
                        RelationshipUI::IsaUI(Isa::new(&from, &i.subtypes)),
                    );
                }
                _ => (),
            };
        }

        let mut paper_hash = HashMap::new();
        paper_hash.insert(paper.id, paper);

        let js = JSFormat {
            paper: EntityFormat::new(paper_hash),
            objects: EntityFormat::new(obj_hash),
            relationships: EntityFormat::new(rel_hash),
        };
        serde_json::to_string_pretty(&js)
    } else {
        // This output was what I originally came up with...
        let mut instances = HashMap::new();
        instances.insert("Objects".to_owned(), obj_vec);
        instances.insert("Relationships".to_owned(), rel_vec);

        serde_json::to_string_pretty(&instances)
    }?;

    if let Some(path) = args.get_one::<PathBuf>("out_file") {
        let mut file = File::create(path)?;
        file.write_all(schema.as_bytes())?;
    } else {
        println!("{schema}");
    }

    Ok(())
}
