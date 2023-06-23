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
use uuid::Uuid;

use nut::codegen::{emitln, get_referent, get_subtypes, Context, Result, SarzakModel};
use nut::drawing::UUID_NS;
use nut::sarzak::*;

fn main() -> Result<()> {
    let args = command!()
        .arg(
            Arg::new("input_model")
                .required(true)
                .help("drawing domain model file")
                .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            Arg::new("output_file")
                .required(true)
                .help("generated code file")
                .value_parser(value_parser!(PathBuf)),
        )
        .get_matches();

    let model_path = args.get_one::<PathBuf>("input_model").unwrap();
    let store = SarzakModel::load_cuckoo_model(&model_path).unwrap();

    let mut context = Context::new(args.get_one::<PathBuf>("output_file").unwrap(), false)?;

    // Generate code for all of the objects in the input_schema
    context.writeln("//! Generated Code -- do not edit");
    context
        .writeln("//! Use the following invocation to reproduce. Be careful running when erring.");
    context.writeln("//! ```ignore");
    emitln!(
        context,
        "//! {}",
        env::args().fold(String::new(), |mut s, z| {
            s += " ";
            s += &z;
            s
        })
    );
    context.writeln("//! ```");
    context.writeln("use serde::{Deserialize, Serialize};");
    context.writeln("use uuid::{Uuid, uuid};\n");
    context.writeln("use crate::drawing::UUID_NS;");
    emitln!(context, "");

    store.sarzak.objects().for_each(|object| {
        // Don't generate code for external objects
        if object.name.inner() != "Object"
            && object.name.inner() != "Binary"
            && object.name.inner() != "Isa"
        {
            // Generate comments -- I'd sure like to split these up so that they aren't so long.
            if object.description.len() > 0 {
                object.description.split('\n').for_each(|desc| {
                    context.writeln(format!("/// {}", desc));
                });
            }

            // The cardinal directions are themselves objects, and therefor need an ID. Here
            // we generate code for them, including a new, which always returns the same
            // instance.
            // I can't currently think of a better way to do this...
            // I actually have, I think.
            //
            // ⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️
            // Oooo. I don't remember what I was thinking above, but it wasn't this.
            // And it wasn't as cool as this either. I can't believe it took me so
            // long to think of it. And I think I'm good at code generation. Sigh. 😞
            //
            // I'm leaving the original output in there, commented out, to remind
            // myself of this. And it'll be cool to see.
            // I'm going to get warnings about the capitalization of consts. I'm
            // probably just going to turn them off, rather than bother with an
            // ugly SCREAMING variable. I fucking hate those.
            // ⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️
            if object.name.inner() == "Top"
                || object.name.inner() == "Right"
                || object.name.inner() == "Bottom"
                || object.name.inner() == "Left"
            {
                let id = Uuid::new_v5(&UUID_NS, object.name.as_bytes());

                emitln!(context, "#[allow(non_upper_case_globals)]");
                emitln!(
                    context,
                    "// ⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️"
                );
                emitln!(
                    context,
                    "pub const {}: Uuid = uuid![\"{}\"];",
                    object.name,
                    id
                );
                context.writeln(format!(
                    r#"
// #[derive(Clone, Debug, Deserialize, Serialize)]
// pub struct {} (pub Uuid);

// // We are always returning the same thing, as intended. I just wish there were a
// // way to make this a const. Maybe there is -- I haven't really looked around.
// impl {} {{
//     pub fn new() -> Self {{
//         Self(Uuid::new_v5(&UUID_NS, "{}".as_bytes()))
//     }}
// }}
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
                    context.writeln("#[derive(Clone, Debug, Deserialize, Serialize)]");
                    context.writeln(format!("pub struct {} {{", object.name));

                    // Generate the struct fields from the objects attributes. These are declared
                    // on the object, so just write them as they are.
                    object.attributes.iter().for_each(|(_, id)| {
                        let attr = store.sarzak.exhume_attribute(id).unwrap();
                        if attr.attr_t == Type::Integer {
                            context.writeln(format!("    pub {}: {},", attr.name, attr.attr_t));
                        } else {
                            context.writeln(format!("    /// pub {}: {},", attr.name, attr.attr_t));
                            context.writeln(format!("    pub {}: Uuid,", attr.name));
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
                                context.writeln(format!("    /// sarzak::{}", referent.name));
                                context.writeln(format!(
                                    "    pub {}: Uuid,",
                                    to_rel.referential_attribute
                                ));
                            } else {
                                if to_rel.cardinality == Cardinality::One {
                                    context.writeln(format!(
                                        "    /// pub {}: {},",
                                        to_rel.referential_attribute, referent.name
                                    ));
                                    context.writeln(format!(
                                        "    pub {}: Uuid,",
                                        to_rel.referential_attribute
                                    ));
                                } else {
                                    context.writeln(format!(
                                        "    /// pub {}: Vec<{}>,",
                                        to_rel.referential_attribute, referent.name
                                    ));
                                    context.writeln(format!(
                                        "    pub {}: Vec<Uuid>,",
                                        to_rel.referential_attribute
                                    ));
                                }
                            }
                        }
                    });

                    context.writeln("}\n");
                } else {
                    // ✋✋✋✋ This next bit is important 🤚🤚🤚🤚
                    // Maybe not that important. I was just looking at how the Edge enum is generated.
                    // It looks weird. I was convinced that it was a terrible implementation, and now
                    // I think it may be optimal. I can't decide right now, so maybe look into it's
                    // repr at some point in the future.
                    //
                    // I'm doing this iteration, but it doesn't make sense. What do you do with an object
                    // with two super/sub relationships? In Rust?
                    super_rel.iter().for_each(|rel| {
                        // Write out the enum starter.
                        context.writeln("#[derive(Clone, Debug, Deserialize, Serialize)]");
                        context.writeln(format!("pub enum {} {{", object.name));

                        let mut subs = Vec::new();

                        // Iterate over the subtypes and add them as variants.
                        get_subtypes!(rel.value, store.sarzak)
                            .iter()
                            .for_each(|sub| {
                                subs.push(&sub.name);

                                if sub.attributes.len() > 0 {
                                    context.writeln(format!("    /// {}({}),", sub.name, sub.name));
                                    context.writeln(format!("    {}(Uuid),", sub.name));
                                } else {
                                    context.writeln(format!("    {},", sub.name));
                                }
                            });
                        context.writeln("}\n");

                        emitln!(context, "impl {} {{", object.name);
                        emitln!(context, "    pub fn get_id(&self) -> Uuid {");
                        emitln!(context, "        match *self {");
                        subs.iter().for_each(|sub| {
                            emitln!(context, "            Self::{}(z) => z,", sub);
                        });
                        emitln!(context, "        }");
                        emitln!(context, "    }");
                        emitln!(context, "}");
                    });
                }
            }
        }
    });

    context.writeln("include!(\"drawing_impls.rs\");");
    context.commit()
}
