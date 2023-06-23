use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::ooa_0::{AttributeName, UUID_NS};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Relationship {
    Binary(Binary),
    Isa(Isa),
    Associative(Associative),
}

impl Relationship {
    pub fn get_number(&self) -> u16 {
        match self {
            Relationship::Binary(b) => b.number,
            Relationship::Isa(i) => i.number,
            Relationship::Associative(a) => a.number,
        }
    }

    pub fn get_id(&self) -> Uuid {
        match self {
            Relationship::Binary(b) => b.id,
            Relationship::Isa(i) => i.id,
            Relationship::Associative(a) => a.id,
        }
    }

    pub fn new_binary(
        number: u16,
        from: Uuid,
        from_cardinality: Cardinality,
        from_conditionality: Conditionality,
        from_description: &str,
        formalizing_attribute_name: &str,
        to: Uuid,
        to_cardinality: Cardinality,
        to_conditionality: Conditionality,
        to_description: &str,
    ) -> Self {
        // To avoid collisions we build the UUID from the following...
        // It would be cool to formalize this. Maybe something like,
        // struct  {
        //
        // }
        // Oh. I was just about to type it out, and realized it's below there. Buried in the
        // format! macro someplace. That format string evaluates to something with meaning. So really
        // there is work to do. It's to associate the formal parameter types in an abstract manner
        // and package them with the formatter.
        let id_str = format!("{}::{}::{}", from, to, number);
        let id = Uuid::new_v5(&UUID_NS, id_str.as_bytes());

        Relationship::Binary(Binary {
            id,
            number,
            from: Independent {
                obj_id: from,
                description: from_description.to_owned(),
                cardinality: from_cardinality,
                conditionality: from_conditionality,
                formalizing_attribute_name: AttributeName::new(
                    formalizing_attribute_name.to_owned(),
                ),
            },
            to: Dependent {
                obj_id: to,
                description: to_description.to_owned(),
                cardinality: to_cardinality,
                conditionality: to_conditionality,
            },
        })
    }

    pub fn new_isa(number: u16, from: Uuid, to: Vec<Uuid>) -> Self {
        let id_str = format!("{}::{}", from, number);
        let id = Uuid::new_v5(&UUID_NS, id_str.as_bytes());

        Relationship::Isa(Isa {
            id,
            number,
            obj_id: from,
            subtypes: to,
        })
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Binary {
    pub id: Uuid,
    pub number: u16,
    pub from: Independent,
    pub to: Dependent,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Dependent {
    pub obj_id: Uuid,
    pub description: String,
    pub cardinality: Cardinality,
    pub conditionality: Conditionality,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Independent {
    pub obj_id: Uuid,
    pub description: String,
    pub cardinality: Cardinality,
    pub conditionality: Conditionality,
    pub formalizing_attribute_name: AttributeName,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Isa {
    pub id: Uuid,
    pub number: u16,
    pub obj_id: Uuid,
    pub subtypes: Vec<Uuid>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Associative {
    pub id: Uuid,
    pub number: u16,
    pub from: AssociativeReferrer,
    pub one: Dependent,   // I'm hijacking these from above. Hopefully to no
    pub other: Dependent, // ill effect.
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct AssociativeReferrer {
    pub id: Uuid,
    pub obj_id: Uuid,
    pub cardinality: Cardinality,
    pub one_referential_attribute: AttributeName,
    pub other_referential_attribute: AttributeName,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Cardinality {
    One,
    Many,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub enum Conditionality {
    Conditional,
    Unconditional,
}
