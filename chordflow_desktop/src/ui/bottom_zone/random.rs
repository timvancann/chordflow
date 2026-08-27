#![allow(non_snake_case)]

use chordflow_music_theory::{
    chord::Chord,
    note::{practical_keys, Note, NoteLetter},
    quality::{Notation, Quality},
};
use dioxus::prelude::*;
use strum::IntoEnumIterator;

use crate::ui::app::AppState;

/// Pick which roots and which chord qualities the random drill draws from.
///
/// Both are multi-select, unlike Circle of Fourths' single-quality dropdown:
/// the point of this mode is narrowing the pool to whatever you are working
/// on, e.g. only dominant and half-diminished sevenths across every key.
///
/// The last chip in either row cannot be turned off. An empty pool has nothing
/// to draw from, so the UI must not be able to create one.
#[component]
pub fn RandomSelector() -> Element {
    let mut app_state = use_context::<Signal<AppState>>();
    let notation = use_context::<Signal<Notation>>();
    let notation = *notation.read();

    rsx! {
        div { class: "random-selector",
            div { class: "random-row",
                span { class: "label-small", "Keys" }
                div { class: "chip-row",
                    {
                        practical_keys()
                            .into_iter()
                            .map(|root| {
                                let selected = app_state.read().random_config.roots.contains(&root);
                                let active_class = if selected { "active" } else { "" };
                                rsx! {
                                    button {
                                        key: "{root}",
                                        class: "key-chip {active_class}",
                                        onclick: move |_| app_state.write().random_config.toggle_root(root),
                                        "{root}"
                                    }
                                }
                            })
                    }
                }
            }
            div { class: "random-row",
                span { class: "label-small", "Qualities" }
                div { class: "chip-row",
                    {
                        Quality::iter()
                            .map(|quality| {
                                let selected = app_state
                                    .read()
                                    .random_config
                                    .qualities
                                    .contains(&quality);
                                let active_class = if selected { "active" } else { "" };
                                // The symbol alone is cryptic and the full name
                                // is long, so show a worked example: "C-7".
                                let example =
                                    Chord::new(Note::new(NoteLetter::C, 0), quality).symbol(notation);
                                rsx! {
                                    button {
                                        key: "{quality:?}",
                                        class: "key-chip {active_class}",
                                        title: "{quality.name()}",
                                        onclick: move |_| {
                                            app_state.write().random_config.toggle_quality(quality)
                                        },
                                        "{example}"
                                    }
                                }
                            })
                    }
                }
            }
        }
    }
}
