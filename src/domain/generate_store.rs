//! Generate ObjectStore for some domain
//!
//! This is where all of the instances from the domain model reside.
use std::path::PathBuf;

use heck::{ToSnakeCase, ToTitleCase};
use log::{debug, trace};

use crate::codegen::{emit_generated_code_comments, emitln, Context, Result, SarzakModel};
use crate::sarzak::{Object, AS_IDENT, AS_TYPE};

pub fn generate_store(
    model: &SarzakModel,
    out_file: &PathBuf,
    _package: &str,
    _meta: bool,
    _doc_tests: bool,
) -> Result<()> {
    let domain = &model.domain;

    debug!("Generating ObjectStore for {}.", domain);

    let mut context = Context::new(out_file, false)?;

    let mut objects: Vec<&Object> = model
        .sarzak
        .iter_object()
        .filter_map(|(id, obj)| {
            if model.config.is_imported(&id) || model.config.is_singleton(&id) {
                None
            } else {
                Some(obj)
            }
        })
        .collect();

    // Stabilize the output
    objects.sort();

    // Generate code for all of the objects in the input_schema
    emitln!(
        context,
        "//! ObjectStore for the instances of the \"{}\" domain",
        domain.to_title_case()
    );
    emitln!(context, "//!");
    emitln!(
        context,
        "//! An end user should have little need to use this directly."
    );
    emitln!(context, "//!");
    emitln!(context, "//! This store contains the following instances:");

    // Make some nice documentation
    objects.iter().for_each(|obj| {
        emitln!(context, "//!    * [`{}`]", obj.render(AS_TYPE));
    });

    emitln!(context, "//!");

    // Emit a pointer back to how to generate this file.
    context += emit_generated_code_comments().into();

    emitln!(context, "use std::collections::HashMap;");
    emitln!(context, "");
    emitln!(context, "use serde::{Deserialize, Serialize};");
    emitln!(context, "use uuid::Uuid;");
    emitln!(context, "");

    emit_use_statement(domain, &objects, &mut context);

    emitln!(context, "");
    emitln!(context, "#[derive(Clone, Debug, Deserialize, Serialize)]");
    emitln!(context, "pub struct ObjectStore {");

    // Generate struct fields
    objects.iter().for_each(|obj| {
        trace!("Generating ObjectStore struct field {}.", obj.name);
        emitln!(
            context,
            "    {}: HashMap<Uuid, {}>,",
            obj.render(AS_IDENT),
            obj.render(AS_TYPE)
        );
    });

    emitln!(context, "}");
    emitln!(context, "");

    emitln!(context, "impl ObjectStore {");

    // Generate implementation
    generate_new_impl(&objects, &mut context);
    emitln!(context, "");
    objects.iter().for_each(|obj| {
        generate_type_impls(obj, &model, &mut context);
        emitln!(context, "");
    });

    emitln!(context, "}");

    context.commit()
}

fn generate_new_impl(objects: &Vec<&Object>, context: &mut Context) {
    emitln!(context, "    pub fn new() -> Self {");
    emitln!(context, "        Self {");
    objects.iter().for_each(|obj| {
        emitln!(
            context,
            "            {}: HashMap::new(),",
            obj.render(AS_IDENT)
        );
    });
    emitln!(context, "        }");
    emitln!(context, "    }");
}

fn generate_type_impls(obj: &Object, model: &SarzakModel, context: &mut Context) {
    let lower = obj.render(AS_IDENT);

    emitln!(
        context,
        "    /// Inter [`{}`] into the [`ObjectStore`]",
        obj.render(AS_TYPE)
    );
    emitln!(context, "    ///");
    // emit!(context, "    /// # Example");
    // emit!(context, "    ///");
    // emit!(context, "    ///```");
    // emit!(context, "    ///# use test_macros::ObjectStore;");
    // emit!(context, "    ///# let store = ObjectStore::new();");
    // emit!(context, "    ///");
    // emit!(context, "    ///```");
    emitln!(
        context,
        "    pub fn inter_{}(&mut self, {}: {}) {{",
        lower,
        lower,
        obj.render(AS_TYPE)
    );

    // If this is a a Supertype relationship, we need to call get_id() because
    // it doesn't have an ID of it's own -- it's an enum.
    //
    // I'm trying to decide just how much of a mess this thing is below. Given
    // I have to do a reverse lookup, what other option is there?
    use crate::sarzak::Supertype;
    if model
        .sarzak
        .iter_supertype()
        .filter_map(|(_, s)| if s.obj_id == obj.id { Some(s) } else { None })
        .collect::<Vec<&Supertype>>()
        .len()
        > 0
    {
        emitln!(
            context,
            "        self.{}.insert({}.get_id(), {});",
            lower,
            lower,
            lower
        );
    } else {
        emitln!(
            context,
            "        self.{}.insert({}.id, {});",
            lower,
            lower,
            lower
        );
    }

    emitln!(context, "    }");
    emitln!(context, "");

    emitln!(
        context,
        "    /// Exhume [`{}`] from the [`ObjectStore`]",
        obj.name
    );
    emitln!(context, "    ///");
    emitln!(
        context,
        "    pub fn exhume_{}(&self, id: &Uuid) -> Option<&{}> {{",
        lower,
        obj.render(AS_TYPE)
    );
    emitln!(context, "        self.{}.get(id)", lower);
    emitln!(context, "    }");
    emitln!(context, "");

    emitln!(
        context,
        "    /// Get an iterator over the internal `HashMap<(&Uuid, {})>` in the [`ObjectStore`]",
        obj.render(AS_TYPE)
    );
    emitln!(context, "    ///");
    emitln!(
        context,
        "    pub fn iter_{}(&self) -> impl Iterator<Item = (&Uuid, &{})> {{",
        lower,
        obj.render(AS_TYPE)
    );
    emitln!(context, "        self.{}.iter()", lower);
    emitln!(context, "    }");
}

fn emit_use_statement(domain: &str, objects: &Vec<&Object>, context: &mut Context) {
    let mut use_decl = format!("use crate::{}::types::{{", domain.to_snake_case());

    objects.iter().for_each(|obj| {
        use_decl += &obj.render(AS_TYPE);
        use_decl += ", ";
    });

    use_decl += "};";

    emitln!(context, "{}", use_decl);
}
