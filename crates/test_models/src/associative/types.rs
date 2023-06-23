//! Types for instances of the "Associative" domain
//! # Domain Description
//!
//! Domain to test Associative Objects/Relationships
//!
//!
//! # Contents
//!
//! The following types are defined herein:
//!    * [`AcknowledgedEvent`]
//!    * [`Anchor`]
//!    * [`Event`]
//!    * [`IsaUi`]
//!    * [`State`]
//!    * [`SubtypeAnchor`]
//!
//! # Generated Code -- edit _with care_.
//!
//! Don't mess with anything between `{"magic":"","kind":"CriticalBlockBegin"}`
//! and `{"magic":"","kind":"CriticalBlockEnd"}`. Otherwise, you should be free
//! to go wild. Happy hacking!
//!
//! Use the following invocation to reproduce:
// {"magic":"","kind":{"IgnoreBlockBegin":{}}}
//! ```shell
//!  /Users/uberfoo/projects/sarzak/nut/target/debug/deps/generate_test_domain-145fdb9ab1f4b4be --nocapture
//! ```
// {"magic":"","kind":"IgnoreBlockEnd"}
// {"magic":"","version":"0.5.0"}
// {"magic":"","version":"1.0.0"}
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"imports"}}}
use crate::associative::store::ObjectStore;
use crate::associative::UUID_NS;
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"imports"}}}

/// An Event that Does Something
///
/// An acknowledged event is an event that a [`State`] knows how to handle.
///
// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"acknowledged_event-struct-definition"}}}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AcknowledgedEvent {
    /// pub id: `Uuid`,
    ///
    pub id: Uuid,
    /// pub event_id: `Event`,
    ///
    pub event_id: Uuid,
    /// pub state_id: `State`,
    ///
    pub state_id: Uuid,
}
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"acknowledged_event-struct-definition"}}}

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"acknowledged_event-new_impl"}}}
impl AcknowledgedEvent {
    //     // {"magic":"","kind":{"CriticalBlockBegin":{"tag":"acknowledged_event-new_impl"}}} //⚡️
    /// Inter a new AcknowledgedEvent and return it's `id`
    ///
    //     // {"magic":"","kind":{"IgnoreBlockBegin":{}}} //⚡️
    // {"magic":"","kind":{"IgnoreBlockBegin":{}}}
    /// # Example
    ///
    ///```
    /// # use test_models::associative::AcknowledgedEvent;
    /// # use test_models::associative::State;
    /// # use test_models::associative::Event;
    /// # let mut store = test_models::associative::ObjectStore::new();
    ///
    /// let accessible_talk = "unadvised_duck".to_owned();
    /// let state_gqk = State::new(&mut store, accessible_talk);
    /// let aboard_paint = "wide_eyed_mint".to_owned();
    /// let event_nyf = Event::new(&mut store, aboard_paint);
    ///
    /// let acknowledged_event = AcknowledgedEvent::new(&mut store, &state_gqk, &event_nyf);
    ///```
    // {"magic":"","kind":"IgnoreBlockEnd"}
    pub fn new(store: &mut ObjectStore, state_id: &State, event_id: &Event) -> Self {
        let id = Uuid::new_v5(
            &UUID_NS,
            format!("{:?}::{:?}::", state_id, event_id,).as_bytes(),
        );
        let new = Self {
            id,
            state_id: state_id.id,
            event_id: event_id.id,
        };

        store.inter_acknowledged_event(new.clone());

        new
    }
    // {"magic":"","kind":{"CriticalBlockEnd":{"tag":"acknowledged_event-new_impl"}}}
}

/// An anchor, or anchor point, is the location where an arrow from a relationship attached
/// to an object.
///
/// Rather than storing the `x` and `y` coordinates of where the anchor attaches, we are related
/// to an [Edge], which is related to a box, which is related to the [Object] to which we are
/// attached. This of course completes the circuit from the [Relationship] for which we are
/// drawing the lines in the first place.
///
/// Anchor also contains a direction, so that we know the orientation to draw the arrows. Finally
///, there is an offset, which is a point that describes the offset from the anchor for the
/// relationship phrase.
///
// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"anchor-struct-definition"}}}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Anchor {
    /// pub id: `Uuid`,
    ///
    pub id: Uuid,
    /// pub number: `i64`,
    ///
    pub number: i64,
}
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"anchor-struct-definition"}}}

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"anchor-new_impl"}}}
impl Anchor {
    //     // {"magic":"","kind":{"CriticalBlockBegin":{"tag":"anchor-new_impl"}}} //⚡️
    /// Inter a new Anchor and return it's `id`
    ///
    //     // {"magic":"","kind":{"IgnoreBlockBegin":{}}} //⚡️
    // {"magic":"","kind":{"IgnoreBlockBegin":{}}}
    /// # Example
    ///
    ///```
    /// # use test_models::associative::Anchor;
    /// # let mut store = test_models::associative::ObjectStore::new();
    ///
    ///
    /// let anchor = Anchor::new(&mut store, 42);
    ///```
    // {"magic":"","kind":"IgnoreBlockEnd"}
    pub fn new(store: &mut ObjectStore, number: i64) -> Self {
        let id = Uuid::new_v5(&UUID_NS, format!("{}::", number,).as_bytes());
        let new = Self { id, number };

        store.inter_anchor(new.clone());

        new
    }
    // {"magic":"","kind":{"CriticalBlockEnd":{"tag":"anchor-new_impl"}}}
}

/// An event is sent to an object, and processed by the current state. Assuming it accepts the
/// event. Otherwise it’s dropped on the floor.
///
// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"event-struct-definition"}}}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Event {
    /// pub id: `Uuid`,
    ///
    pub id: Uuid,
    /// pub name: `std::string::String`,
    ///
    pub name: std::string::String,
}
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"event-struct-definition"}}}

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"event-new_impl"}}}
impl Event {
    //     // {"magic":"","kind":{"CriticalBlockBegin":{"tag":"event-new_impl"}}} //⚡️
    /// Inter a new Event and return it's `id`
    ///
    //     // {"magic":"","kind":{"IgnoreBlockBegin":{}}} //⚡️
    // {"magic":"","kind":{"IgnoreBlockBegin":{}}}
    /// # Example
    ///
    ///```
    /// # use test_models::associative::Event;
    /// # let mut store = test_models::associative::ObjectStore::new();
    ///
    /// let beneficial_channel = "handsomely_son".to_owned();
    ///
    /// let event = Event::new(&mut store, beneficial_channel);
    ///```
    // {"magic":"","kind":"IgnoreBlockEnd"}
    pub fn new(store: &mut ObjectStore, name: std::string::String) -> Self {
        let id = Uuid::new_v5(&UUID_NS, format!("{}::", name,).as_bytes());
        let new = Self { id, name };

        store.inter_event(new.clone());

        new
    }
    // {"magic":"","kind":{"CriticalBlockEnd":{"tag":"event-new_impl"}}}
}

/// This represents additional data necessary to render an `Isa` relationship in the user interface
///.
///
// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"isa_ui-struct-definition"}}}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct IsaUi {
    /// pub id: `Uuid`,
    ///
    pub id: Uuid,
    /// pub number: `i64`,
    ///
    pub number: i64,
}
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"isa_ui-struct-definition"}}}

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"isa_ui-new_impl"}}}
impl IsaUi {
    //     // {"magic":"","kind":{"CriticalBlockBegin":{"tag":"isa_ui-new_impl"}}} //⚡️
    /// Inter a new IsaUi and return it's `id`
    ///
    //     // {"magic":"","kind":{"IgnoreBlockBegin":{}}} //⚡️
    // {"magic":"","kind":{"IgnoreBlockBegin":{}}}
    /// # Example
    ///
    ///```
    /// # use test_models::associative::IsaUi;
    /// # let mut store = test_models::associative::ObjectStore::new();
    ///
    ///
    /// let isa_ui = IsaUi::new(&mut store, 42);
    ///```
    // {"magic":"","kind":"IgnoreBlockEnd"}
    pub fn new(store: &mut ObjectStore, number: i64) -> Self {
        let id = Uuid::new_v5(&UUID_NS, format!("{}::", number,).as_bytes());
        let new = Self { id, number };

        store.inter_isa_ui(new.clone());

        new
    }
    // {"magic":"","kind":{"CriticalBlockEnd":{"tag":"isa_ui-new_impl"}}}
}

/// An [Object] state, more precisely, a set of states, is where all the action happens.
///
// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"state-struct-definition"}}}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct State {
    /// pub id: `Uuid`,
    ///
    pub id: Uuid,
    /// pub name: `std::string::String`,
    ///
    pub name: std::string::String,
}
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"state-struct-definition"}}}

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"state-new_impl"}}}
impl State {
    //     // {"magic":"","kind":{"CriticalBlockBegin":{"tag":"state-new_impl"}}} //⚡️
    /// Inter a new State and return it's `id`
    ///
    //     // {"magic":"","kind":{"IgnoreBlockBegin":{}}} //⚡️
    // {"magic":"","kind":{"IgnoreBlockBegin":{}}}
    /// # Example
    ///
    ///```
    /// # use test_models::associative::State;
    /// # let mut store = test_models::associative::ObjectStore::new();
    ///
    /// let bloody_toothpaste = "distinct_whistle".to_owned();
    ///
    /// let state = State::new(&mut store, bloody_toothpaste);
    ///```
    // {"magic":"","kind":"IgnoreBlockEnd"}
    pub fn new(store: &mut ObjectStore, name: std::string::String) -> Self {
        let id = Uuid::new_v5(&UUID_NS, format!("{}::", name,).as_bytes());
        let new = Self { id, name };

        store.inter_state(new.clone());

        new
    }
    // {"magic":"","kind":{"CriticalBlockEnd":{"tag":"state-new_impl"}}}
}

/// Subtype Anchor
///
/// Just as it sounds, these are [`Anchor`]s used by [`Subtype`]s in an [`Isa`] relationship
///.
///
// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"subtype_anchor-struct-definition"}}}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct SubtypeAnchor {
    /// pub id: `Uuid`,
    ///
    pub id: Uuid,
    /// pub anchor_id: `Anchor`,
    ///
    pub anchor_id: Uuid,
    /// pub isaui_id: `IsaUI`,
    ///
    pub isaui_id: Uuid,
}
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"subtype_anchor-struct-definition"}}}

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"subtype_anchor-new_impl"}}}
impl SubtypeAnchor {
    //     // {"magic":"","kind":{"CriticalBlockBegin":{"tag":"subtype_anchor-new_impl"}}} //⚡️
    /// Inter a new SubtypeAnchor and return it's `id`
    ///
    //     // {"magic":"","kind":{"IgnoreBlockBegin":{}}} //⚡️
    // {"magic":"","kind":{"IgnoreBlockBegin":{}}}
    /// # Example
    ///
    ///```
    /// # use test_models::associative::IsaUi;
    /// # use test_models::associative::SubtypeAnchor;
    /// # use test_models::associative::Anchor;
    /// # let mut store = test_models::associative::ObjectStore::new();
    ///
    /// let anchor_amw = Anchor::new(&mut store, 42);
    /// let isa_ui_zje = IsaUi::new(&mut store, 42);
    ///
    /// let subtype_anchor = SubtypeAnchor::new(&mut store, &anchor_amw, &isa_ui_zje);
    ///```
    // {"magic":"","kind":"IgnoreBlockEnd"}
    pub fn new(store: &mut ObjectStore, anchor_id: &Anchor, isaui_id: &IsaUi) -> Self {
        let id = Uuid::new_v5(
            &UUID_NS,
            format!("{:?}::{:?}::", anchor_id, isaui_id,).as_bytes(),
        );
        let new = Self {
            id,
            anchor_id: anchor_id.id,
            isaui_id: isaui_id.id,
        };

        store.inter_subtype_anchor(new.clone());

        new
    }
    // {"magic":"","kind":{"CriticalBlockEnd":{"tag":"subtype_anchor-new_impl"}}}
}
