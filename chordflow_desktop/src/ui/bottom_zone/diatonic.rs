#![allow(non_snake_case)]

use chordflow_music_theory::{note::practical_keys, scale::ParentScale};
use dioxus::prelude::*;
use strum::IntoEnumIterator;

use crate::ui::app::AppState;

pub fn DiatonicSelector() -> Element {
    let mut config_state = use_context::<Signal<AppState>>();

    rsx! {
        div { class: "control-group-right",
            // Which parent scale's degrees to walk. Picking harmonic or
            // melodic minor drills all seven of that parent's modes rather
            // than the major scale's.
            span { class: "label-small", "Key" }
            select {
                class: "select-styled",
                onchange: move |e| {
                    if let Some(parent) = ParentScale::iter()
                        .nth(e.value().parse::<usize>().unwrap_or(0))
                    {
                        config_state.write().diatonic_config.set_parent(parent);
                    }
                },
                for (i, parent) in ParentScale::iter().enumerate() {
                    option {
                        value: "{i}",
                        selected: parent == config_state.read().diatonic_config.parent,
                        "{parent}"
                    }
                }
            }

            // Root note. The twelve practical keys, not all seventeen
            // spellings: nobody practises in A♯.
            span { class: "label-small", "Root" }
            select {
                class: "select-styled",
                onchange: move |e| {
                    if let Some(root) = practical_keys().get(e.value().parse::<usize>().unwrap_or(0))
                    {
                        config_state.write().diatonic_config.set_root(*root);
                    }
                },
                for (i, root) in practical_keys().into_iter().enumerate() {
                    option {
                        value: "{i}",
                        selected: root == config_state.read().diatonic_config.scale.root,
                        "{root}"
                    }
                }
            }

            // Sevenths instead of triads
            span { class: "label-small", "7ths" }
            input {
                r#type: "checkbox",
                checked: config_state.read().diatonic_config.use_sevenths,
                onchange: move |e| {
                    let on = e.value().parse::<bool>().unwrap_or(false);
                    config_state.write().diatonic_config.set_use_sevenths(on);
                }
            }

            // Random mode checkbox
            span { class: "label-small", "Random" }
            input {
                r#type: "checkbox",
                checked: config_state.read().diatonic_config.is_random,
                onchange: move |e| {
                    config_state.write().diatonic_config.is_random = e.value().parse::<bool>().unwrap_or(false);
                }
            }
        }
    }
}
