#![allow(non_snake_case)]

use dioxus::prelude::*;

use crate::ui::reference::{layout::ReferenceState, rows::rows_for_family};

/// One row per scale in the active family: name, formula, and the notes in the
/// chosen key. Clicking a row opens its chords beneath it.
#[component]
pub fn ScaleTable() -> Element {
    let mut reference_state = use_context::<Signal<ReferenceState>>();
    let state = reference_state.read();
    let rows = rows_for_family(state.family, state.root);
    let expanded = state.expanded;
    let root = state.root;
    drop(state);

    rsx! {
        div { class: "scale-table",
            div { class: "scale-table-head",
                span { class: "col-chevron" }
                span { class: "col-name", "scale" }
                span { class: "col-formula", "formula" }
                span { class: "col-notes", "notes in {root}" }
            }
            {
                rows
                    .into_iter()
                    .map(|row| {
                        let scale_type = row.scale_type;
                        let is_open = expanded == Some(scale_type);
                        let open_class = if is_open { "open" } else { "" };
                        rsx! {
                            div { key: "{row.name}", class: "scale-row-group",
                                button {
                                    class: "scale-row {open_class}",
                                    onclick: move |_| reference_state.write().toggle_expanded(scale_type),
                                    span { class: "col-chevron", "\u{203a}" }
                                    span { class: "col-name", "{row.name}" }
                                    span { class: "col-formula mono", "{row.formula}" }
                                    span { class: "col-notes mono", "{row.notes}" }
                                }
                                if is_open {
                                    div { class: "scale-detail",
                                        match (&row.triads, &row.sevenths) {
                                            (Some(triads), Some(sevenths)) => rsx! {
                                                div { class: "detail-line",
                                                    span { class: "reference-label", "triads" }
                                                    span { class: "mono", "{triads}" }
                                                }
                                                div { class: "detail-line",
                                                    span { class: "reference-label", "sevenths" }
                                                    span { class: "mono", "{sevenths}" }
                                                }
                                            },
                                            _ => rsx! {
                                                p { class: "detail-note",
                                                    "{row.name} has {row.notes.split(' ').count()} notes, so it has no diatonic chords. Stacking thirds by scale degree only has an agreed meaning for seven-note scales."
                                                }
                                            },
                                        }
                                    }
                                }
                            }
                        }
                    })
            }
        }
    }
}
