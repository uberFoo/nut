use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::ooa_0::attribute::Attribute;
use crate::ooa_0::{AttributeName, UUID_NS};

/// In general an object is a memory representation of a set of collected attributes/fields/members
/// (we call them Attributes), and functions. We are currently ignoring functions.
///
/// Please don't object about this not being called `Class`.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Object {
    pub id: Uuid,
    pub key_letter: String,
    pub name: AttributeName,
    pub description: String,
    pub attributes: HashMap<Uuid, Attribute>,
}

impl Object {
    pub fn new(name: &str, key_letter: &str) -> Self {
        Object {
            id: Uuid::new_v5(&UUID_NS, name.as_bytes()),
            key_letter: key_letter.to_owned(),
            name: AttributeName::new(name.to_owned()),
            description: String::new(),
            attributes: HashMap::new(),
        }
    }

    pub fn add_attribute(mut self, mut attr: Attribute) -> Self {
        // We need to prepend our ID to the attribute name as a namespace to avoid attribute
        // name collisions.
        let id_str = format!("{}::{}", self.name, attr.name);
        let id = Uuid::new_v5(&UUID_NS, id_str.as_bytes());
        attr.id = id;

        self.attributes.insert(id, attr);

        self
    }
}
