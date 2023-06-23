//! Generate Domain Types
//!
//! This is where we generate `struct`s and `enum`s for use _what do we call the
//! domain stuff_?
use std::path::PathBuf;

use heck::{ToSnakeCase, ToTitleCase};

use crate::codegen::{
    begin_crit, emit_enum_main, emit_generated_code_comments, emit_object_comments, emit_singleton,
    emit_struct_main, emitln, end_crit, get_subtypes, Context, Result, SarzakModel,
};
use crate::sarzak::{Attribute, Object, RelPointer, RelSide, AS_CONST, AS_IDENT, AS_TYPE};

pub fn generate_types(
    store: &SarzakModel,
    out_file: &PathBuf,
    package: &str,
    meta: bool,
    doc_tests: bool,
    new_impl: bool,
    extrude_impl: bool,
    ignore_ignore: bool,
) -> Result<()> {
    let domain = &store.domain;

    let mut objects: Vec<&Object> = store
        .sarzak
        .iter_object()
        .filter_map(|(id, obj)| {
            if store.config.is_imported(&id) {
                None
            } else {
                Some(obj)
            }
        })
        .collect();

    // Make everything object related output in a stable order.
    objects.sort_by(|a, b| a.name.cmp(&b.name));

    let mut context = Context::new(out_file, ignore_ignore)?;

    // Generate code for all of the objects in the input_schema
    //
    // Generate the prelude? What's this called? I'm sure it's got a name.
    //
    emitln!(
        context,
        "//! Types for instances of the \"{}\" domain",
        domain.to_title_case()
    );
    emitln!(context, "//! # Domain Description");
    emitln!(context, "//!");
    context += emit_object_comments(&store.description, "//!").into();

    emitln!(context, "//!");
    emitln!(context, "//! # Contents");
    emitln!(context, "//!");
    emitln!(context, "//! The following types are defined herein:");

    // Make some nice documentation
    for obj in &objects {
        if store.config.is_singleton(&obj.id) {
            emitln!(context, "//!    * [`{}`]", obj.render(AS_CONST));
        } else {
            emitln!(context, "//!    * [`{}`]", obj.render(AS_TYPE));
        }
    }
    emitln!(context, "//!");

    // Emit a pointer back to how to generate this file. Having this seemed really
    // useful when the program was invoked by `cargo run --bin ...`. Now it's
    // just the sarzak executable. Still, nice to have a pointer back.
    context += emit_generated_code_comments().into();

    // Take care of all of our use imports
    //
    context.writeln("use serde::{Deserialize, Serialize};");

    //
    // ❗️Only use `uuid::uuid` if there is a singleton.
    //
    if store.config.get_singleton_objects().len() > 0 {
        emitln!(context, "use uuid::{uuid, Uuid};");
    } else {
        emitln!(context, "use uuid::Uuid;");
    }

    emitln!(context, "");

    // Imports
    //
    begin_crit!(context, "imports")?;
    emitln!(
        context,
        "use crate::{}::store::ObjectStore;",
        domain.to_snake_case()
    );
    if extrude_impl {
        emitln!(context, "use nut::codegen::{SarzakObjectStore, Extrude};");
    }
    emitln!(context, "use crate::{}::UUID_NS;", domain.to_snake_case());
    end_crit!(context, "imports")?;
    emitln!(context, "");

    // Imported Objects
    //
    // These are imported from another domain. The are marked up that way in
    // the Object.description.
    if store.config.get_imported_objects().len() > 0 {
        emitln!(context, "// Imported Objects");

        begin_crit!(context, "imported-objects")?;

        let mut imports: Vec<String> = store
            .config
            .get_imported_objects()
            .iter()
            .map(|(id, io)| {
                // We need to deal with built-in types differently than UDTs
                let obj = store.sarzak.exhume_object(&id).unwrap();

                if meta {
                    format!("use crate::{}::types::{};", io.domain, obj.render(AS_TYPE))
                } else {
                    format!(
                        "use {}::{}::{};",
                        io.package,
                        io.domain,
                        obj.render(AS_TYPE)
                    )
                }
            })
            .collect();

        imports.sort();
        for i in imports {
            emitln!(context, "{}", i)
        }

        end_crit!(context, "imported-objects")?;
        emitln!(context, "");
    }

    if extrude_impl {
        begin_crit!(context, "context-extrude_impl")?;
        emitln!(context, "pub(crate) struct Context<'a> {");
        context.increase_indent();
        emitln!(context, "sarzak: &'a SarzakObjectStore,");
        context.decrease_indent();
        emitln!(context, "}");
        end_crit!(context, "context-extrude_impl")?;
        emitln!(context, "");
    }

    // Main code generation loop
    //
    for object in objects.iter() {
        log::debug!("Generating code for {}", object.name);
        // Generate `struct` and `enum` header
        //
        // Generate comments
        context += emit_object_comments(&object.description, "///").into();

        // Generate singleton enums if marked as such. This is a compiler configuration.
        //
        if store.config.is_singleton(&object.id) {
            context += emit_singleton(object).into();
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
                let mut attrs: Vec<&Attribute> = object
                    .attributes
                    .iter()
                    .map(|(_, id)| store.sarzak.exhume_attribute(id).unwrap())
                    .collect();

                // Sort the attributes as they will be output.
                attrs.sort_by(|a, b| a.name.render(AS_IDENT).cmp(&b.name.render(AS_IDENT)));

                context += emit_struct_main(
                    object,
                    &attrs,
                    &store,
                    &domain,
                    package,
                    meta,
                    doc_tests,
                    new_impl,
                    extrude_impl,
                )?
                .into();
            } else {
                // ✋✋✋✋ This next bit is important 🤚🤚🤚🤚
                // Maybe not that important. I was just looking at how the Edge enum is generated.
                // It looks weird. I was convinced that it was a terrible implementation, and now
                // I think it may be optimal. I can't decide right now, so maybe look into it's
                // repr at some point in the future.
                //
                // I'm doing this iteration, but it doesn't make sense. What do you do with an object
                // with two super/sub relationships in Rust? We can work something out, I'm sure.
                for rel in super_rel.iter() {
                    let mut subtypes = get_subtypes!(rel.value, store.sarzak);
                    subtypes.sort();

                    context +=
                        emit_enum_main(object, subtypes, store, domain, package, doc_tests)?.into();
                }
            }
        }
    }

    // context.write("include!(\"drawing_impls.rs\");");
    context.commit()
}

// fn make_new_object(
//     object: &Object,
//     store: &SarzakModel,
//     cache: &mut HashMap<&Object, Cache>,
// ) -> CachingContext {
//     let context = CachingContext::new();

//     let mut s = String::new();
//     for (_, id) in &object.attributes {
//         let attr = store.sarzak.exhume_attribute(id).unwrap();
//         // Here we need to deal with all the different sorts of attributes we may
//         // have. Each of the different types has a constructor, etc. We need to
//         // call the ctor, or pass in something good enough, like a &str, for instance.
//         // I don't have it in me atm.
//     }

//     context
// }
