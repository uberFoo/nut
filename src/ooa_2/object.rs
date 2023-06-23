use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::ooa_1::object::RelPointer;
use crate::ooa_2::{AttributeName, Context, Extrude};

/// The mighty Object
///
/// This is just an abstract way of representing data in memory. It may be
/// translated into a `struct` or an `enum`. Or maybe some of each? No way
/// to say just now.
///
/// I'm wondering what it would look like to have a pointer to the original ooa_0
/// object here. And then make the methods point back to the original? Seems
/// like there would be little benefit.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Object {
    pub id: Uuid,
    pub key_letter: String,
    pub name: AttributeName,
    pub description: String,
    /// These are the [Attribute][a]s that belong to the `Object`. The relationship
    /// between `Object` and `Attribute` in Cuckoo is R1. It's conditional, which
    /// goes against what I said elsewhere. Someplace I suggested that conditional
    /// relationships aren't owned by the [Independent][i] side.
    ///
    /// In any case they are owned by `Object` and stored in a `HashMap`, indexed
    /// by the attribute's name.
    ///
    /// [a]: crate::ooa_0::attribute::Attribute
    /// [i]: crate::ooa_0::relationship::Independent
    pub attributes: HashMap<String, Uuid>,
    // We can do better than just point at the relationship. It would be great
    // if we could tell what part of the relationship we are involved with.
    pub rels: HashMap<String, RelPointer>,
    pub is_referrer: bool,
}

/// Extrude an [ooa_1::Object] to an [Object]
///
use crate::ooa_1;
impl Extrude<ooa_1::Object, Context<'_>> for Object {
    fn extrude(input: ooa_1::Object, context: &mut Context) -> Self {
        // Extract the Attributes from the map, and store them in the ObjectStore.
        let attributes = input
            .attributes
            .into_iter()
            .map(|(k, a)| {
                let result = (k, a.id);
                context.0.inter_attribute(a);
                result
            })
            .collect::<Vec<(String, Uuid)>>()
            .into_iter()
            .collect::<HashMap<String, Uuid>>();

        Self {
            id: input.id,
            key_letter: input.key_letter,
            name: input.name,
            description: input.description,
            attributes,
            rels: input.rels,
            is_referrer: input.is_referrer,
        }
    }
}
