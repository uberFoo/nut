//! "Template" for rendering Types
//!
//! I'm really pushing the template thing. This is really just a submodule.
//! The macro one didn't turn out too bad, but this one has all sorts of logic
//! buried in it.
//!
//! The enum stuff was pretty clean until I needed it for doc tests. Then it got
//! messy too.
//!
//! I need another level of abstraction. Something I'll address in the next code
//! generator. It'll be based off of this generated code, and I'm going to call
//! it ✨grace✨🐶.
use std::{collections::HashSet, env};

use heck::ToSnakeCase;
use log::{debug, trace};
use names::Generator;
use snafu::prelude::*;
use unicode_segmentation::UnicodeSegmentation;
use uuid::Uuid;

use crate::codegen::{
    create_arg_string, emit, emitln, func_name, get_referent,
    sarzak_maybe_get_one_r_sup_across_r14, CachingContext, Field, LoadCuckooModelSnafu, Ref,
    Result, SarzakModel, Symbol,
};

use crate::sarzak::{
    Attribute, Conditionality, Object, RelSide, Relationship, AS_CONST, AS_IDENT, AS_RIDENT,
    AS_TYPE, UUID_NS,
};

const MAX_LEN: usize = 90;

/// Generate struct/enum Documentation
///
/// The text from the tool is really long lines separated by `\n`. We split
/// the lines up on unicode word boundaries and then reconstitute keeping the
/// generated line length less than `MAX_LEN` characters.
///
/// It would be extra sweet to extract the doc links and construct pointers to
/// known types. For example, "points at an [`Object`]", would turn into
/// "points at an [`Object`][o]", and we'd generate an "[o]: nut::sarzak::Object"
/// at the bottom of the comments.
///
/// This is still pretty cool compared to before. The long strings really got
/// to me.
pub fn emit_object_comments(input: &str, comment: &str) -> CachingContext {
    let mut context = CachingContext::new();

    if input.len() > 0 {
        input.split('\n').for_each(|line| {
            emit!(context, "{} ", comment);
            let mut length = 4;

            // Split the string by words, and append a word until we run out
            // of room in the line. Then start another.
            line.split_word_bounds().for_each(|word| match length {
                n if n < MAX_LEN + word.len() => {
                    emit!(context, word);
                    length += word.len();
                }
                _ => {
                    // Trim the trailing space, which I think is guaranteed to
                    // be there, but I'll be cautious anyway. Oh, but I can't
                    // because I don't own the buffer. Shit.

                    // Add a newline
                    emitln!(context, "");
                    length = 0;

                    emit!(context, "{}{}", comment, word);
                    length += word.len() + 3;
                }
            });

            // Add a trailing newline
            emitln!(context, "");
        });

        emitln!(context, "{}", comment);
    }

    context
}

/// Generate a singleton object
///
pub fn emit_singleton(object: &Object) -> CachingContext {
    log::trace!("Generating singleton for {}", object.name);
    let mut context = CachingContext::new();

    // Well now this is tricky. We will use the one from sarzak, I guess.
    let id = Uuid::new_v5(&UUID_NS, object.name.as_bytes());

    emitln!(context, "//");
    emitln!(
        context,
        "pub const {}: Uuid = uuid![\"{}\"];",
        object.render(AS_CONST),
        id
    );
    emitln!(context, "");

    context
}

/// Generate Enum
///
pub fn emit_enum_main(
    object: &Object,
    subtypes: Vec<&Object>,
    store: &SarzakModel,
    domain: &str,
    package: &str,
    doc_tests: bool,
) -> Result<CachingContext> {
    trace!("in `{}` with {}", func_name!(), object.name);

    let mut context = CachingContext::new();

    let mut subtype_variants = Vec::new();

    begin_crit!(context, "{}-enum-definition", object.render(AS_IDENT))?;

    // Write out the enum starter.
    context.writeln("#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]");
    context.writeln(format!("pub enum {} {{", object.render(AS_TYPE)));

    // Iterate over the (sorted) subtypes and add them as variants.
    context.increase_indent();

    subtypes.iter().for_each(|sub| {
        let type_str = sub.render(AS_TYPE);
        subtype_variants.push(type_str.clone());

        if sub.attributes.len() > 0 {
            context.writeln(format!("/// `{}({})`,", type_str, type_str));
            emitln!(context, "///");
            context.writeln(format!("{}(Uuid),", type_str));
        } else {
            context.writeln(format!("{},", type_str));
        }
    });
    context.decrease_indent();

    context.writeln("}");

    end_crit!(context, "{}-enum-definition")?;

    context.writeln("");

    context += emit_enum_main_impl(&object, &subtype_variants)?;

    if doc_tests {
        context += emit_enum_default_impl(&object, &subtypes, store, domain, package)?;
    }

    Ok(context)
}

/// Generate Enum implementations
///
/// This specific implementation is to create the `get_id` method for the enum, which
/// varies with the enum value.
pub fn emit_enum_main_impl(
    object: &Object,
    subtype_variants: &Vec<String>,
) -> Result<CachingContext> {
    let mut context = CachingContext::new();

    begin_crit!(context, "{}-enum-get-id-impl", object.render(AS_TYPE))?;

    emitln!(context, "impl {} {{", object.render(AS_TYPE));

    context.increase_indent();
    emitln!(context, "pub fn get_id(&self) -> Uuid {");

    context.increase_indent();
    emitln!(context, "match *self {");

    context.increase_indent();
    subtype_variants.iter().for_each(|sub| {
        emitln!(context, "Self::{}(z) => z,", sub);
    });
    context.decrease_indent();
    emitln!(context, "}");

    context.decrease_indent();
    emitln!(context, "}");

    context.decrease_indent();
    emitln!(context, "}");

    end_crit!(context, "{}-enum-get-id-impl", object.render(AS_TYPE))?;

    // What's this mean? I wish I knew.
    // Self-closing braces anyone?
    emitln!(context, "");

    Ok(context)
}

/// Generate Default implementation
///
/// This specific implementation is to implement the Default trait
///
/// We're just going to grab the first one in the list. I think it's reasonable to
/// assume that there will eventually be a compiler flag to pick a specific subtype.
///
/// I began this because I needed `default` in the code that generates tests. This
/// provides value, and makes the test generation code simpler. Of course, I should
/// have know I'd just be moving the problem here. 😢
///
/// So, the problem is creating the variant. The first impulse is
/// `Self::Foo{Foo::new(&mut store).id}`. Ok, maybe not the first. This does
/// not work in general because we don't have arguments for the thing.
///
/// So to really do this, and here's why it's not implemented in the macro,
/// we need to call the `new` code. That recursive devil. I have no clue how
/// it's going to be working that in. If I did my job well it'll be easy.
///
/// I'd say I did my job well. 🎉 The hardest part was getting the things I
/// needed passed in as parameters to the function. The code generation bit was
/// simplicity itself.
pub fn emit_enum_default_impl(
    object: &Object,
    subtypes: &Vec<&Object>,
    store: &SarzakModel,
    domain: &str,
    package: &str,
) -> Result<CachingContext> {
    let mut context = CachingContext::new();

    begin_crit!(context, "{}-test_default", object.render(AS_IDENT))?;
    emitln!(context, "impl {} {{", object.render(AS_TYPE));

    context.increase_indent();
    // I'd like to inter store into the symbol table, and then have them figure
    // it out downstream. The problem is that the symbol table doesn't propagate
    // that direction. I really should have figured out how to pass a mutable
    // Context downstream. Next iteration. It must be possible somehow. I could
    // always resort to interior mutability I guess.
    //
    // That leaves now. The easiest thing to do is add a parameter to `emit_render_new`
    // so that if can emit the correct sort of store argument.
    emitln!(
        context,
        "pub fn test_default(store: &mut ObjectStore) -> Self {"
    );
    context.increase_indent();

    let variant = subtypes[0];
    // 🦀 These should get better names. Like, is_supertype(). I imagine they will
    // get added to code generation as time goes on. Not that the fqn needs to go
    // away.
    //
    // This is complicated. Some things have a new, some things have a default,
    // and some things are consts. I need to know type information. I can maybe
    // write a function to extract that?
    //
    // Structs all have new. Enums should have default, when I'm done. The trick
    // below is really about variants. I need the type of the variant.
    //
    // Also, since I wasn't explicit, singletons are enums.
    //
    // Traverse Object -> Supertype across r14, i.e., is variant a supertype.
    if sarzak_maybe_get_one_r_sup_across_r14!(variant, store).is_some() {
        emitln!(
            context,
            "let test = Self::{}({}::test_default(store).get_id());",
            variant.render(AS_TYPE),
            variant.render(AS_TYPE)
        );
    } else {
        if store.config.is_singleton(&variant.id) {
            emitln!(
                context,
                "let test = Self::{}({});",
                variant.render(AS_TYPE),
                variant.render(AS_CONST)
            )
        } else {
            debug!("getting ☯️  in {} for {}", func_name!(), object.name);
            trace!(
                "calling emit_render_new from `{}` with {}",
                func_name!(),
                object.name
            );
            let mut use_statements = HashSet::new();
            let (ctx, args) = emit_render_new(
                variant,
                store,
                domain,
                package,
                &mut use_statements,
                true,
                true,
            )?;

            context.begin_ignore_block()?;
            for us in &use_statements {
                emitln!(context, us);
            }

            context += ctx;

            emitln!(
                context,
                "let test = Self::{}({}::new(store{}).id);",
                variant.render(AS_TYPE),
                variant.render(AS_TYPE),
                create_arg_string(&args, &context)
            );

            context.end_ignore_block()?;
        }
    }

    emitln!(context, "");
    emitln!(
        context,
        "store.inter_{}(test.clone());",
        object.render(AS_IDENT)
    );
    emitln!(context, "");

    emitln!(context, "test");

    context.decrease_indent();
    emitln!(context, "}");

    context.decrease_indent();
    emitln!(context, "}");
    end_crit!(context, "{}-test_default", object.render(AS_IDENT))?;
    // Self-closing braces anyone?
    emitln!(context, "");

    Ok(context)
}

// Generate Struct
//
pub fn emit_struct_main(
    object: &Object,
    attrs: &Vec<&Attribute>,
    store: &SarzakModel,
    domain: &str,
    package: &str,
    _meta: bool,
    doc_tests: bool,
    new_impl: bool,
    extrude_impl: bool,
) -> Result<CachingContext> {
    let mut context = CachingContext::new();

    trace!("in `{}` with {}", func_name!(), object.name);

    begin_crit!(context, "{}-struct-definition", object.render(AS_IDENT))?;
    // Generate the struct "prelude"?
    emitln!(
        context,
        "#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]"
    );
    emitln!(
        context,
        format!(
            "pub struct {} {{",
            //❗️This does weird things like ObjectUI -> ObjectUi. The worst is UUID-> Uuid.
            // That causes problems with Rust. I don't mind this in general -- it's
            // just a side effect of code generation. But we need a way to fix Uuid.
            // I can add a config option, hardcode it, or change the model. None are
            // all that appealing.
            //
            // This is not fixed. I think the problems above are when trying to generate
            // code for the sarzak domain.
            object.render(AS_TYPE)
        )
    );

    // Generate the struct fields from the object's attributes. These are declared
    // on the object, so just write them as they are.
    // Note that we are sorting them so that the output is stable.
    context.increase_indent();

    // These were passed in sorted, but sometimes they generate out of order, but
    // only when generating sarzak and drawing. I tried turning meta off, but that
    // didn't do anything.
    for attr in attrs {
        emitln!(context, "/// pub {}: `{}`,", attr.name, attr.attr_t);
        emitln!(context, "///");
        emitln!(context, "pub {}: {},", attr.render(AS_IDENT), attr.attr_t);
    }
    context.decrease_indent();

    // Generate attributes from relationship that we are formalizing.
    context += emit_struct_rel_attr(object, store, false);
    emitln!(context, "}");
    end_crit!(context, "{}-struct-definition", object.render(AS_IDENT))?;

    emitln!(context, "");

    if new_impl {
        context += emit_struct_impls(object, store, domain, package, doc_tests)?;
    }

    if extrude_impl {
        context += emit_struct_extrude_impl(object, domain)?;
    }

    Ok(context)
}

/// Generate Relationship Attributes
///
/// The for_from thing is just gross. Seriously. This is long overdue for a
/// refactor, but it's at end of life...
fn emit_struct_rel_attr(object: &Object, store: &SarzakModel, for_from: bool) -> CachingContext {
    let mut context = CachingContext::new();

    // We are looking for relationships where we are the Referrer. These are either
    // pointers to external objects, or we own them outright.
    // This short of bullshit is why we autogenerate relationship navigation. Too
    // bad I don't have a relationship in the metamodel...

    // This is all to sort our attributes...
    let mut binary_rels = Vec::new();
    let mut assoc_rels = Vec::new();
    for (_, r) in store.sarzak.iter_relationship() {
        match r {
            Relationship::Binary(b) => {
                let rel = store.sarzak.exhume_binary(b).unwrap();
                let from_rel = store.sarzak.exhume_referrer(&rel.from).unwrap();
                let fuck = store.sarzak.exhume_referent(&rel.to).unwrap();
                let referent = store.sarzak.exhume_object(&fuck.obj_id).unwrap();
                if from_rel.obj_id == object.id {
                    binary_rels.push((
                        from_rel.referential_attribute.render(AS_IDENT).clone(),
                        referent.name.clone(),
                        from_rel.referential_attribute.clone(),
                        referent.id,
                        from_rel.conditionality,
                        referent.render(AS_TYPE).clone(),
                    ));
                }
            }
            Relationship::Associative(a) => {
                let rel = store.sarzak.exhume_associative(a).unwrap();
                let from_ref = store.sarzak.exhume_associative_referrer(&rel.from).unwrap();
                let one_ref = store.sarzak.exhume_associative_referent(&rel.one).unwrap();
                let one_obj = store.sarzak.exhume_object(&one_ref.obj_id).unwrap();
                let other_ref = store
                    .sarzak
                    .exhume_associative_referent(&rel.other)
                    .unwrap();
                let other_obj = store.sarzak.exhume_object(&other_ref.obj_id).unwrap();

                if from_ref.obj_id == object.id {
                    assoc_rels.push((
                        from_ref.one_referential_attribute.render(AS_IDENT).clone(),
                        from_ref.one_referential_attribute.clone(),
                        one_obj.name.clone(),
                    ));
                    assoc_rels.push((
                        from_ref
                            .other_referential_attribute
                            .render(AS_IDENT)
                            .clone(),
                        from_ref.other_referential_attribute.clone(),
                        other_obj.name.clone(),
                    ));
                }
            }
            _ => {}
        }
    }

    binary_rels.sort_by(|a, b| a.0.cmp(&b.0));
    assoc_rels.sort_by(|a, b| a.0.cmp(&b.0));

    context.increase_indent();

    // Emit associative attributes one and other
    for (ident, attr_name, obj_name) in &assoc_rels {
        if !for_from {
            emitln!(context, "/// pub {}: `{}`,", attr_name, obj_name);
            emitln!(context, "///");
            emitln!(context, "pub {}: Uuid,", ident);
        } else {
            emitln!(context, "{}: orig.{},", ident, ident)
        }
    }

    // Emit binary referrer attributes
    for (ident, referent_name, attr_name, id, cond, ty) in &binary_rels {
        if !for_from {
            // If this is an imported object, then indicate so in the comments.
            // We know that it's a pointer, so it's got type `Uuid`.
            match store.config.is_imported(&id) {
                true => {
                    let config = store.config.get_imported(&id).unwrap();
                    emitln!(context, "/// Imported from the {} domain.", config.domain);
                    emitln!(context, "/// [`nut::{}::{}`]", config.domain, ty);
                    emitln!(context, "///");
                    emitln!(context, "pub {}: Uuid,", ident);
                }
                false => match cond {
                    Conditionality::Unconditional => {
                        emitln!(context, "/// pub {}: `{}`,", attr_name, referent_name);
                        emitln!(context, "///");
                        emitln!(context, "pub {}: Uuid,", ident);
                    }
                    Conditionality::Conditional => {
                        emitln!(
                            context,
                            "/// pub {}: `Option<{}>`,",
                            attr_name,
                            referent_name
                        );
                        emitln!(context, "///");
                        emitln!(context, "pub {}: Option<Uuid>,", ident);
                    }
                },
            }
        } else {
            emitln!(context, "{}: orig.{},", ident, ident)
        }
    }
    context.decrease_indent();

    context
}

fn emit_struct_impls(
    object: &Object,
    store: &SarzakModel,
    domain: &str,
    package: &str,
    doc_tests: bool,
) -> Result<CachingContext> {
    let mut context = CachingContext::new();

    let fields = get_object_fields(object, store);

    let mut field_list = String::new();
    let mut id_format = String::new();
    let mut id_list = String::new();

    for f in &fields {
        match f {
            Field::Reference(r) => {
                if r.optional {
                    field_list += &format!(
                        "{}: Option<&{}>, ",
                        r.ref_attr.render(AS_IDENT),
                        f.render(AS_TYPE)
                    );
                    id_format += "{:?}::";
                    id_list += &format!("{}, ", r.ref_attr.render(AS_IDENT));
                } else {
                    field_list +=
                        &format!("{}: &{}, ", r.ref_attr.render(AS_IDENT), f.render(AS_TYPE));
                    id_format += "{:?}::";
                    id_list += &format!("{}, ", r.ref_attr.render(AS_IDENT));
                }
            }
            Field::Attribute(a) => {
                field_list += &format!("{}: {}, ", f.render(AS_IDENT), a.attr_t);
                id_format += "{}::";
                id_list += &format!("{}, ", f.render(AS_IDENT));
            }
        }
    }

    begin_crit!(context, "{}-new_impl", object.render(AS_IDENT))?;

    emitln!(context, "impl {} {{", object.render(AS_TYPE));
    context.increase_indent();

    emitln!(
        context,
        "/// Inter a new {} and return it's `id`",
        object.render(AS_TYPE)
    );
    emitln!(context, "///");

    if doc_tests {
        context.begin_ignore_block()?;
        context += emit_struct_doc_tests(object, store, domain, package)?;
        context.end_ignore_block()?;
    }

    // Emit the function header
    emitln!(
        context,
        "pub fn new(store: &mut ObjectStore, {}) -> Self {{",
        field_list
    );

    context.increase_indent();
    emitln!(
        context,
        "let id = Uuid::new_v5(&UUID_NS, format!(\"{}\", {}).as_bytes());",
        id_format,
        id_list
    );
    emitln!(context, "let new = Self {");

    // Render the field list
    context.increase_indent();
    emitln!(context, "id,");
    for f in &fields {
        match f {
            Field::Attribute(a) => {
                emitln!(context, "{},", a.render(AS_IDENT));
            }
            Field::Reference(r) => {
                // Simply taking an ".id" on the end of the reference is not going
                // to suffice. If the referent is an enum, then we need to issue
                // a ".get_id()" instead.
                //
                // Now we have a problem with imported objects. How do I tell that
                // in imported object is a supertype?
                let id = if store.config.is_imported(&r.referent.id) {
                    let config = store.config.get_imported(&r.referent.id).unwrap();
                    debug!(
                        "❗️Imported object `{}` supertype lookup in {} domain.",
                        r.referent.name, config.domain
                    );
                    // Now that the fun is over with, we need to load the model.
                    let mut path = env::current_dir().unwrap();
                    path.push("models");
                    path.push(&config.model_path);
                    path.push("fubared");
                    path.set_file_name(&config.domain);
                    path.set_extension("json");

                    // 🦀 This should be cached or something.
                    let io_store = SarzakModel::load_cuckoo_model(&path)
                        .context(LoadCuckooModelSnafu { path: &path })?;
                    //         ✨                    ✨         ✨
                    // How the fuck does this work? Oh -- the UUID is based off it's name,
                    // so as long as the name is correct, we'll be able to look it up by
                    // the id it has in our domain.
                    //
                    if sarzak_maybe_get_one_r_sup_across_r14!(&r.referent, io_store).is_some() {
                        trace!("{} is a supertype", r.referent.name);
                        "get_id()"
                    } else {
                        "id"
                    }
                } else if sarzak_maybe_get_one_r_sup_across_r14!(r.referent, store).is_some() {
                    trace!("{} is a supertype", r.referent.name);
                    "get_id()"
                } else {
                    "id"
                };

                if r.optional {
                    emitln!(
                        context,
                        "{}: {}.map(|o| o.{}),",
                        r.ref_attr.render(AS_IDENT),
                        r.ref_attr.render(AS_IDENT),
                        id
                    );
                } else {
                    emitln!(
                        context,
                        "{}: {}.{},",
                        r.ref_attr.render(AS_IDENT),
                        r.ref_attr.render(AS_IDENT),
                        id
                    );
                }
            }
        }
    }

    context.decrease_indent();
    emitln!(context, "};");

    emitln!(context, "");
    // ❗️ I should make a little function that builds these invocations for me.
    emitln!(
        context,
        "store.inter_{}(new.clone());",
        object.render(AS_IDENT)
    );
    emitln!(context, "");
    emitln!(context, "new");

    context.decrease_indent();
    emitln!(context, "}");

    end_crit!(context, "{}-new_impl", object.render(AS_IDENT))?;
    context.decrease_indent();
    emitln!(context, "}");

    emitln!(context, "");

    Ok(context)
}

fn emit_struct_extrude_impl(object: &Object, domain: &str) -> Result<CachingContext> {
    let mut context = CachingContext::new();

    begin_crit!(context, "{}-extrude_impl", object.name.render(AS_IDENT))?;

    emitln!(
        context,
        "impl Extrude<nut::{}::{}, Context<'_>> for {} {{",
        domain,
        object.name.render(AS_TYPE),
        object.name.render(AS_TYPE)
    );
    context.increase_indent();

    emitln!(
        context,
        "fn extrude(orig: nut::{}::{}, context: &mut Context<'_>) -> Self {{",
        domain,
        object.name.render(AS_TYPE)
    );
    context.increase_indent();

    emitln!(context, "let Context { from, ref mut to} = context;");
    emitln!(context, "");
    emitln!(context, "Self::default()");

    context.decrease_indent();
    emitln!(context, "}");

    context.decrease_indent();
    emitln!(context, "}");
    end_crit!(context, "{}-extrude_impl", object.name.render(AS_IDENT))?;

    emitln!(context, "");

    Ok(context)
}

/// Generate Doc Tests
///
/// Oey vey!
fn emit_struct_doc_tests(
    object: &Object,
    store: &SarzakModel,
    domain: &str,
    package: &str,
) -> Result<CachingContext> {
    let mut context = CachingContext::new();

    emitln!(context, "/// # Example");
    emitln!(context, "///");
    emitln!(context, "///```");

    // Render the doc test
    trace!(
        "calling emit_render_new from `{}` with {}",
        func_name!(),
        object.name
    );
    let mut use_statements = HashSet::new();
    let (mut ctx, args) = emit_render_new(
        object,
        store,
        domain,
        package,
        &mut use_statements,
        false,
        false,
    )?;

    for us in &use_statements {
        emitln!(context, "/// # {}", us);
    }

    emitln!(
        context,
        "/// # let mut store = {}::{}::ObjectStore::new();",
        package,
        domain.to_snake_case()
    );
    emitln!(context, "///");
    ctx.insert_prefix("/// ");
    context += ctx;

    emitln!(context, "///");
    emitln!(
        context,
        "/// let {} = {}::new(&mut store{});",
        object.render(AS_IDENT),
        object.render(AS_TYPE),
        create_arg_string(&args, &context)
    );
    emitln!(context, "///```");

    Ok(context)
}

/// Render a `new` invocation
///
/// Render all of the statements necessary to invoke object::new(). This function
/// is recursive ☯️.
///
/// I wonder how hard it would be to make this work for any function invocation?
/// As it is we are brushing up against the problem of knowing about function
/// arguments. Dodged a bullet with `Object::default`.
pub(crate) fn emit_render_new(
    object: &Object,
    store: &SarzakModel,
    domain: &str,
    package: &str,
    use_stmts: &mut HashSet<String>,
    internal: bool,
    store_is_ref: bool,
) -> Result<(CachingContext, Vec<Uuid>)> {
    trace!("in `{}` with {}", func_name!(), object.name);

    let mut context = CachingContext::new();

    if !internal && !store.config.is_imported(&object.id) {
        trace!(
            "emitting use statement for domain: {}::{}::{}",
            package,
            domain.to_snake_case(),
            object.render(AS_TYPE)
        );

        use_stmts.insert(format!(
            "use {}::{}::{};",
            package,
            domain.to_snake_case(),
            object.render(AS_TYPE)
        ));
    }

    // OMG, what are we doing here? I think we are resolving our fields into
    // rvalues. Yeah, that's it.
    // The fact that we aren't actually calling new will throw you. But then
    // you'll remember that you changed it so that it' emitted at the call site.
    let fields = get_object_fields(object, store);
    let mut args = Vec::new();
    for f in &fields {
        match f {
            Field::Attribute(a) => {
                // Had to make everything a String to get the generated strings to
                // live long enough. Rust is smart.
                let arg = match a.attr_t.to_string().as_str() {
                    "bool" => "true".to_owned(),
                    "f64" => "42.0".to_owned(),
                    "i64" => "42".to_owned(),
                    "std::string::String" => {
                        let ident = Generator::default().next().unwrap().to_snake_case();
                        emitln!(
                            context,
                            "let {} = \"{}\".to_owned();",
                            ident,
                            Generator::default().next().unwrap().to_snake_case()
                        );
                        ident
                    }
                    "Uuid" => {
                        let ident = Generator::default().next().unwrap().to_snake_case();
                        emitln!(context, "let {} = Uuid::default();", ident);
                        ident
                    }
                    wtf => unimplemented!(
                        "I don't know about the `{}` type. How'd that get in here? 🤔",
                        wtf
                    ),
                };

                let id = Uuid::new_v4();
                context.inter_symbol(
                    id,
                    Symbol {
                        value: arg,
                        value_type: a.attr_t.to_string(),
                        is_reference: false,
                    },
                );
                args.push(id);
            }
            Field::Reference(r) => {
                if r.optional {
                    if r.referent.id == object.id {
                        trace!("avoiding stack overflow!");
                        let id = Uuid::new_v4();
                        context.inter_symbol(
                            id,
                            Symbol {
                                value: "None".to_string(),
                                value_type: "Option::None".to_string(),
                                is_reference: false,
                            },
                        );
                        args.push(id);
                    } else {
                        trace!(
                            "calling emit_render_new from `{}`-✨ with {}",
                            func_name!(),
                            object.name
                        );
                        let (ctx, new_args) = emit_render_new(
                            r.referent,
                            store,
                            domain,
                            package,
                            use_stmts,
                            internal,
                            store_is_ref,
                        )?;
                        context += ctx;

                        if store_is_ref {
                            emitln!(
                                context,
                                "let {} = {}::new(store{});",
                                r.referent.render(AS_IDENT),
                                r.referent.render(AS_TYPE),
                                create_arg_string(&new_args, &context)
                            );
                        } else {
                            emitln!(
                                context,
                                "let {} = {}::new(&mut store{});",
                                r.referent.render(AS_IDENT),
                                r.referent.render(AS_TYPE),
                                create_arg_string(&new_args, &context)
                            );
                        }

                        let some = format!("Some(&{})", r.referent.render(AS_IDENT));
                        let some_type = format!("Option::Some(&{}))", r.referent.render(AS_TYPE));
                        let id = Uuid::new_v4();
                        context.inter_symbol(
                            id,
                            Symbol {
                                value: some,
                                value_type: some_type,
                                is_reference: false,
                            },
                        );
                        args.push(id);
                    }
                } else if sarzak_maybe_get_one_r_sup_across_r14!(r.referent, store).is_some() {
                    // It's a supertype, emit default
                    //
                    if !internal {
                        trace!(
                            "emitting use statement for domain: {}::{}::{}",
                            package,
                            domain.to_snake_case(),
                            r.referent.render(AS_TYPE)
                        );

                        use_stmts.insert(format!(
                            "use {}::{}::{};",
                            package,
                            domain.to_snake_case(),
                            r.referent.render(AS_TYPE)
                        ));
                    }

                    let lhs = r.referent.render(AS_RIDENT);
                    if store_is_ref {
                        emitln!(
                            context,
                            "let {} = {}::test_default(store);",
                            lhs,
                            r.referent.render(AS_TYPE)
                        );
                    } else {
                        emitln!(
                            context,
                            "let {} = {}::test_default(&mut store);",
                            lhs,
                            r.referent.render(AS_TYPE)
                        );
                    }

                    let id = Uuid::new_v4();
                    context.inter_symbol(
                        id,
                        Symbol {
                            value: lhs,
                            value_type: r.referent.render(AS_TYPE),
                            is_reference: true,
                        },
                    );
                    args.push(id);
                } else if store.config.is_singleton(&r.referent.id) {
                    // It's a singleton
                    //

                    let id = Uuid::new_v4();
                    context.inter_symbol(
                        id,
                        Symbol {
                            value: r.referent.render(AS_CONST),
                            value_type: r.referent.render(AS_TYPE),
                            is_reference: false,
                        },
                    );
                    args.push(id);
                } else {
                    if r.referent.id == object.id {
                        trace!("avoiding stack overflow!");
                        let id = Uuid::new_v4();
                        context.inter_symbol(
                            id,
                            Symbol {
                                value: "None".to_string(),
                                value_type: "Option::None".to_string(),
                                is_reference: false,
                            },
                        );
                        args.push(id);
                    } else {
                        // ☯️ It's a reference, so it's about to get recursive...
                        //
                        trace!(
                            "calling emit_render_new from `{}`-⭐️ with {}",
                            func_name!(),
                            object.name
                        );
                        let (ctx, new_args) = emit_render_new(
                            r.referent,
                            store,
                            domain,
                            package,
                            use_stmts,
                            internal,
                            store_is_ref,
                        )?;
                        context += ctx;

                        let (lhs_name, lhs_type) = if store.config.is_imported(&r.referent.id) {
                            // Deal with using an imported object.
                            //
                            // It's imported, which means we need to test for it's type, just
                            // like we are doing in the surrounding scope. If I weren't so
                            // close to being done I'd think about refactoring. Something to
                            // do in the future.
                            let config = store.config.get_imported(&r.referent.id).unwrap();

                            if !internal {
                                trace!(
                                    "emitting use statement for imported domain: {}::{}::{}",
                                    config.package,
                                    config.domain,
                                    r.referent.render(AS_TYPE)
                                );
                                use_stmts.insert(format!(
                                    "use {}::{}::{};",
                                    config.package,
                                    config.domain,
                                    r.referent.render(AS_TYPE)
                                ));
                            }

                            let lhs = r.referent.render(AS_RIDENT);
                            let lhs_type = r.referent.render(AS_TYPE);

                            // WTF am I blabbering about here?
                            //
                            // Calling default is sort of not cool. It doesn't get the other
                            // side's store involved. Really I should be calling new, with a
                            // different store. But this is only supposed to be for a test.
                            // Except that I'm also using it to generate the default impl...
                            // Maybe I should quit while I'm ahead.
                            //
                            // This did indeed get more complicated. While I'd hoped that using
                            // `default` would get me out of having to do anything fancy here,
                            // it turns out I'm mistaken. The problem is enums, and they don't
                            // have a default method.
                            //
                            // I was going to go in here and do fancy shit to figure out a
                            // random variant and use it. But the more generally useful
                            // thing to do is generate a default implementation for the
                            // enums. I'll leave this note here for now, just to remind
                            // myself that I made a good decision.⭐️
                            //
                            // Ugh. I think that this would have been easier.
                            //
                            let config = store.config.get_imported(&r.referent.id).unwrap();
                            debug!(
                                "❗️{}: imported object `{}` supertype lookup in {} domain.",
                                func_name!(),
                                r.referent.name,
                                config.domain
                            );
                            // Now that the fun is over with, we need to load the model.
                            let mut path = env::current_dir().unwrap();
                            path.push("models");
                            path.push(&config.model_path);
                            path.push("fubared");
                            path.set_file_name(&config.domain);
                            path.set_extension("json");

                            // 🦀 This should be cached or something.
                            let io_store = SarzakModel::load_cuckoo_model(&path)
                                .context(LoadCuckooModelSnafu { path: &path })?;

                            // debug!("made it past");

                            if sarzak_maybe_get_one_r_sup_across_r14!(&r.referent, io_store)
                                .is_some()
                            {
                                if store_is_ref {
                                    emitln!(
                                        context,
                                        "let {} = {}::test_default(store);",
                                        lhs,
                                        r.referent.render(AS_TYPE)
                                    );
                                } else {
                                    emitln!(
                                        context,
                                        "let {} = {}::test_default(&mut store);",
                                        lhs,
                                        r.referent.render(AS_TYPE)
                                    );
                                }

                                emitln!(context, "");
                            } else {
                                // This won't get interred into the store. Another bug to fix...
                                emitln!(
                                    context,
                                    "let {} = {}::default();",
                                    lhs,
                                    r.referent.render(AS_TYPE)
                                );
                                emitln!(context, "");
                            }

                            (lhs, lhs_type)
                        } else {
                            // Not an imported object.

                            if !internal {
                                trace!(
                                    "emitting use statement for domain: {}::{}::{}",
                                    package,
                                    domain.to_snake_case(),
                                    r.referent.render(AS_TYPE)
                                );

                                use_stmts.insert(format!(
                                    "use {}::{}::{};",
                                    package,
                                    domain.to_snake_case(),
                                    r.referent.render(AS_TYPE)
                                ));
                            }

                            let lhs = r.referent.render(AS_RIDENT);
                            let lhs_type = r.referent.render(AS_TYPE);

                            if store_is_ref {
                                emitln!(
                                    context,
                                    "let {} = {}::new(store{});",
                                    lhs,
                                    r.referent.render(AS_TYPE),
                                    create_arg_string(&new_args, &context)
                                );
                            } else {
                                emitln!(
                                    context,
                                    "let {} = {}::new(&mut store{});",
                                    lhs,
                                    r.referent.render(AS_TYPE),
                                    create_arg_string(&new_args, &context)
                                );
                            }

                            (lhs, lhs_type)
                        };

                        let id = Uuid::new_v4();
                        context.inter_symbol(
                            id,
                            Symbol {
                                value: lhs_name,
                                value_type: lhs_type,
                                is_reference: true,
                            },
                        );
                        args.push(id);
                    }
                }
            }
        }
    }

    Ok((context, args))
}

fn get_object_fields<'a>(object: &Object, store: &'a SarzakModel) -> Vec<Field<'a>> {
    // We need to sort out our parameters. There should be one per attribute, except for *id, which
    // we will generate, based on ... on what? I was going to say object name, but then the *ids
    // would all be the same. Normally I'd base it on other attributes, but I'm staring at objects
    // with no other attributes. Time, maybe? Yeah, maybe. v1 and v2 are both time based. Maybe I
    // back off the idea of using v5 uuid's. Ok, so id's are autogenerated v1 uuids. Moving on.
    //
    // Coming back to this conversation, we put a restriction on sarzak v1 models that unless you
    // want a singleton, you need to have attributes that are sufficient to distinguish your set
    // of instances. I just don't want to lose the benefits of having unique instances.
    //
    // We need one parameter per attribute. For any referential attributes, we will need to
    // be passed a reference to the object. Se, let's get building?
    let mut params = Vec::new();
    for (_, id) in &object.attributes {
        let attr = store.sarzak.exhume_attribute(id).unwrap();
        if attr.name.inner() != "id" {
            params.push(Field::Attribute(&attr));
        }
    }

    for (_, r_ptr) in &object.rels {
        match r_ptr.side {
            RelSide::Referrer => {
                let rel = store.sarzak.exhume_binary(&r_ptr.value).unwrap();
                let to_rel = store.sarzak.exhume_referrer(&rel.from).unwrap();
                let referent = get_referent!(r_ptr, store.sarzak);

                match to_rel.conditionality {
                    Conditionality::Unconditional => {
                        let field = Field::Reference(Ref {
                            referent: &referent,
                            ref_attr: &to_rel.referential_attribute,
                            optional: false,
                        });
                        trace!("added field {:?}", &field);
                        params.push(field);
                    }
                    Conditionality::Conditional => {
                        let field = Field::Reference(Ref {
                            referent: &referent,
                            ref_attr: &to_rel.referential_attribute,
                            optional: true,
                        });
                        trace!("added field {:?}", &field);
                        params.push(field);
                    }
                }
            }
            RelSide::AssocFrom => {
                let rel = store.sarzak.exhume_associative(&r_ptr.value).unwrap();
                let from_ref = store.sarzak.exhume_associative_referrer(&rel.from).unwrap();
                let one_ref = store.sarzak.exhume_associative_referent(&rel.one).unwrap();
                let one_obj = store.sarzak.exhume_object(&one_ref.obj_id).unwrap();
                let other_ref = store
                    .sarzak
                    .exhume_associative_referent(&rel.other)
                    .unwrap();
                let other_obj = store.sarzak.exhume_object(&other_ref.obj_id).unwrap();

                let field = Field::Reference(Ref {
                    referent: &one_obj,
                    ref_attr: &from_ref.one_referential_attribute,
                    optional: false,
                });
                trace!("added field {:?}", &field);
                params.push(field);

                let field = Field::Reference(Ref {
                    referent: &other_obj,
                    ref_attr: &from_ref.other_referential_attribute,
                    optional: false,
                });
                trace!("added field {:?}", &field);
                params.push(field);
            }
            _ => {}
        }
    }

    // We require stability in parameter ordering.
    params.sort();

    params
}
