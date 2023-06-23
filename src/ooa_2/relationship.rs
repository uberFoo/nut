use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::ooa_0::{
    self,
    relationship::{Cardinality, Conditionality},
};
use crate::ooa_1::{self, UUID_NS};
use crate::ooa_2::{Context, Extrude, ObjectStore};

/// An association between two or more objects
///
/// A relationship connects two or more objects, depending on the type of
/// relationship.
///
/// There are three:
/// * [Binary]
/// * [Isa]
/// * [Associative][ass]
///
/// A binary relationship connects two objects. An isa relationship connects two
/// or more objects. One objects is the [Supertype][sup], and the others are [Subtype][sub]s.
/// An associative relationship associates _three_ objects. TODO: I'll explain once it's
/// actually implemented.
///
/// A relationship has a number, that is unique in the model.
///
/// [ass]: crate::ooa_0::relationship::Associative
/// [sup]: crate::ooa_1::relationship::Supertype
/// [sub]: crate::ooa_1::relationship::Subtype
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Relationship {
    Binary(Uuid),
    Isa(Uuid),
    Associative(Uuid),
}

impl Extrude<ooa_1::Relationship, Context<'_>> for Relationship {
    fn extrude(rel: ooa_1::Relationship, context: &mut Context<'_>) -> Self {
        match rel {
            ooa_1::Relationship::Binary(b) => {
                let b = Binary::extrude(b, context);
                let b_id = b.id;
                context.0.inter_binary(b);
                Self::Binary(b_id)
            }
            ooa_1::Relationship::Isa(i) => {
                let i = Isa::extrude(i, context);
                let i_id = i.id;
                context.0.inter_isa(i);
                Self::Isa(i_id)
            }
            ooa_1::Relationship::Associative(a) => {
                let a = Associative::extrude(a, context);
                let a_id = a.id;
                context.0.inter_associative(a);
                Self::Associative(a_id)
            }
        }
    }
}

impl Relationship {
    pub fn get_number(&self, store: &ObjectStore) -> Option<u16> {
        match self {
            Self::Binary(b) => store.exhume_binary(b).map(|b| b.number),
            Self::Isa(i) => store.exhume_isa(i).map(|i| i.number),
            Self::Associative(_) => todo!(),
        }
    }

    pub fn get_id(&self) -> &Uuid {
        match self {
            Self::Binary(b) => b,
            Self::Isa(i) => i,
            Self::Associative(a) => a,
        }
    }
}

/// A relationship between two objects
///
/// A binary relationship connects two objects in some manner that makes sense
/// to both objects. Thus, there are descriptions for each side of the relationship.
///
/// One side formalizes the relationship by storing a reference to the other side.
/// The formalizing side is called the `Referrer`, and the other the `Referent`.
///
/// In order for the referrer to work, it needs a reference to the referent. This
/// reference takes the shape of an [Attribute][a] on referrer. The attribute
/// is of type [Type::ForeignKey][fk], and stores a UUID. It's basically a pointer.
/// What it points at is the id of the referent.
///
/// The attribute is not actually stored on the [Object][o], but rather it is part
/// of the relationship. Objects realize this attribute as part of their
/// relationship handling.
///
/// [a]: crate::ooa_0::Attribute
/// [fk]: crate::ooa_0::attribute::Type::ForeignKey
/// [o]: crate::ooa_1::Object
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Binary {
    pub id: Uuid,
    pub number: u16,
    pub from: Uuid,
    pub to: Uuid,
}

impl Extrude<ooa_1::relationship::Binary, Context<'_>> for Binary {
    fn extrude(input: ooa_1::relationship::Binary, context: &mut Context<'_>) -> Self {
        let from = input.from.id;
        let to = input.to.id;

        context.0.inter_referrer(input.from);
        context.0.inter_referent(input.to);

        Self {
            id: input.id,
            number: input.number,
            from,
            to,
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Isa {
    pub id: Uuid,
    pub number: u16,
    pub supertype: Uuid,
    pub subtypes: Vec<Uuid>,
}

impl Extrude<ooa_1::relationship::Isa, Context<'_>> for Isa {
    fn extrude(input: ooa_1::relationship::Isa, context: &mut Context<'_>) -> Self {
        let sup = input.supertype.id;
        context.0.inter_supertype(input.supertype);

        let subs = input
            .subtypes
            .into_iter()
            .map(|s| {
                let s_id = s.id;
                context.0.inter_subtype(s);
                s_id
            })
            .collect();

        Self {
            id: input.id,
            number: input.number,
            supertype: sup,
            subtypes: subs,
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Associative {
    pub id: Uuid,
    pub number: u16,
    pub from: Uuid,
    pub one: Uuid,
    pub other: Uuid,
}

impl Extrude<ooa_0::relationship::Associative, Context<'_>> for Associative {
    fn extrude(input: ooa_0::relationship::Associative, context: &mut Context<'_>) -> Self {
        let from = input.from.id;
        context.0.inter_associative_referrer(input.from);

        let one = AssociativeReferent::from_with_assoc_id(input.one, input.id);
        let one_id = one.id;
        context.0.inter_associative_referent(one);

        let other = AssociativeReferent::from_with_assoc_id(input.other, input.id);
        let other_id = other.id;
        context.0.inter_associative_referent(other);

        Self {
            id: input.id,
            number: input.number,
            from,
            one: one_id,
            other: other_id,
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct AssociativeReferent {
    pub id: Uuid,
    pub associative_id: Uuid,
    pub obj_id: Uuid,
    pub description: String,
    pub cardinality: Cardinality,
    pub conditionality: Conditionality,
}

impl AssociativeReferent {
    fn from_with_assoc_id(value: ooa_0::relationship::Dependent, associative_id: Uuid) -> Self {
        let id = Uuid::new_v5(
            &UUID_NS,
            format!("{}::{}", associative_id, value.obj_id).as_bytes(),
        );

        Self {
            id,
            associative_id,
            obj_id: value.obj_id,
            description: value.description,
            cardinality: value.cardinality,
            conditionality: value.conditionality,
        }
    }
}
