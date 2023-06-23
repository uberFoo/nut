//! ObjectStore for the instances of the "Associative" domain
//!
//! An end user should have little need to use this directly.
//!
//! This store contains the following instances:
//!    * [`Anchor`]
//!    * [`AcknowledgedEvent`]
//!    * [`State`]
//!    * [`IsaUi`]
//!    * [`SubtypeAnchor`]
//!    * [`Event`]
//!
//! # Generated Code -- edit _with care_.
//!
//! Don't mess with anything between `{"magic":"","kind":"CriticalBlockBegin"}`
//! and `{"magic":"","kind":"CriticalBlockEnd"}`. Otherwise, you should be free
//! to go wild. Happy hacking!
//!
//! Use the following invocation to reproduce:
// {"magic":"","kind":{"IgnoreBlockBegin":{}}}
// {"magic":"","kind":{"IgnoreBlockBegin":{}}}
//! ```shell
//!  /Users/uberfoo/projects/sarzak/nut/target/debug/deps/generate_test_domain-0ff2341248e740c2 --nocapture
//!  /Users/uberfoo/projects/sarzak/nut/target/debug/deps/generate_test_domain-145fdb9ab1f4b4be --nocapture
//! ```
// {"magic":"","kind":"IgnoreBlockEnd"}
// {"magic":"","version":"0.2.0"}
// {"magic":"","version":"0.5.0"}
// {"magic":"","version":"1.0.0"}
use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::associative::types::{AcknowledgedEvent, Anchor, Event, IsaUi, State, SubtypeAnchor};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ObjectStore {
    anchor: HashMap<Uuid, Anchor>,
    acknowledged_event: HashMap<Uuid, AcknowledgedEvent>,
    state: HashMap<Uuid, State>,
    isa_ui: HashMap<Uuid, IsaUi>,
    subtype_anchor: HashMap<Uuid, SubtypeAnchor>,
    event: HashMap<Uuid, Event>,
}

impl ObjectStore {
    pub fn new() -> Self {
        Self {
            anchor: HashMap::new(),
            acknowledged_event: HashMap::new(),
            state: HashMap::new(),
            isa_ui: HashMap::new(),
            subtype_anchor: HashMap::new(),
            event: HashMap::new(),
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

    /// Inter [`AcknowledgedEvent`] into the [`ObjectStore`]
    ///
    pub fn inter_acknowledged_event(&mut self, acknowledged_event: AcknowledgedEvent) {
        self.acknowledged_event
            .insert(acknowledged_event.id, acknowledged_event);
    }

    /// Exhume [`Acknowledged Event`] from the [`ObjectStore`]
    ///
    pub fn exhume_acknowledged_event(&self, id: &Uuid) -> Option<&AcknowledgedEvent> {
        self.acknowledged_event.get(id)
    }

    /// Get an iterator over the internal `HashMap<(&Uuid, AcknowledgedEvent)>` in the [`ObjectStore`]
    ///
    pub fn iter_acknowledged_event(&self) -> impl Iterator<Item = (&Uuid, &AcknowledgedEvent)> {
        self.acknowledged_event.iter()
    }

    /// Inter [`State`] into the [`ObjectStore`]
    ///
    pub fn inter_state(&mut self, state: State) {
        self.state.insert(state.id, state);
    }

    /// Exhume [`State`] from the [`ObjectStore`]
    ///
    pub fn exhume_state(&self, id: &Uuid) -> Option<&State> {
        self.state.get(id)
    }

    /// Get an iterator over the internal `HashMap<(&Uuid, State)>` in the [`ObjectStore`]
    ///
    pub fn iter_state(&self) -> impl Iterator<Item = (&Uuid, &State)> {
        self.state.iter()
    }

    /// Inter [`IsaUi`] into the [`ObjectStore`]
    ///
    pub fn inter_isa_ui(&mut self, isa_ui: IsaUi) {
        self.isa_ui.insert(isa_ui.id, isa_ui);
    }

    /// Exhume [`IsaUI`] from the [`ObjectStore`]
    ///
    pub fn exhume_isa_ui(&self, id: &Uuid) -> Option<&IsaUi> {
        self.isa_ui.get(id)
    }

    /// Get an iterator over the internal `HashMap<(&Uuid, IsaUi)>` in the [`ObjectStore`]
    ///
    pub fn iter_isa_ui(&self) -> impl Iterator<Item = (&Uuid, &IsaUi)> {
        self.isa_ui.iter()
    }

    /// Inter [`SubtypeAnchor`] into the [`ObjectStore`]
    ///
    pub fn inter_subtype_anchor(&mut self, subtype_anchor: SubtypeAnchor) {
        self.subtype_anchor
            .insert(subtype_anchor.id, subtype_anchor);
    }

    /// Exhume [`Subtype Anchor`] from the [`ObjectStore`]
    ///
    pub fn exhume_subtype_anchor(&self, id: &Uuid) -> Option<&SubtypeAnchor> {
        self.subtype_anchor.get(id)
    }

    /// Get an iterator over the internal `HashMap<(&Uuid, SubtypeAnchor)>` in the [`ObjectStore`]
    ///
    pub fn iter_subtype_anchor(&self) -> impl Iterator<Item = (&Uuid, &SubtypeAnchor)> {
        self.subtype_anchor.iter()
    }

    /// Inter [`Event`] into the [`ObjectStore`]
    ///
    pub fn inter_event(&mut self, event: Event) {
        self.event.insert(event.id, event);
    }

    /// Exhume [`Event`] from the [`ObjectStore`]
    ///
    pub fn exhume_event(&self, id: &Uuid) -> Option<&Event> {
        self.event.get(id)
    }

    /// Get an iterator over the internal `HashMap<(&Uuid, Event)>` in the [`ObjectStore`]
    ///
    pub fn iter_event(&self) -> impl Iterator<Item = (&Uuid, &Event)> {
        self.event.iter()
    }
}
