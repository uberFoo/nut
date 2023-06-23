use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub use crate::ooa_0::AttributeName;
use crate::ooa_1::{extrude::Extrude, Attribute, ObjectContext, Relationship};

/// An enum to disambiguate relationship type
///
/// This allows us to determine the type of relationship with which we are involved.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum RelSide {
    Referent,
    Referrer,
    Subtype,
    Supertype,
    AssocFrom,
    AssocOne,
    AssocOther,
}

/// A struct to store a pointer and type
///
/// This just bundles together a UUID and a `RelSide` enum. I could have used a
/// tuple, but I like things having names. Plus, there's no difference in memory
/// compared to a tuple struct.
///
/// _NB_: `value` points at the relationship variant.
/// _NB_: `side` indicates the type of _this_ side.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct RelPointer {
    pub value: Uuid,
    pub side: RelSide,
}

/// The mighty Object
///
/// This is just an abstract way of representing data in memory. It may be
/// translated into a `struct` or an `enum`. Or maybe some of each? No way
/// to say just now.
///
/// I'm wondering what it would look like to have a pointer to the original ooa_0
/// object here. And then make the methods point back to the original? Seems
/// like there would be little benefit.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Object {
    pub id: Uuid,
    pub key_letter: String,
    /// The name of the object.
    ///
    /// Why is this an AttributeName? When did I come up with that?
    pub name: AttributeName,
    pub description: String,
    /// These are the [Attribute]s that belong to the `Object`. The relationship
    /// between `Object` and `Attribute` in Cuckoo is R1. It's conditional, which
    /// goes against what I said elsewhere. Someplace I suggested that conditional
    /// relationships aren't owned by the [Independent][i] side.
    ///
    /// In any case they are owned by `Object` and stored in a `HashMap`, indexed
    /// by the attribute's name.
    ///
    /// [i]: crate::ooa_0::relationship::Independent
    pub attributes: HashMap<String, Attribute>,
    // We can do better than just point at the relationship. It would be great
    // if we could tell what part of the relationship we are involved with.
    pub rels: HashMap<String, RelPointer>,
    /// This isn't being used.
    ///
    /// It should die.
    pub is_referrer: bool,
}

/// Extrude an [ooa_0::Object] to an [Object]
///
use crate::ooa_0;
impl Extrude<ooa_0::Object, ObjectContext<'_>> for Object {
    fn extrude(input: ooa_0::Object, context: &mut ObjectContext) -> Self {
        // First off, let's fix the attribute key in the hashmap.
        // Also, let's fix a bug where the attribute id's are fucked up.
        // See https://git.uberfoo.com/sarzak/sarzak/-/issues/1
        let attributes = input
            .attributes
            .into_iter()
            .map(|(_, mut v)| {
                let id_should_be = Uuid::new_v5(
                    &crate::ooa_0::UUID_NS,
                    format!("{}::{}", input.id, v.name.inner()).as_bytes(),
                );
                if v.id != id_should_be {
                    log::info!(
                        "changing id of attribute {}::{} to {}",
                        input.name,
                        v.name,
                        id_should_be
                    );
                    v.id = id_should_be;
                }
                let name = v.name.inner();
                (name.to_owned(), v)
            })
            .into_iter()
            .collect::<HashMap<String, Attribute>>();

        let mut referrer = false;
        // Next we need to tackle relationship navigation.
        // RelPointer::value should in all cases point to the relationship
        // variant.
        let rels = context
            .relationships
            .iter()
            .filter_map(|(_, r)| match r {
                Relationship::Binary(b) => {
                    if b.from.obj_id == input.id {
                        referrer = true;
                        let name = format!("R{}", r.get_number());
                        Some((
                            name,
                            RelPointer {
                                value: b.id,
                                side: RelSide::Referrer,
                            },
                        ))
                    } else if b.to.obj_id == input.id {
                        let name = format!("R{}", r.get_number());
                        Some((
                            name,
                            RelPointer {
                                value: b.id,
                                side: RelSide::Referent,
                            },
                        ))
                    } else {
                        None
                    }
                }
                Relationship::Associative(a) => {
                    if a.from.obj_id == input.id {
                        let name = format!("R{}", r.get_number());
                        Some((
                            name,
                            RelPointer {
                                value: a.id,
                                side: RelSide::AssocFrom,
                            },
                        ))
                    } else if a.one.obj_id == input.id {
                        let name = format!("R{}", r.get_number());
                        Some((
                            name,
                            RelPointer {
                                value: a.id,
                                side: RelSide::AssocOne,
                            },
                        ))
                    } else if a.other.obj_id == input.id {
                        let name = format!("R{}", r.get_number());
                        Some((
                            name,
                            RelPointer {
                                value: a.id,
                                side: RelSide::AssocOther,
                            },
                        ))
                    } else {
                        None
                    }
                }
                // This is yukky because we still have to deal with the ooa_0 messed-up
                // implementation that's missing supertype and subtype, and is just
                // generally jacked up.
                Relationship::Isa(i) => {
                    if i.supertype.obj_id == input.id {
                        let name = format!("R{}", r.get_number());
                        Some((
                            name,
                            RelPointer {
                                value: i.id,
                                side: RelSide::Supertype,
                            },
                        ))
                    } else {
                        let name = format!("R{}", r.get_number());
                        i.subtypes
                            .iter()
                            .find(|s| s.obj_id == input.id)
                            .and_then(|_| {
                                Some((
                                    name,
                                    RelPointer {
                                        value: i.id,
                                        side: RelSide::Subtype,
                                    },
                                ))
                            })
                    }
                }
            })
            .collect::<Vec<(String, RelPointer)>>()
            .into_iter()
            .collect::<HashMap<String, RelPointer>>();

        // I don't know where this originated, but there are some of these in sarzak
        // and drawing...
        // We can't fix them because of cross domain dependencies, but it shouldn't
        // be a problem really.
        let id_should_be = Uuid::new_v5(&crate::ooa_0::UUID_NS, input.name.inner().as_bytes());
        if input.id != id_should_be {
            log::info!("changing id of object {} to {}", input.name, id_should_be);
        }

        Self {
            id: input.id,
            key_letter: input.key_letter,
            name: input.name,
            description: input.description,
            attributes,
            rels,
            is_referrer: referrer,
        }
    }
}
