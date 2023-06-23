//! ObjectStore for the instances of the "Drawing" domain
//!
//! An end user should have little need to use this directly.
//!
//! This store contains the following instances:
//!    * [`Anchor`]
//!    * [`BinaryUI`]
//!    * [`Point`]
//!    * [`ObjectEdge`]
//!    * [`Edge`]
//!    * [`RelationshipUI`]
//!    * [`ObjectUI`]
//!    * [`IsaUI`]
//!    * [`AssociativeUI`]
//!
//! Generated Code -- do not edit -- yet.
//! Use the following invocation to reproduce:
//! ```shell
//!  target/debug/sarzak gen drawing -p ../nut/crates/sarzak
//! ```
use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::drawing::{
    Anchor, AssociativeUI, BinaryUI, Edge, IsaUI, ObjectEdge, ObjectUI, Point, RelationshipUI,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ObjectStore {
    anchor: HashMap<Uuid, Anchor>,
    binary_ui: HashMap<Uuid, BinaryUI>,
    point: HashMap<Uuid, Point>,
    object_edge: HashMap<Uuid, ObjectEdge>,
    edge: HashMap<Uuid, Edge>,
    relationship_ui: HashMap<Uuid, RelationshipUI>,
    object_ui: HashMap<Uuid, ObjectUI>,
    isa_ui: HashMap<Uuid, IsaUI>,
    associative_ui: HashMap<Uuid, AssociativeUI>,
}

impl ObjectStore {
    pub fn new() -> Self {
        Self {
            anchor: HashMap::new(),
            binary_ui: HashMap::new(),
            point: HashMap::new(),
            object_edge: HashMap::new(),
            edge: HashMap::new(),
            relationship_ui: HashMap::new(),
            object_ui: HashMap::new(),
            isa_ui: HashMap::new(),
            associative_ui: HashMap::new(),
        }
    }

    /// Inter [`Anchor`] into the [`ObjectStore`]
    ///
    pub fn inter_anchor(&mut self, anchor: Anchor) {
        self.anchor.insert(anchor.id, anchor);
    }

    /// Exhume [`Anchor`] from the [`ObjectStore`]
    ///
    pub fn exhume_anchor(&self, id: &Uuid) -> Option<&Anchor> {
        self.anchor.get(id)
    }

    /// Get an iterator over the internal `HashMap<(&Uuid, Anchor)>` in the [`ObjectStore`]
    ///
    pub fn iter_anchor(&self) -> impl Iterator<Item = (&Uuid, &Anchor)> {
        self.anchor.iter()
    }

    /// Inter [`BinaryUI`] into the [`ObjectStore`]
    ///
    pub fn inter_binary_ui(&mut self, binary_ui: BinaryUI) {
        self.binary_ui.insert(binary_ui.id, binary_ui);
    }

    /// Exhume [`BinaryUI`] from the [`ObjectStore`]
    ///
    pub fn exhume_binary_ui(&self, id: &Uuid) -> Option<&BinaryUI> {
        self.binary_ui.get(id)
    }

    /// Get an iterator over the internal `HashMap<(&Uuid, BinaryUI)>` in the [`ObjectStore`]
    ///
    pub fn iter_binary_ui(&self) -> impl Iterator<Item = (&Uuid, &BinaryUI)> {
        self.binary_ui.iter()
    }

    /// Inter [`Point`] into the [`ObjectStore`]
    ///
    pub fn inter_point(&mut self, point: Point) {
        self.point.insert(point.id, point);
    }

    /// Exhume [`Point`] from the [`ObjectStore`]
    ///
    pub fn exhume_point(&self, id: &Uuid) -> Option<&Point> {
        self.point.get(id)
    }

    /// Get an iterator over the internal `HashMap<(&Uuid, Point)>` in the [`ObjectStore`]
    ///
    pub fn iter_point(&self) -> impl Iterator<Item = (&Uuid, &Point)> {
        self.point.iter()
    }

    /// Inter [`ObjectEdge`] into the [`ObjectStore`]
    ///
    pub fn inter_object_edge(&mut self, object_edge: ObjectEdge) {
        self.object_edge.insert(object_edge.id, object_edge);
    }

    /// Exhume [`Object Edge`] from the [`ObjectStore`]
    ///
    pub fn exhume_object_edge(&self, id: &Uuid) -> Option<&ObjectEdge> {
        self.object_edge.get(id)
    }

    /// Get an iterator over the internal `HashMap<(&Uuid, ObjectEdge)>` in the [`ObjectStore`]
    ///
    pub fn iter_object_edge(&self) -> impl Iterator<Item = (&Uuid, &ObjectEdge)> {
        self.object_edge.iter()
    }

    /// Inter [`Edge`] into the [`ObjectStore`]
    ///
    pub fn inter_edge(&mut self, edge: Edge) {
        self.edge.insert(edge.get_id(), edge);
    }

    /// Exhume [`Edge`] from the [`ObjectStore`]
    ///
    pub fn exhume_edge(&self, id: &Uuid) -> Option<&Edge> {
        self.edge.get(id)
    }

    /// Get an iterator over the internal `HashMap<(&Uuid, Edge)>` in the [`ObjectStore`]
    ///
    pub fn iter_edge(&self) -> impl Iterator<Item = (&Uuid, &Edge)> {
        self.edge.iter()
    }

    /// Inter [`RelationshipUI`] into the [`ObjectStore`]
    ///
    pub fn inter_relationship_ui(&mut self, relationship_ui: RelationshipUI) {
        self.relationship_ui
            .insert(relationship_ui.get_id(), relationship_ui);
    }

    /// Exhume [`RelationshipUI`] from the [`ObjectStore`]
    ///
    pub fn exhume_relationship_ui(&self, id: &Uuid) -> Option<&RelationshipUI> {
        self.relationship_ui.get(id)
    }

    /// Get an iterator over the internal `HashMap<(&Uuid, RelationshipUI)>` in the [`ObjectStore`]
    ///
    pub fn iter_relationship_ui(&self) -> impl Iterator<Item = (&Uuid, &RelationshipUI)> {
        self.relationship_ui.iter()
    }

    /// Inter [`ObjectUI`] into the [`ObjectStore`]
    ///
    pub fn inter_object_ui(&mut self, object_ui: ObjectUI) {
        self.object_ui.insert(object_ui.id, object_ui);
    }

    /// Exhume [`ObjectUI`] from the [`ObjectStore`]
    ///
    pub fn exhume_object_ui(&self, id: &Uuid) -> Option<&ObjectUI> {
        self.object_ui.get(id)
    }

    /// Get an iterator over the internal `HashMap<(&Uuid, ObjectUI)>` in the [`ObjectStore`]
    ///
    pub fn iter_object_ui(&self) -> impl Iterator<Item = (&Uuid, &ObjectUI)> {
        self.object_ui.iter()
    }

    /// Inter [`IsaUI`] into the [`ObjectStore`]
    ///
    pub fn inter_isa_ui(&mut self, isa_ui: IsaUI) {
        self.isa_ui.insert(isa_ui.id, isa_ui);
    }

    /// Exhume [`IsaUI`] from the [`ObjectStore`]
    ///
    pub fn exhume_isa_ui(&self, id: &Uuid) -> Option<&IsaUI> {
        self.isa_ui.get(id)
    }

    /// Get an iterator over the internal `HashMap<(&Uuid, IsaUI)>` in the [`ObjectStore`]
    ///
    pub fn iter_isa_ui(&self) -> impl Iterator<Item = (&Uuid, &IsaUI)> {
        self.isa_ui.iter()
    }

    /// Inter [`AssociativeUI`] into the [`ObjectStore`]
    ///
    pub fn inter_associative_ui(&mut self, associative_ui: AssociativeUI) {
        self.associative_ui
            .insert(associative_ui.id, associative_ui);
    }

    /// Exhume [`AssociativeUI`] from the [`ObjectStore`]
    ///
    pub fn exhume_associative_ui(&self, id: &Uuid) -> Option<&AssociativeUI> {
        self.associative_ui.get(id)
    }

    /// Get an iterator over the internal `HashMap<(&Uuid, AssociativeUI)>` in the [`ObjectStore`]
    ///
    pub fn iter_associative_ui(&self) -> impl Iterator<Item = (&Uuid, &AssociativeUI)> {
        self.associative_ui.iter()
    }
}
