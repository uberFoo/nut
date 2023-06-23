//! This is NUT!!! 🤣
//!
//! This is my little experiment in code generation. Take a model and turn it into
//! code. Since my favorite, and about the only language I can work in, is Rust,
//! I'm going to generate Rust. At least that's the plan. Don't shoot me if I
//! generate some Typescript along the way. It could happen.
//!
//! # Module Structure
//! I'm doing the code generation in stages. I sort of have a general idea of how
//! it will work. Nothing quite coherent though. So I'm nudging things in the general
//! direction I'm feeling at the time. When it feels like a good place to stop, which
//! has generally been one simple transformation, then I start a new module.
//!
//! ## ooa_0
//! I started with a handful of [structs and enums][ooa_0] and a binary that created
//! instances of said enums and spit out JSON. I got bored creating instances of
//! them by hand. So I built Cuckoo, and together they evolved.
//!
//! I got to the point that Cuckoo is pretty stable, and it seemed that [ooa_0]
//! would be frozen. That hasn't turned out to be exactly true, but the Cuckoo facing
//! stuff has been mostly stable.
//!
//! So it seemed like a good time to start working on [ooa_1].
//!
//! ## ooa_1
//! This is a nascent work in progress. The general direction it seems to be taking
//! is changing the Javascript friendly JSON into something more abstract. Well,
//! maybe it's not changing the level of abstraction. It's rendering a homogeneous
//! way to navigate relationships. I'm thinking that this is going to be driven
//! by relationships. If they are in fact functors, then that would make sense, I think.
//!
//! We'll see how it goes.
//!
//! ## ooa_2
//! It went ok. The problem we are solving now is adding relationship pointers
//! to objects. That is, for each relationship connected to an object, we want
//! to make it available to the object itself. We didn't do this last time because
//! we didn't have updated relationships during extrusion. Now we do!
//!
//! ## ooa_1 & 2 redo
//!
//! I assume this is going to get cleaned out sometime, and this will disappear.
//! In the mean time, notes! It turns out that ooa_2 was a mistake. I'm going the
//! total other direction -- normalizing it all. So I'm redoing ooa_1 basically.
//!
//! ## model
//!
//! So the redo went really well. I think everything is normalized now. There are some
//! macros for relationship navigation. All in all, it's pretty slick.
//!
//! Now I need to import the model domain side of things from Cuckoo. It's for Nutter,
//! the new UI.
//!
//! ## drawing
//!
//! This is _generated_. It's the domain for storing drawing geometry. The program that
//! performs the generation is `bin/drawing/gen_code.rs`.
pub mod codegen;
pub mod domain;
pub mod drawing;
pub mod model;
pub(crate) mod ooa_0;
pub(crate) mod ooa_1;
pub(crate) mod ooa_2;
pub mod sarzak;

pub use model::extract_ooa2;

// Most of below is to support legacy code that worked with certain versions of the
// data structures. I don't expect that these will be generally useful, and going
// forward everything will import from one of the top level public modules.
//
// I can maybe see this schema stuff being moved to codegen.
pub use ooa_0::{
    Object as Object_v0, ObjectType, ReadCuckooModel, ReadSchema as ReadSchema_v0,
    Relationship as Relationship_v0, Schema as Schema_v0, WriteSchema as WriteSchema_v0,
    UUID_NS as UUID_NS_v0,
};
pub use ooa_1::{
    ReadSchema as ReadSchema_v1, Relationship as Relationship_V1, Schema as Schema_v1,
    WriteSchema as WriteSchema_v1,
};
