//! Generate macros for drawing domain
//!
//! These are the macros that provide relationship navigation
use std::path::PathBuf;

use heck::{ToSnakeCase, ToTitleCase};

use crate::codegen::{
    emit_assoc_many, emit_assoc_maybe_get_one, emit_associative_main, emit_binary_main,
    emit_generated_code_comments, emit_many_conditional_lookup, emit_many_unconditional,
    emit_one_conditional, emit_one_conditional_lookup, emit_one_unconditional,
    emit_one_unconditional_lookup, emitln, function, Context, Result, SarzakModel,
};
use crate::sarzak::{Associative, Binary, Cardinality, Conditionality, Relationship};

pub fn generate_macros(
    model: &SarzakModel,
    output_file: &PathBuf,
    package: &str,
    _meta: bool,
    doc_tests: bool,
    ignore_ignore: bool,
) -> Result<()> {
    let domain = &model.domain;

    let mut context = Context::new(output_file, ignore_ignore)?;

    // Generate code for all of the objects in the input_schema
    emitln!(
        context,
        "//! Macros for navigating the \"{}\" domain",
        domain.to_title_case()
    );
    emitln!(context, "//!");

    // Emit a pointer back to how to generate this file.
    context += emit_generated_code_comments().into();

    // We'd really like a stable sort. So much for iter_relationship...
    // And there must be a better way to do this. I tried getting tricky with a
    // closure but I never got it working. See ooa2::ObjectStore.
    let mut relationships: Vec<&Relationship> = model.sarzak.relationships().collect();
    relationships.sort();

    for rel in relationships.iter() {
        match *rel {
            Relationship::Binary(ref b) => {
                let binary = model.sarzak.exhume_binary(b).unwrap();
                generate_binary_macros(binary, &domain, package, &model, &mut context, doc_tests)?;
            }
            // Isa Relationship Traversal makes no sense
            Relationship::Isa(_) => {}
            Relationship::Associative(a) => {
                let assoc = model.sarzak.exhume_associative(a).unwrap();
                generate_associative_macros(
                    assoc,
                    &domain,
                    &model,
                    package,
                    &mut context,
                    doc_tests,
                )?;
            }
        }
    }

    context.commit()
}

fn generate_associative_macros(
    ass: &Associative,
    domain: &str,
    model: &SarzakModel,
    package: &str,
    context: &mut Context,
    doc_tests: bool,
) -> Result<()> {
    let ass_ref = model.sarzak.exhume_associative_referrer(&ass.from).unwrap();
    let ass_obj = model.sarzak.exhume_object(&ass_ref.obj_id).unwrap();
    let one_ref = model.sarzak.exhume_associative_referent(&ass.one).unwrap();
    let one_obj = model.sarzak.exhume_object(&one_ref.obj_id).unwrap();
    let other_ref = model
        .sarzak
        .exhume_associative_referent(&ass.other)
        .unwrap();
    let other_obj = model.sarzak.exhume_object(&other_ref.obj_id).unwrap();

    // This takes care of one direction
    let (inner, name) = match one_ref.cardinality {
        Cardinality::One => {
            let inner = emit_assoc_maybe_get_one(
                &one_obj,
                &ass_ref.one_referential_attribute,
                &ass_ref.other_referential_attribute,
                &ass_obj,
            );
            let name = format!(
                "{}_maybe_get_one_{}_across_r{}",
                domain,
                one_obj.key_letter.to_snake_case(),
                ass.number
            );
            (inner, name)
        }
        Cardinality::Many => {
            let inner = emit_assoc_many(
                &one_obj,
                &ass_ref.one_referential_attribute,
                &ass_ref.other_referential_attribute,
                &ass_obj,
            );
            let name = format!(
                "{}_get_many_{}_across_r{}",
                domain,
                one_obj.key_letter.to_snake_case(),
                ass.number
            );
            (inner, name)
        }
    };

    let outer = emit_associative_main(
        domain,
        &name,
        &other_obj,
        &one_obj,
        &ass_obj,
        ass.number,
        function!(),
        inner,
        model,
        package,
        doc_tests,
        one_ref.conditionality == Conditionality::Conditional,
        one_ref.cardinality == Cardinality::Many,
    )?;

    *context += outer.into();

    // This takes care of the other direction.
    let (inner, name) = match other_ref.cardinality {
        Cardinality::One => {
            let inner = emit_assoc_maybe_get_one(
                &other_obj,
                &ass_ref.other_referential_attribute,
                &ass_ref.one_referential_attribute,
                &ass_obj,
            );
            let name = format!(
                "{}_maybe_get_one_{}_across_r{}",
                domain,
                other_obj.key_letter.to_snake_case(),
                ass.number
            );
            (inner, name)
        }
        Cardinality::Many => {
            let inner = emit_assoc_many(
                &other_obj,
                &ass_ref.other_referential_attribute,
                &ass_ref.one_referential_attribute,
                &ass_obj,
            );
            let name = format!(
                "{}_get_many_{}_across_r{}",
                domain,
                other_obj.key_letter.to_snake_case(),
                ass.number
            );
            (inner, name)
        }
    };

    let outer = emit_associative_main(
        domain,
        &name,
        &one_obj,
        &other_obj,
        &ass_obj,
        ass.number,
        function!(),
        inner,
        model,
        package,
        doc_tests,
        other_ref.conditionality == Conditionality::Conditional,
        other_ref.cardinality == Cardinality::Many,
    )?;

    *context += outer.into();

    Ok(())
}

fn generate_binary_macros(
    binary: &Binary,
    domain: &str,
    package: &str,
    model: &SarzakModel,
    context: &mut Context,
    doc_tests: bool,
) -> Result<()> {
    generate_binary_macro_referrer_to_referent_imp(
        &binary, domain, package, &model, context, doc_tests,
    )?;
    generate_binary_macro_referent_to_referrer_imp(
        &binary, domain, package, &model, context, doc_tests,
    )?;

    Ok(())
}

/// Generate Macro for Binary Referrer to Referent
///
/// These are traversals that go from the side holding the referential attribute
/// to the side without the referential attribute. Therefore these traversals
/// have a pointer to the other side.
fn generate_binary_macro_referrer_to_referent_imp(
    binary: &Binary,
    domain: &str,
    package: &str,
    model: &SarzakModel,
    context: &mut Context,
    doc_tests: bool,
) -> Result<()> {
    let from = model.sarzak.exhume_referrer(&binary.from).unwrap();
    let referrer = model.sarzak.exhume_object(&from.obj_id).unwrap();
    let to = model.sarzak.exhume_referent(&binary.to).unwrap();
    let referent = model.sarzak.exhume_object(&to.obj_id).unwrap();

    // Only one source of objects allowed per domain, so we don't follow.
    if model.config.is_imported(&referent.id) {
        return Ok(());
    }

    #[derive(Debug)]
    struct Cond<'a> {
        name: &'a str,
        desc: &'a str,
    }

    // We are traversing the relationship in the referrer -> referent direction.
    let cond = if from.conditionality == Conditionality::Conditional {
        Cond {
            desc: "(c)",
            name: "maybe_",
        }
    } else {
        Cond { desc: "", name: "" }
    };

    #[derive(Debug)]
    struct Card<'a> {
        name: &'a str,
    }

    // We are traversing the relationship in the referrer -> referent direction.
    // The cardinality of this traversal is on the referent (to) side.
    let card = if from.cardinality == Cardinality::One {
        Card { name: "one" }
    } else {
        Card { name: "many" }
    };

    let plural = if card.name == "many" { "s" } else { "" };
    let macro_name = format!(
        "{}_{}get_{}_{}{}_across_r{}",
        domain,
        cond.name,
        card.name,
        referent.key_letter.to_snake_case(),
        plural,
        binary.number
    );

    // 👻 This is correct for card == many, cond == unconditional
    // Maybe correct. Definitely not tested to my liking.
    // Abusing {cond|card}.name like this is embarrassing
    let inner = if card.name == "many" && cond.name == "" {
        // This is never called.
        emit_many_unconditional(&referent, &from.referential_attribute)
    }
    // 👻 This one should work for card == one, cond == conditional
    else if card.name == "one" && cond.name == "maybe_" {
        emit_one_conditional(&referent, &from.referential_attribute)
    }
    // 👻 This is card == one, cond == unconditional
    else if card.name == "one" && cond.name == "" {
        emit_one_unconditional(&referent, &from.referential_attribute)
    } else if card.name == "many" && cond.name == "maybe_" {
        emit_many_unconditional(&referent, &from.referential_attribute)
    } else {
        unimplemented!("{} card: {:?} cond: {:?}", function!(), card, cond)
    };

    let body = emit_binary_main(
        &domain,
        &macro_name,
        &referrer,
        &referent,
        binary.number,
        cond.desc == "(c)",
        function!(),
        inner,
        &model,
        package,
        doc_tests,
        false,
        false,
    )?;

    *context += body.into();

    Ok(())
}

/// Ugh.
///
/// The `Cond` and `Card` stuff is so ugly. It made sense when I was just trying
/// to figure out what the fuck was going on. But now, I think I can reason about
/// things, and make a better implementation.
fn generate_binary_macro_referent_to_referrer_imp(
    binary: &Binary,
    domain: &str,
    package: &str,
    model: &SarzakModel,
    context: &mut Context,
    doc_tests: bool,
) -> Result<()> {
    let from = model.sarzak.exhume_referrer(&binary.from).unwrap();
    let referrer = model.sarzak.exhume_object(&from.obj_id).unwrap();
    let to = model.sarzak.exhume_referent(&binary.to).unwrap();
    let referent = model.sarzak.exhume_object(&to.obj_id).unwrap();

    // Only one source of objects allowed per domain, so we don't follow.
    // Don't follow imported objects when generating macros.
    if model.config.is_imported(&referent.id) {
        return Ok(());
    }

    #[derive(Debug)]
    struct Cond<'a> {
        name: &'a str,
        desc: &'a str,
    }

    // We are traversing the relationship in the referent -> referrer direction.
    let cond = if to.conditionality == Conditionality::Conditional {
        Cond {
            desc: "(c)",
            name: "maybe_",
        }
    } else {
        Cond { desc: "", name: "" }
    };

    #[derive(Debug)]
    struct Card<'a> {
        name: &'a str,
    }

    // We are traversing the relationship in the referent -> referrer direction.
    // The cardinality of this traversal is on the referrer (from) side.
    let card = if to.cardinality == Cardinality::One {
        Card { name: "one" }
    } else {
        Card { name: "many" }
    };

    let plural = if card.name == "many" { "s" } else { "" };
    let macro_name = format!(
        "{}_{}get_{}_{}{}_across_r{}",
        domain,
        cond.name,
        card.name,
        referrer.key_letter.to_snake_case(),
        plural,
        binary.number
    );

    // 👻 This is correct for card == many, cond == unconditional
    // Maybe correct. Definitely not tested to my liking.
    // Abusing {cond|card}.name like this is embarrassing
    let (inner, many) = if card.name == "many" {
        (
            emit_many_conditional_lookup(
                &model,
                &referrer,
                &referent,
                &from.referential_attribute,
                from.conditionality == Conditionality::Conditional,
            ),
            true,
        )
    }
    // 👻 This one should work for card == one, cond == conditional
    else if card.name == "one" && cond.name == "maybe_" {
        (
            emit_one_conditional_lookup(
                &model,
                &referrer,
                &referent,
                &from.referential_attribute,
                from.conditionality == Conditionality::Conditional,
            ),
            false,
        )
    }
    // 👻 This is card == one, cond == unconditional
    else if card.name == "one" && cond.name == "" {
        (
            emit_one_unconditional_lookup(
                &model,
                &referrer,
                &referent,
                &from.referential_attribute,
            ),
            false,
        )
    } else {
        println!("card: {:?} cond: {:?}", card, cond);
        unimplemented!("{} card: {:?} cond: {:?}", function!(), card, cond)
    };

    let body = emit_binary_main(
        &domain,
        &macro_name,
        &referent,
        &referrer,
        binary.number,
        cond.desc == "(c)",
        function!(),
        inner,
        &model,
        package,
        doc_tests,
        true, // This isn't super great either
        many, // And then there is this
    )?;

    *context += body.into();

    Ok(())
}
