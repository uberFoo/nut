//! Things that will help when generating types
//!
use crate::sarzak::{Attribute, AttributeName, Object};

/// I called it Parameter...
///
/// ...when deconstructing attributes. I can't think of anything better right now.
/// It needs to store information that we can use when building the fields in the
/// struct. I guess I could call it Field!
///
/// It hold all that. I want to also use it to build up an argument list to the
/// new method. And also for building a format string for Uuid.
///
/// So I need to know how to construct these things, and I need to know how to get
/// a hold of a string representation.
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Field<'a> {
    Reference(Ref<'a>),
    Attribute(&'a Attribute),
}

impl<'a> Field<'a> {
    pub fn render(&self, t: &str) -> String {
        match self {
            Self::Reference(r) => r.referent.render(t),
            Self::Attribute(a) => a.render(t),
        }
    }
}

/// A Reference Field
///
/// This is a field that comes from a referential attribute. It's "imported"
/// from the relationship. It's one of two field types generated for a struct
/// implementation of an [`Object`].
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Ref<'a> {
    /// The [`Referent`]
    ///
    /// This comes from the side being pointed at in the relationship.
    pub referent: &'a Object,
    /// The Referential Attribute
    ///
    /// This is just a string, wrapped up in an [`AttributeName`]. It's how the
    /// [`Referrer`] refers to the [`Referent`].
    pub ref_attr: &'a AttributeName,
    /// Optional Flag
    ///
    /// This indicates that the referential attribute in optional. This is the
    /// result of the conditionality of the relationship on the [`Referrer`]
    /// side is conditional.
    ///
    /// This generally means that we will emit the referential attribute wrapped
    /// in an `Option`.
    pub optional: bool,
}
