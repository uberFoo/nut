//! Generated Code -- do not edit
//! Use the following invocation to reproduce. Be careful running when erring.
//! ```ignore
//!  target/debug/gen_code_drawing models/drawing_2.json src/drawing/drawing.rs
//! ```
use serde::{Deserialize, Serialize};
use uuid::{uuid, Uuid};

use crate::drawing::UUID_NS;

/// This represents additional information necessary to render a `Binary` relationship in the user interface.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BinaryUI {
    /// pub id: `Uuid`,
    pub id: Uuid,
    /// pub from: `Anchor`,
    pub from: Uuid,
    /// pub to: `Anchor`,
    pub to: Uuid,
    /// [`nut::sarzak::Binary`]
    pub binary: Uuid,
}

/// This represents additional data necessary to render an `Isa` relationship in the user interface.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IsaUI {
    /// pub id: `Uuid`,
    pub id: Uuid,
    /// pub to: `Vec<Anchor>`,
    pub to: Vec<Uuid>,
    /// [`nut::sarzak::Isa`]
    pub isa: Uuid,
    /// pub from: `Anchor`,
    pub from: Uuid,
}

/// Render a rectangle
///
/// This represents additional information that is necessary to draw an Object in the user interface.
///
/// Note that although we are logically related to [Edge] via `R14` we actually render our own edges. We use the svg rect primitive to do this.
///
/// I’m throwing this in for the fuck of it. I don’t know if it’ll be useful or not.
///
/// ```js
/// var rect = document.createElementNS('http://www.w3.org/2000/svg', 'rect');
/// rect.setAttribute('class', 'objectRect');
/// rect.setAttribute('id', obj.id);
/// rect.setAttribute('x', obj.x);
/// rect.setAttribute('y', obj.y);
/// rect.setAttribute('width', obj.width);
///  rect.setAttribute('height', obj.height);
/// ```
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ObjectUI {
    pub height: i64,
    /// pub id: `Uuid`,
    pub id: Uuid,
    pub width: i64,
    /// pub edges: `Vec<Edge>`,
    pub edges: Vec<Uuid>,
    /// pub origin: `Point`,
    pub origin: Uuid,
    /// [`nut::sarzak::Object`]
    pub object: Uuid,
}

/// Additional information necessary to render relationships in the user interface.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum RelationshipUI {
    /// `BinaryUI(BinaryUI)`,
    BinaryUI(Uuid),
    /// `IsaUI(IsaUI)`,
    IsaUI(Uuid),
    AssociativeUI(Uuid),
}

impl RelationshipUI {
    pub fn get_id(&self) -> Uuid {
        match *self {
            Self::BinaryUI(z) => z,
            Self::IsaUI(z) => z,
            Self::AssociativeUI(z) => z,
        }
    }
}
/// A point is a two-tuple that represents a location on the drawing canvas.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Point {
    /// pub id: `Uuid`,
    pub id: Uuid,
    pub y: i64,
    pub x: i64,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ObjectEdge {
    pub id: Uuid,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct AssociativeUI {
    /// pub id: `Uuid`,
    ///
    pub id: Uuid,
    /// pub from: `Anchor`,
    ///
    pub from: Uuid,
    /// pub other: `Anchor`,
    ///
    pub other: Uuid,
    /// [`nut::sarzak::Associative`]
    ///
    pub associative_id: Uuid,
    /// pub middle: `Point`,
    ///
    pub middle: Uuid,
    /// pub one: `Anchor`,
    ///
    pub one: Uuid,
}

/// The top edge of the rendered box
#[allow(non_upper_case_globals)]
// ⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️
pub const Top: Uuid = uuid!["e9a50304-bcda-5842-8fd3-329876e838c2"];

// #[derive(Clone, Debug, Deserialize, Serialize)]
// pub struct Top (pub Uuid);

// // We are always returning the same thing, as intended. I just wish there were a
// // way to make this a const. Maybe there is -- I haven't really looked around.
// impl Top {
//     pub fn new() -> Self {
//         Self(Uuid::new_v5(&UUID_NS, "Top".as_bytes()))
//     }
// }

/// The left side of a rendered box
#[allow(non_upper_case_globals)]
// ⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️
pub const Left: Uuid = uuid!["52636bac-3f47-5792-8a32-166dbe8af74f"];

// #[derive(Clone, Debug, Deserialize, Serialize)]
// pub struct Left (pub Uuid);

// // We are always returning the same thing, as intended. I just wish there were a
// // way to make this a const. Maybe there is -- I haven't really looked around.
// impl Left {
//     pub fn new() -> Self {
//         Self(Uuid::new_v5(&UUID_NS, "Left".as_bytes()))
//     }
// }

/// The right side of a rendered box
#[allow(non_upper_case_globals)]
// ⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️
pub const Right: Uuid = uuid!["c824949b-058d-5145-981c-4c91a6554d96"];

// #[derive(Clone, Debug, Deserialize, Serialize)]
// pub struct Right (pub Uuid);

// // We are always returning the same thing, as intended. I just wish there were a
// // way to make this a const. Maybe there is -- I haven't really looked around.
// impl Right {
//     pub fn new() -> Self {
//         Self(Uuid::new_v5(&UUID_NS, "Right".as_bytes()))
//     }
// }

/// The bottom of a rendered box
#[allow(non_upper_case_globals)]
// ⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️
pub const Bottom: Uuid = uuid!["2d05ae4a-b681-59d9-8d79-4ea372cc11f1"];

// #[derive(Clone, Debug, Deserialize, Serialize)]
// pub struct Bottom (pub Uuid);

// // We are always returning the same thing, as intended. I just wish there were a
// // way to make this a const. Maybe there is -- I haven't really looked around.
// impl Bottom {
//     pub fn new() -> Self {
//         Self(Uuid::new_v5(&UUID_NS, "Bottom".as_bytes()))
//     }
// }

/// An anchor, or anchor point, is the location where an arrow from a relationship attached to an object.
///
/// Rather than storing the `x` and `y` coordinates of where the anchor attaches, we are related to an [Edge], which is related to a box, which is related to the [Object] to which we are attached. This of course completes the circuit from the [Relationship] for which we are drawing the lines in the first place.
///
/// Anchor also contains a direction, so that we know the orientation to draw the arrows. Finally, there is an offset, which is a point that describes the offset from the anchor for the relationship phrase.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Anchor {
    /// pub id: `Uuid`,
    pub id: Uuid,
    /// pub edge: `Edge`,
    pub edge: Uuid,
    /// pub location: `Point`,
    pub location: Uuid,
    /// pub offset: `Point`,
    pub offset: Uuid,
}

/// An attachment point for an [Anchor]
///
/// It’s used with [Anchor] to orient the arrows on the ends of the lines according to the side of the box to which they are attached. Some arrows are on top, some bottom, etc.
///
/// This is not rendered as a visible item. The [ObjectUI] manages that by itself. This instead renders an invisible line. The line is used for several things. For one, when hovered over the cursor changes to the appropriate one for resizing.
///
/// Also, this is used to register where relationship may anchor.
///
/// It’s this last regard that is somewhat concerning. Indicating that an anchor is attached to an edge get’s us the connection we need between an [Object] and a [Relationship]. But it’s under-specified. It doesn’t indicate where along the edge the arrow is connected.
///
/// I’m considering put a relationship back between [Anchor] and [Point].
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Edge {
    /// `Top(Top)`,
    Top(Uuid),
    /// `Left(Left)`,
    Left(Uuid),
    /// `Bottom(Bottom)`,
    Bottom(Uuid),
    /// `Right(Right)`,
    Right(Uuid),
}

impl Edge {
    pub fn get_id(&self) -> Uuid {
        match *self {
            Self::Top(z) => z,
            Self::Left(z) => z,
            Self::Bottom(z) => z,
            Self::Right(z) => z,
        }
    }
}
include!("drawing_impls.rs");
