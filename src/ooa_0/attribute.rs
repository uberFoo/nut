use std::fmt;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::ooa_0::UUID_NS;

/// AttributeName NewType
///
/// Used to wrap a string so that we can implement Display, and rewrite type.
/// It's actually broken in the Xuder macros. Something about r#type just isn't
/// liked in generated code. So, I'll probably change this to return "ty"
/// instead. Probably in ooa_1 though. Or maybe I won't bother because I don't
/// know if Xuder is going to be needed or not.
#[derive(Clone, Debug, Default, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct AttributeName(String);

// This doesn't work with the way that I'm currently spitting out attribute names.
// I wonder if there's still a way to utilize it though...
//
// Note to self: this is being done in the render method up in sarzak.
impl fmt::Display for AttributeName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0.as_str() {
            "type" => write!(f, "r#{}", self.0),
            _ => write!(f, "{}", self.0),
        }
    }
}

impl AttributeName {
    pub fn new(name: String) -> Self {
        Self(name)
    }

    pub fn inner(&self) -> &String {
        &self.0
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Attribute {
    pub id: Uuid,
    pub name: AttributeName,
    #[serde(rename = "type")]
    pub attr_t: Type,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Type {
    Uuid,
    Integer,
    Float,
    String,
    Boolean,
    #[serde(rename = "foreign_key")]
    ForeignKey(Uuid),
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Uuid => write!(f, "Uuid"),
            Type::Integer => write!(f, "i64"),
            Type::Float => write!(f, "f64"),
            Type::String => write!(f, "std::string::String"),
            Type::Boolean => write!(f, "bool"),
            Type::ForeignKey(u) => write!(f, "&{}", u),
        }
    }
}

impl Attribute {
    pub fn new(name: &str, ty: Type) -> Self {
        // This will likely result in namespace collisions. It needs to be hashed with the name
        // of the object to which it belongs.
        let id = Uuid::new_v5(&UUID_NS, name.as_bytes());

        Attribute {
            id,
            name: AttributeName(name.to_owned()),
            attr_t: ty,
        }
    }
}
