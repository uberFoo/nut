//! Generate code for model module
//!
//! This is generating the structures that the model module is extruding actual Cuckoo
//! model data into. Eventually some of this will shake out in new abstractions and it
//! won't be such a mess. Nor will it be bespoke like this one is. But this is the
//! first one. First real one, the one I did originally has nothing on this.
//!
//! This is turning into quite the mess. Clearly I don't yet have enough information
//! coming from the model. That's a tool problem. I'm working a way to supplement
//! the model with additional hints to code generation. Currently I'm leaning
//! towards another JSON file that's parsed and used during translation. The idea
//! is that as the tool matures, the supplemental file will become thinner and thinner.
//!
//! Until then, I'm hacking at this file. This file is longer than what it generates,
//! and I'd argue more complicated than the ooa_* stuff in here. But it's not weird
//! that it is larger than it's output. Look at source code and compiled code. Right?
//!
//! I've been generating code as if it's going to run someplace. I actually need
//! to be turning references into UUIDs, because this is an _abstract_ representation.
//! What get's into my head sometimes?
//!
//! Here's what we are doing. Generate structs that own their referents. If a
//! referent is an Object, Binary, or Isa, which exist in sarzak, we want to
//! output that as a UUID.
//!
//! Man, I was wrong again. 😫 Everything is a UUID. So nothing owns anything, it's
//! all pointers, just not memory pointers.
//!
//! Hold on a second! What about primitive types? They need to pass through the sieve,
//! right? It seems like storing a pointer to a primitive value is stupid. Not gonna
//! do it! Interesting aside: we could use this to inter strings if we ever wanted
//! to somehow use this DB representation, outside of code generation.
use std::{env, path::PathBuf};

use clap::{command, value_parser, Arg};

use nut::codegen::{emit, get_referent, get_subtypes, Context, SarzakModel};
use nut::sarzak::*;

fn main() -> std::io::Result<()> {
    let args = command!()
        .arg(
            Arg::new("input_model")
                .required(true)
                .help("drawing domain schema file")
                .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            Arg::new("domain_name")
                .required(true)
                .help("generated domain module name")
                .value_parser(value_parser!(String)),
        )
        .arg(
            Arg::new("module_location")
                .required(true)
                .help("path to module destination, relative to ./src")
                .value_parser(value_parser!(String)),
        )
        .get_matches();

    let model_path = args.get_one::<PathBuf>("input_model").unwrap();
    let store = SarzakModel::load_model(&model_path.to_string_lossy()).unwrap();

    let domain = args.get_one::<String>("domain_name").unwrap();
    let module = args.get_one::<String>("module_location").unwrap();

    let mut module_path = PathBuf::new();
    module_path.push(env::current_dir().unwrap());
    module_path.push("src");
    module_path.push(module);

    // Generate the module file
    let mut module_file = module_path.clone();
    module_file.push(format!("{}.rs", domain));
    let mut context = Context::new(&module_file).unwrap();

    context.write("//! Generated Code -- do not edit");
    context.write("//! Use the following invocation to reproduce. Be careful running when erring.");
    context.write("//! ```ignore");
    emit!(
        context,
        "//! {}",
        env::args().fold(String::new(), |mut s, z| {
            s += " ";
            s += &z;
            s
        })
    );
    context.write("//! ```");
    emit!(context, "mod {};", domain);

    context.commit()?;

    module_path.push(domain);

    // Generate the file that contains the structs and enums
    let mut impl_file = module_path.clone();
    impl_file.push(format!("{}_impls.rs", domain));

    let context = Context::new(&impl_file)?;
    // TODO: We could generate some skeleton code here. At least for doing extrusions.
    //TODO: ⭐️⭐️ Don't let this overwrite an existing file. You have been warned. ⭐️⭐️
    context.commit()?;

    // TODO: Need ObjectStore and macros still.

    // Generate the file that contains the structs and enums
    let mut data_file = module_path;
    data_file.push(format!("{}.rs", domain));

    let mut context = Context::new(&data_file)?;

    // Generate code for all of the objects in the input_schema
    context.write("//! Generated Code -- do not edit");
    context.write("//! Use the following invocation to reproduce. Be careful running when erring.");
    context.write("//! ```ignore");
    emit!(
        context,
        "//! {}",
        env::args().fold(String::new(), |mut s, z| {
            s += " ";
            s += &z;
            s
        })
    );
    context.write("//! ```");
    context.write("use serde::{Deserialize, Serialize};");
    context.write("use uuid::Uuid;\n");
    context.write("use crate::drawing::UUID_NS;");

    store.sarzak.objects().for_each(|object| {
        // Don't generate code for external objects
        if object.name.inner() != "Object"
            && object.name.inner() != "Binary"
            && object.name.inner() != "Isa"
        {
            // Generate comments -- I'd sure like to split these up so that they aren't so long.
            if object.description.len() > 0 {
                object.description.split('\n').for_each(|desc| {
                    context.write(format!("/// {}", desc));
                });
            }

            // The cardinal directions are themselves objects, and therefor need an ID. Here
            // we generate code for them, including a new, which always returns the same
            // instance.
            // I can't currently think of a better way to do this...
            // I actually have, I think.
            if object.name.inner() == "North"
                || object.name.inner() == "East"
                || object.name.inner() == "South"
                || object.name.inner() == "West"
            {
                context.write(format!(
                    r#"
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct {} (pub Uuid);

// We are always returning the same thing, as intended. I just wish there were a
// way to make this a const. Maybe there is -- I haven't really looked around.
impl {} {{
    pub fn new() -> Self {{
        Self(Uuid::new_v5(&UUID_NS, "{}".as_bytes()))
    }}
}}
"#,
                    object.name, object.name, object.name
                ));
            } else {
                // We need to determine if we are writing a struct or an enum. If the object is a
                // supertype then we generate an enum.
                let super_rel = object
                    .rels
                    .iter()
                    .filter_map(|(_, r_ptr)| {
                        if r_ptr.side == RelSide::Supertype {
                            Some(r_ptr)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<&RelPointer>>();

                // We are not a supertype, so just do regular struct stuff.
                if super_rel.len() == 0 {
                    // Generate the struct opening.
                    context.write("#[derive(Clone, Debug, Deserialize, Serialize)]");
                    context.write(format!("pub struct {} {{", object.name));

                    // Generate the struct fields from the objects attributes. These are declared
                    // on the object, so just write them as they are.
                    object.attributes.iter().for_each(|(_, id)| {
                        let attr = store.sarzak.exhume_attribute(id).unwrap();
                        if attr.attr_t == Type::Integer {
                            context.write(format!("    pub {}: {},", attr.name, attr.attr_t));
                        } else {
                            context.write(format!("    /// pub {}: {},", attr.name, attr.attr_t));
                            context.write(format!("    pub {}: Uuid,", attr.name));
                        }
                    });

                    // We are looking for relationships were we are the Referrer. These are either
                    // pointers to external objects, or we own them outright.
                    object.rels.iter().for_each(|(_, r_ptr)| {
                        // We need to look at all of our relationships where we are the referrer.
                        // In that case we'll need to add some attributes.
                        if r_ptr.side == RelSide::Referrer {
                            let rel = store.sarzak.exhume_binary(&r_ptr.value).unwrap();
                            let to_rel = store.sarzak.exhume_referrer(&rel.from).unwrap();
                            let referent = get_referent!(r_ptr, store.sarzak);

                            if referent.name.inner() == "Object"
                                || referent.name.inner() == "Binary"
                                || referent.name.inner() == "Isa"
                            {
                                context.write(format!("    /// sarzak::{}", referent.name));
                                context.write(format!(
                                    "    pub {}: Uuid,",
                                    to_rel.referential_attribute
                                ));
                            } else {
                                if to_rel.cardinality == Cardinality::One {
                                    context.write(format!(
                                        "    /// pub {}: {},",
                                        to_rel.referential_attribute, referent.name
                                    ));
                                    context.write(format!(
                                        "    pub {}: Uuid,",
                                        to_rel.referential_attribute
                                    ));
                                } else {
                                    context.write(format!(
                                        "    /// pub {}: Vec<{}>,",
                                        to_rel.referential_attribute, referent.name
                                    ));
                                    context.write(format!(
                                        "    pub {}: Vec<Uuid>,",
                                        to_rel.referential_attribute
                                    ));
                                }
                            }
                        }
                    });

                    context.write("}\n");
                } else {
                    // I'm doing this iteration, but it doesn't make sense. What do you do with an object
                    // with two super/sub relationships? In Rust?
                    super_rel.iter().for_each(|rel| {
                        // Write out the enum starter.
                        context.write("#[derive(Clone, Debug, Deserialize, Serialize)]");
                        context.write(format!("pub enum {} {{", object.name));

                        let mut subs = Vec::new();

                        // Iterate over the subtypes and add them as variants.
                        get_subtypes!(rel.value, store.sarzak)
                            .iter()
                            .for_each(|sub| {
                                subs.push(&sub.name);

                                if sub.attributes.len() > 0 {
                                    context.write(format!("    /// {}({}),", sub.name, sub.name));
                                    context.write(format!("    {}(Uuid),", sub.name));
                                } else {
                                    context.write(format!("    {},", sub.name));
                                }
                            });
                        context.write("}\n");

                        emit!(context, "impl {} {{", object.name);
                        emit!(context, "    pub fn get_id(&self) -> Uuid {");
                        emit!(context, "        match *self {");
                        subs.iter().for_each(|sub| {
                            emit!(context, "            Self::{}(z) => z,", sub);
                        });
                        emit!(context, "        }");
                        emit!(context, "    }");
                        emit!(context, "}");
                    });
                }
            }
        }
    });

    emit!(context, "include!(\"{}_impls.rs\");", domain);
    context.commit()
}
