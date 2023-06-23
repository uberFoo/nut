//! Macros for navigating the "Associative" domain
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
// {"magic":"","version":"0.2.0"}
// {"magic":"","version":"0.5.0"}
// {"magic":"","version":"1.0.0"}

/// Macro to traverse [`Anchor`][🦀] ➡ [`IsaUi`][🦞], across [`SubtypeAnchor`][🦑] via _R10_
///
/// This macro expects a &[`Anchor`][🦀], and returns a &[`IsaUi`][🦞].
///
/// Generated by `nut::domain::generate_macros::generate_associative_macros`
///
/// [🦀]: crate::associative::types::Anchor
/// [🦞]: crate::associative::types::IsaUi
/// [🦑]: crate::associative::types::SubtypeAnchor
// {"magic":"","kind":{"IgnoreBlockBegin":{}}}
///
/// # Example
///
///```
/// # use test_models::associative::Anchor;
/// # use test_models::associative::IsaUi;
/// # use test_models::associative::SubtypeAnchor;
/// # use test_models::associative_maybe_get_one_iui_across_r10;
/// # let mut store = test_models::associative::ObjectStore::new();
///
/// let anchor_egg = Anchor::new(&mut store, 42);
/// let isa_ui_rob = IsaUi::new(&mut store, 42);
/// let subtype_anchor = SubtypeAnchor::new(&mut store, &anchor_egg, &isa_ui_rob);
///
/// let isa_ui_jgq = associative_maybe_get_one_iui_across_r10!(anchor_egg, store);
/// assert_eq!(Some(&isa_ui_rob), isa_ui_jgq);
///```
// {"magic":"","kind":"IgnoreBlockEnd"}
#[macro_export]
// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"associative_maybe_get_one_iui_across_r10-emit_associative_main"}}}
macro_rules! associative_maybe_get_one_iui_across_r10 {
    ($input:expr, $store:expr) => {{
        // nut::codegen::template::macros::emit_assoc_maybe_get_one
        $store
            .iter_subtype_anchor()
            .filter(|a| a.1.anchor_id == $input.id)
            .next()
            .and_then(|a| $store.exhume_isa_ui(&a.1.isaui_id))
    }};
}
pub use associative_maybe_get_one_iui_across_r10;
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"associative_maybe_get_one_iui_across_r10-emit_associative_main"}}}

/// Macro to traverse [`IsaUi`][🦀] ➡ [`Anchor`][🦞], across [`SubtypeAnchor`][🦑] via _R10_
///
/// This macro expects a &[`IsaUi`][🦀], and returns a &[`Anchor`][🦞].
///
/// Generated by `nut::domain::generate_macros::generate_associative_macros`
///
/// [🦀]: crate::associative::types::IsaUi
/// [🦞]: crate::associative::types::Anchor
/// [🦑]: crate::associative::types::SubtypeAnchor
// {"magic":"","kind":{"IgnoreBlockBegin":{}}}
///
/// # Example
///
///```
/// # use test_models::associative::Anchor;
/// # use test_models::associative::SubtypeAnchor;
/// # use test_models::associative::IsaUi;
/// # use test_models::associative_get_many_anch_across_r10;
/// # let mut store = test_models::associative::ObjectStore::new();
///
/// let anchor_zhb = Anchor::new(&mut store, 42);
/// let isa_ui_ngi = IsaUi::new(&mut store, 42);
/// let subtype_anchor = SubtypeAnchor::new(&mut store, &anchor_zhb, &isa_ui_ngi);
///
/// let anchor_kbw = associative_get_many_anch_across_r10!(isa_ui_ngi, store);
/// assert!(anchor_kbw.iter().find(|&x| **x == anchor_zhb).is_some());
///```
// {"magic":"","kind":"IgnoreBlockEnd"}
#[macro_export]
// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"associative_get_many_anch_across_r10-emit_associative_main"}}}
macro_rules! associative_get_many_anch_across_r10 {
    ($input:expr, $store:expr) => {{
        // nut::codegen::template::macros::emit_assoc_many
        $store
            .iter_subtype_anchor()
            .filter(|a| a.1.isaui_id == $input.id)
            .map(|a| $store.exhume_anchor(&a.1.anchor_id).unwrap())
            .collect::<Vec<&Anchor>>()
    }};
}
pub use associative_get_many_anch_across_r10;
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"associative_get_many_anch_across_r10-emit_associative_main"}}}

/// Macro to traverse [`State`][🦀] ➡ [`Event`][🦞], across [`AcknowledgedEvent`][🦑] via _R20_
///
/// This macro expects a &[`State`][🦀], and returns a &[`Event`][🦞].
///
/// Generated by `nut::domain::generate_macros::generate_associative_macros`
///
/// [🦀]: crate::associative::types::State
/// [🦞]: crate::associative::types::Event
/// [🦑]: crate::associative::types::AcknowledgedEvent
// {"magic":"","kind":{"IgnoreBlockBegin":{}}}
///
/// # Example
///
///```
/// # use test_models::associative::AcknowledgedEvent;
/// # use test_models::associative::State;
/// # use test_models::associative::Event;
/// # use test_models::associative_get_many_e_across_r20;
/// # let mut store = test_models::associative::ObjectStore::new();
///
/// let solid_ants = "hallowed_fear".to_owned();
/// let state_xda = State::new(&mut store, solid_ants);
/// let aberrant_humor = "sassy_stick".to_owned();
/// let event_ijk = Event::new(&mut store, aberrant_humor);
/// let acknowledged_event = AcknowledgedEvent::new(&mut store, &state_xda, &event_ijk);
///
/// let event_ykv = associative_get_many_e_across_r20!(state_xda, store);
/// assert!(event_ykv.iter().find(|&x| **x == event_ijk).is_some());
///```
// {"magic":"","kind":"IgnoreBlockEnd"}
#[macro_export]
// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"associative_get_many_e_across_r20-emit_associative_main"}}}
macro_rules! associative_get_many_e_across_r20 {
    ($input:expr, $store:expr) => {{
        // nut::codegen::template::macros::emit_assoc_many
        $store
            .iter_acknowledged_event()
            .filter(|a| a.1.state_id == $input.id)
            .map(|a| $store.exhume_event(&a.1.event_id).unwrap())
            .collect::<Vec<&Event>>()
    }};
}
pub use associative_get_many_e_across_r20;
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"associative_get_many_e_across_r20-emit_associative_main"}}}

/// Macro to traverse [`Event`][🦀] ➡ [`State`][🦞], across [`AcknowledgedEvent`][🦑] via _R20_
///
/// This macro expects a &[`Event`][🦀], and returns a &[`State`][🦞].
///
/// Generated by `nut::domain::generate_macros::generate_associative_macros`
///
/// [🦀]: crate::associative::types::Event
/// [🦞]: crate::associative::types::State
/// [🦑]: crate::associative::types::AcknowledgedEvent
// {"magic":"","kind":{"IgnoreBlockBegin":{}}}
///
/// # Example
///
///```
/// # use test_models::associative::Event;
/// # use test_models::associative::State;
/// # use test_models::associative::AcknowledgedEvent;
/// # use test_models::associative_get_many_s_across_r20;
/// # let mut store = test_models::associative::ObjectStore::new();
///
/// let untidy_support = "unequal_song".to_owned();
/// let state_udq = State::new(&mut store, untidy_support);
/// let impolite_invention = "agreeable_history".to_owned();
/// let event_elq = Event::new(&mut store, impolite_invention);
/// let acknowledged_event = AcknowledgedEvent::new(&mut store, &state_udq, &event_elq);
///
/// let state_sot = associative_get_many_s_across_r20!(event_elq, store);
/// assert!(state_sot.iter().find(|&x| **x == state_udq).is_some());
///```
// {"magic":"","kind":"IgnoreBlockEnd"}
#[macro_export]
// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"associative_get_many_s_across_r20-emit_associative_main"}}}
macro_rules! associative_get_many_s_across_r20 {
    ($input:expr, $store:expr) => {{
        // nut::codegen::template::macros::emit_assoc_many
        $store
            .iter_acknowledged_event()
            .filter(|a| a.1.event_id == $input.id)
            .map(|a| $store.exhume_state(&a.1.state_id).unwrap())
            .collect::<Vec<&State>>()
    }};
}
pub use associative_get_many_s_across_r20;
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"associative_get_many_s_across_r20-emit_associative_main"}}}