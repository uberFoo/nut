use std::{collections::HashMap, env, fs::File, path::PathBuf};

use clap::{command, value_parser, Arg};
use quote::quote;
use uuid::Uuid;

// use nut::ooa_0::{
//     object::Object as O0,
//     relationship::{RelGetters, Relationship as R0},
//     ObjectType,
// };
// Eventually I'm going to need to write some From impls between versions.
use nut::{
    ObjectType, Object_v0 as Object, ReadCuckooModel, Relationship_v0 as Relationship,
    Schema_v0 as Schema,
};

// use nut_derive::MyTrait;

// pub trait ObjectWithAttributes {
//     type Item;
//     type IntoIter: Iterator<Item = Self::Item>;

//     fn attributes(&self, attrs: &HashMap<Uuid, Attribute>) -> Self::IntoIter;
// }

// impl ObjectWithAttributes for Object {
//     type Item = Attribute;
//     type IntoIter: std::vec::IntoIter<Self::Item>;

// }

fn main() -> std::io::Result<()> {
    let args = command!()
        .arg(
            Arg::new("schema_file")
                .required(true)
                .help("input file that contains the schema")
                .value_parser(value_parser!(PathBuf)),
        )
        .get_matches();

    // Read the model from the JSON schema
    let path = args.get_one::<PathBuf>("schema_file").unwrap();
    let ooa = File::open(path)?.from_json()?;

    // Unwrap the Vec<ObjectType::Object> into Vec<Object> to make access more pleasant.
    let objects = ooa
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
    let relationships = ooa
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

    // Now we has a model!
    let model = Schema {
        version: "ooa_0".to_owned(),
        objects,
        relationships,
    };

    // Let's write it out for the next stage

    println!(
        r##"use serde::{{Deserialize, Serialize}};
use std::rc::Rc;
use uuid::Uuid;

use nut::ooa_1::UUID_NS;

fn main() {{
    let a = Object {{
        id: Uuid::new_v5(&UUID_NS, "uberFoo".as_bytes()),
        name: "uberFoo".to_owned(),
    }};

    let rc = Rc::new(a);

    let b0 = SubType {{
        id: Uuid::new_v5(&UUID_NS, "SubType_0".as_bytes()),
        obj_id: rc.clone(),
    }};

    let b1 = SubType {{
        id: Uuid::new_v5(&UUID_NS, "SubType_1".as_bytes()),
        obj_id: rc.clone(),
    }};

    let v = vec![b0, b1];

    println!("JSON of generated structs with Rc<T>!:\n{{}}", serde_json::to_string_pretty(&v).unwrap());

    let json = r#"[
  {{
    "id": "9f4cae0a-22ab-5fce-9d8e-282f6e2825b7",
    "obj_id": {{
      "id": "412a01e4-be6c-5415-bd70-366afc923fd2",
      "name": "uberFoo"
    }}
  }},
  {{
    "id": "1898c671-7f68-596e-901d-6bc850f22d20",
    "obj_id": {{
      "id": "412a01e4-be6c-5415-bd70-366afc923fd2",
      "name": "uberFoo"
    }}
  }}
]"#;

    let v1 = serde_json::from_str::<Vec<SubType>>(json).unwrap();

    println!("structs in memory:");
    println!("{{:#?}}", rc);
    println!("{{:#?}}", v1);
}}
"##
    );

    // Now what to do with it?
    model.objects.iter().for_each(|(id, obj)| {
        // TODO:
        // There should be an easy way to access the relationship that you are a part of.
        // TODO:
        // Need a buffer to collect output into. Ideally the buffers are connected later, not
        // necessarily in the same order they were defined. Buffers should hold state as well as
        // generated output. See below for an example.

        // Printing out the object description as comments is slick.
        obj.description.split('\n').for_each(|line| {
            if line.len() > 0 {
                println!("/// {}", line)
            } else {
                println!("///");
            }
        });
        println!("#[derive(Clone, Debug, Deserialize, Serialize)]");
        if model
            .relationships
            .iter()
            .filter(|(_, r)| match r {
                Relationship::Binary(b) => b.from.obj_id == *id,
                _ => false,
            })
            .count()
            > 0
        {
            println!("struct {} {{", obj.name);
            // println!("struct {}<'a> {{", obj.name);
        } else {
            println!("struct {} {{", obj.name);
        }

        for (_id, attr) in &obj.attributes {
            println!("    pub {}: {},", attr.name, attr.attr_t);
        }
        // We don't even have referential attributes attached directly to the
        // objects. We have to do this interesting thing. How was I thinking that
        // Xuder would help me with this? It's like I went crazy. Off the rails.
        // Or maybe I'm just tired, and it'll make sense in the morning.
        model
            .relationships
            .iter()
            .filter(|(_, r)| match r {
                Relationship::Binary(b) => b.from.obj_id == *id,
                _ => false,
            })
            .for_each(|(_id, rel)| match rel {
                Relationship::Binary(b) => {
                    let to_obj = model.objects.get(&b.to.obj_id).unwrap();
                    println!(
                        "    pub {}: Rc<{}>,",
                        b.from.formalizing_attribute_name, to_obj.name
                    )
                }
                _ => (),
            });
        println!("}}\n");
    });

    model.relationships.iter().for_each(|(_id, rel)| {
        let foo = match rel {
            Relationship::Binary(_b) => {
                let id = quote::format_ident!("Rel{}", rel.get_number());
                // What good would this be? Not sure, but it's intriguing.
                quote! {
                    struct #id {}
                }
            }
            _ => quote! {},
        };
        println!("{}", foo);
    });

    Ok(())
}
