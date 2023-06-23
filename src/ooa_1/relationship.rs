//! Relationship Transmutation
//!
//! Binary referent and referrer, whatever they _were_ called had to get fixed here.
//! Same thing for Isa, but for different reasons.
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::ooa_0::{
    self,
    attribute::AttributeName,
    relationship::{Associative, Cardinality, Conditionality},
};
use crate::ooa_1::UUID_NS;

/// An association between two or more objects
///
/// A relationship connects two or more objects, depending on the type of
/// relationship.
///
/// There are three:
/// * [Binary]
/// * [Isa]
/// * [Associative]
///
/// A binary relationship connects two objects. An isa relationship connects two
/// or more objects. One objects is the [Supertype], and the others are [Subtype]s.
/// An associative relationship associates _three_ objects. TODO: I'll explain once it's
/// actually implemented.
///
/// A relationship has a number, that is unique in the model.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Relationship {
    Binary(Binary),
    Isa(Isa),
    Associative(Associative),
}

impl From<ooa_0::Relationship> for Relationship {
    fn from(rel: ooa_0::Relationship) -> Self {
        match rel {
            ooa_0::Relationship::Binary(b) => Self::Binary(b.into()),
            ooa_0::Relationship::Isa(i) => Self::Isa(i.into()),
            ooa_0::Relationship::Associative(a) => Self::Associative(a),
        }
    }
}

impl Relationship {
    pub fn get_number(&self) -> u16 {
        match self {
            Self::Binary(b) => b.number,
            Self::Isa(i) => i.number,
            Self::Associative(a) => a.number,
        }
    }

    pub fn get_id(&self) -> Uuid {
        match self {
            Self::Binary(b) => b.id,
            Self::Isa(i) => i.id,
            Self::Associative(a) => a.id,
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
    pub from: Referrer,
    pub to: Referent,
}

impl From<ooa_0::relationship::Binary> for Binary {
    // Note that we are swizzling the from and to, as well as changing their type.
    fn from(value: ooa_0::relationship::Binary) -> Self {
        // Notice that we are swapping types with the into() calls.
        let (from, to) = (
            Referrer::from_with_bin_id(
                value.from.clone(),
                value.id,
                &value.from.formalizing_attribute_name,
            ),
            Referent::from_with_bin_id(value.to, value.id),
        );

        Self {
            id: value.id,
            number: value.number,
            from: from,
            to: to,
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Referrer {
    pub id: Uuid,
    pub binary_id: Uuid,
    pub obj_id: Uuid,
    pub referential_attribute: AttributeName,
    pub description: String,
    pub cardinality: Cardinality,
    pub conditionality: Conditionality,
}

impl Referrer {
    fn from_with_bin_id(
        value: ooa_0::relationship::Independent,
        binary_id: Uuid,
        attr_name: &AttributeName,
    ) -> Self {
        let id_str = format!("Referrer::{}", binary_id);
        let id = Uuid::new_v5(&UUID_NS, id_str.as_bytes());

        Self {
            id,
            binary_id,
            obj_id: value.obj_id,
            description: value.description,
            cardinality: value.cardinality,
            conditionality: value.conditionality,
            referential_attribute: attr_name.clone(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Referent {
    pub id: Uuid,
    pub binary_id: Uuid,
    pub obj_id: Uuid,
    pub description: String,
    pub cardinality: Cardinality,
    pub conditionality: Conditionality,
}

impl Referent {
    fn from_with_bin_id(value: ooa_0::relationship::Dependent, binary_id: Uuid) -> Self {
        let id_str = format!("Referent::{}", binary_id);
        let id = Uuid::new_v5(&UUID_NS, id_str.as_bytes());

        Self {
            id,
            binary_id,
            obj_id: value.obj_id,
            description: value.description,
            cardinality: value.cardinality,
            conditionality: value.conditionality,
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Isa {
    pub id: Uuid,
    pub number: u16,
    pub supertype: Supertype,
    pub subtypes: Vec<Subtype>,
}

impl From<ooa_0::relationship::Isa> for Isa {
    fn from(rel: ooa_0::relationship::Isa) -> Self {
        // We don't have a supertype in ooa_0. This is nifty.
        let sup = Supertype::new(&rel.id, rel.obj_id);

        Self {
            id: rel.id,
            number: rel.number,
            supertype: sup,
            subtypes: rel
                .subtypes
                .iter()
                .map(|s| Subtype::new(&rel.id, *s))
                .collect(),
        }
    }
}

/// This is a brand new class and has no From impl.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Supertype {
    pub id: Uuid,
    /// This formalizes the [Isa] relationship.
    pub isa_id: Uuid,
    /// This is the [Object](crate::ooa_0::object::Object) to which the relationship points.
    pub obj_id: Uuid,
}

impl Supertype {
    pub fn new(isa: &Uuid, obj_id: Uuid) -> Self {
        let id_str = format!("{}::{}", isa, obj_id);
        let id = Uuid::new_v5(&UUID_NS, id_str.as_bytes());

        Self {
            id,
            isa_id: isa.to_owned(),
            obj_id: obj_id.clone(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Subtype {
    pub id: Uuid,
    /// This formalizes the [Isa] relationship.
    pub isa_id: Uuid,
    /// This is the [Object](crate::ooa_0::object::Object) to which the relationship points.
    pub obj_id: Uuid,
}

impl Subtype {
    pub fn new(isa: &Uuid, obj_id: Uuid) -> Self {
        let id_str = format!("{}::{}", isa, obj_id);
        let id = Uuid::new_v5(&UUID_NS, id_str.as_bytes());

        Self {
            id,
            isa_id: isa.to_owned(),
            obj_id: obj_id.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_ooa_0_to_ooa_1() {
        let json_isa = r#"
        {
            "id": "a097f6f4-4d97-5ef6-b359-3021766ec90b",
            "number": 3,
            "obj_id": "b8ec6afc-ddbd-53d6-9be3-e4b738941c2f",
            "subtypes": [
                "fae606a2-e37c-5f82-8754-1fc11c09fe4c",
                "f3d5c0a4-850d-5071-a7e3-50e53389e3a8",
                "04fbbc6c-a351-5e6d-b193-191f5510033e",
                "9803e73c-4984-5179-8460-529fe4ef7921"
            ]
        }"#;
        let r = serde_json::from_str::<ooa_0::relationship::Isa>(json_isa).unwrap();

        let isa: Isa = r.clone().into();

        assert_eq!(isa.supertype.obj_id, r.obj_id);
        assert_eq!(isa.subtypes[0].obj_id, r.subtypes[0]);
        assert_eq!(isa.subtypes[1].obj_id, r.subtypes[1]);
        assert_eq!(isa.subtypes[2].obj_id, r.subtypes[2]);
        assert_eq!(isa.subtypes[3].obj_id, r.subtypes[3]);
    }
}
