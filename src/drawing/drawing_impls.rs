use crate::codegen::Extrude;
/// First extrusion of the Drawing domain
///
/// We are taking the original, ad-hoc "schema", and transform it to the schema that
/// was modeled with Cuckoo (and who's code was generated! (I'm not going to get over
/// that anytime soon, I don't think)).
///
/// These are outer, is that right?, comments because I had the brilliant idea of including
/// this file from the generated file. Nice, huh?
///
/// It's still not user friendly. Iterating on the code generation is an exercise in
/// commenting out things to get to run the generator, and then uncommenting them to
/// see how it went. I'm going to try to not do it live, and see how well my eye picks
/// it up. Actually, why don't I just undo the modified buffer? Duh? Gonna try it.
/// Worked a treat!!!
use crate::drawing::{drawing as to, Context};
use crate::model::jsformat as from;

impl From<from::Direction> for to::Edge {
    fn from(value: from::Direction) -> Self {
        match value {
            from::Direction::North => to::Edge::Top(to::Top),
            from::Direction::East => to::Edge::Right(to::Right),
            from::Direction::South => to::Edge::Bottom(to::Bottom),
            from::Direction::West => to::Edge::Left(to::Left),
        }
    }
}

impl From<from::Point> for to::Point {
    fn from(value: from::Point) -> Self {
        to::Point::new(value.x as i64, value.y as i64)
    }
}

impl to::Point {
    pub fn new(x: i64, y: i64) -> Self {
        Self {
            x,
            y,
            id: Uuid::new_v5(&UUID_NS, format!("{}:{}", x, y).as_bytes()),
        }
    }
}

impl Extrude<from::AnchorPoint, Context<'_>> for to::Anchor {
    fn extrude(value: from::AnchorPoint, context: &mut Context) -> Self {
        let edge: to::Edge = value.dir.into();
        let edge_id = edge.get_id();
        context.drawing.inter_edge(edge);

        let point = to::Point::new(value.x as i64, value.y as i64);
        let point_id = point.id;
        context.drawing.inter_point(point);

        let location = to::Point::new(value.x as i64, value.y as i64);
        let location_id = location.id;
        context.drawing.inter_point(location);

        Self {
            id: Uuid::new_v5(
                &UUID_NS,
                format!("{}:{}:{:?}:{:?}", value.x, value.y, edge_id, point_id).as_bytes(),
            ),
            edge: edge_id,
            offset: point_id,
            location: location_id,
        }
    }
}

impl Extrude<from::RelationshipUI, Context<'_>> for to::RelationshipUI {
    fn extrude(value: from::RelationshipUI, context: &mut Context) -> Self {
        match value {
            from::RelationshipUI::BinaryUI(b) => {
                let b = to::BinaryUI::extrude(b, context);
                let b_id = b.id;
                context.drawing.inter_binary_ui(b);

                to::RelationshipUI::BinaryUI(b_id)
            }
            from::RelationshipUI::IsaUI(i) => {
                let i = to::IsaUI::extrude(i, context);
                let i_id = i.id;
                context.drawing.inter_isa_ui(i);

                to::RelationshipUI::IsaUI(i_id)
            }
            from::RelationshipUI::AssociativeUI(a) => {
                let a = to::AssociativeUI::extrude(a, context);
                let a_id = a.id;
                context.drawing.inter_associative_ui(a);

                to::RelationshipUI::AssociativeUI(a_id)
            }
        }
    }
}

impl Extrude<from::Associative, Context<'_>> for to::AssociativeUI {
    fn extrude(value: from::Associative, context: &mut Context) -> Self {
        let associative = context
            .sarzak
            .exhume_associative(&context.id.take().unwrap())
            .unwrap();

        let middle = to::Anchor::extrude(value.middle, context);
        let middle_id = middle.id;
        context.drawing.inter_anchor(middle);

        let one = to::Anchor::extrude(value.one, context);
        let one_id = one.id;
        context.drawing.inter_anchor(one);

        let other = to::Anchor::extrude(value.other, context);
        let other_id = other.id;
        context.drawing.inter_anchor(other);

        let from: to::Point = value.from.into();
        let from_id = from.id;
        context.drawing.inter_point(from);

        let id = Uuid::new_v5(
            &UUID_NS,
            format!(
                "{}:{}:{}:{}:{}",
                associative.id, from_id, one_id, other_id, middle_id
            )
            .as_bytes(),
        );

        Self {
            id,
            associative_id: associative.id,
            from: from_id,
            one: one_id,
            other: other_id,
            middle: middle_id,
        }
    }
}

impl Extrude<from::PointTuple, Context<'_>> for to::BinaryUI {
    fn extrude(value: from::PointTuple, context: &mut Context) -> Self {
        let binary = context
            .sarzak
            .exhume_binary(&context.id.take().unwrap())
            .unwrap();

        let to = to::Anchor::extrude(value.to, context);
        let to_id = to.id;
        context.drawing.inter_anchor(to);

        let from = to::Anchor::extrude(value.from, context);
        let from_id = from.id;
        context.drawing.inter_anchor(from);

        let id = Uuid::new_v5(
            &UUID_NS,
            format!("{}:{:?}:{:?}", binary.id, from_id, to_id).as_bytes(),
        );

        Self {
            id: id,
            to: to_id,
            from: from_id,
            binary: binary.id,
        }
    }
}

impl Extrude<from::Isa, Context<'_>> for to::IsaUI {
    fn extrude(value: from::Isa, context: &mut Context) -> Self {
        let isa = context
            .sarzak
            .exhume_isa(&context.id.take().unwrap())
            .unwrap();

        let from = to::Anchor::extrude(value.from, context);
        let from_id = from.id;
        context.drawing.inter_anchor(from);

        let id = Uuid::new_v5(&UUID_NS, format!("{}:{:?}", isa.id, from_id).as_bytes());

        Self {
            id: id,
            from: from_id,
            to: value
                .to
                .into_iter()
                .map(|to| {
                    let a = to::Anchor::extrude(to, context);
                    let to_id = a.id;
                    context.drawing.inter_anchor(a);
                    to_id
                })
                .collect(),
            isa: isa.id,
        }
    }
}

/// Convert from a Rect to an ObjectUI
///
/// A [Rect][r] is just (x, y, width, height).
///
/// An [ObjectUI] additionally contains a pointer to the [Object][o] as well as a set
/// of [Edge]s.
///
/// [r]: crate::model::jsformat::Rect
/// [o]: crate::sarzak::Object
impl Extrude<from::Rect, Context<'_>> for to::ObjectUI {
    fn extrude(value: from::Rect, context: &mut Context) -> Self {
        let obj = context
            .sarzak
            .exhume_object(&context.id.take().unwrap())
            .unwrap();

        let id = Uuid::new_v5(
            &UUID_NS,
            format!(
                "{}:{}:{}:{}:{}",
                obj.id, value.x, value.y, value.width, value.height
            )
            .as_bytes(),
        );

        let point = to::Point::new(value.x as i64, value.y as i64);
        let point_id = point.id;
        context.drawing.inter_point(point);

        // We'll need to link to each of the four edges. Silly perhaps, but
        // necessary.
        let edges = vec![to::Top, to::Right, to::Bottom, to::Left];

        Self {
            height: value.height as i64,
            id: id,
            width: value.width as i64,
            object: obj.id.clone(),
            origin: point_id,
            edges,
        }
    }
}
