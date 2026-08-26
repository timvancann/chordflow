#![allow(non_snake_case)]

use dioxus::prelude::*;

use crate::ui::reference::{
    chord_popup::SelectedChord, layout::ReferenceState, rows::rows_for_family,
};

/// One row per scale in the active family: name, formula, and the notes in the
/// chosen key. Clicking a row opens its diatonic sevenths beneath it, and
/// clicking one of those opens the chord popup.
#[component]
pub fn ScaleTable() -> Element {
    let mut reference_state = use_context::<Signal<ReferenceState>>();
    let mut selected_chord = use_context::<Signal<SelectedChord>>();

    let state = reference_state.read();
    let rows = rows_for_family(state.family, state.root);
    let selected = state.selected;
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
                        let is_open = selected == scale_type;
                        let open_class = if is_open { "open" } else { "" };
                        let note_count = row.notes.split(' ').count();
                        rsx! {
                            div { key: "{row.name}", class: "scale-row-group",
                                button {
                                    class: "scale-row {open_class}",
                                    onclick: move |_| reference_state.write().select_scale(scale_type),
                                    span { class: "col-chevron", "\u{203a}" }
                                    span { class: "col-name", "{row.name}" }
                                    span { class: "col-formula mono", "{row.formula}" }
                                    span { class: "col-notes mono", "{row.notes}" }
                                }
                                if is_open {
                                    div { class: "scale-detail",
                                        match &row.sevenths {
                                            Some(sevenths) => rsx! {
                                                div { class: "detail-line",
                                                    span { class: "reference-label", "sevenths" }
                                                    div { class: "chord-buttons",
                                                        {
                                                            sevenths
                                                                .iter()
                                                                .copied()
                                                                .map(|chord| rsx! {
                                                                    button {
                                                                        key: "{chord}",
                                                                        class: "chord-button mono",
                                                                        onclick: move |_| selected_chord.write().select(chord),
                                                                        "{chord}"
                                                                    }
                                                                })
                                                        }
                                                    }
                                                }
                                            },
                                            None => rsx! {
                                                p { class: "detail-note",
                                                    "{row.name} has {note_count} notes, so it has no diatonic chords. Stacking thirds by scale degree only has an agreed meaning for seven-note scales."
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
