#![allow(non_snake_case)]

use chordflow_music_theory::note::practical_keys;
use dioxus::prelude::*;

use crate::ui::reference::layout::ReferenceState;

/// The twelve practical keys, around the circle of fifths. Changing the key
/// respells everything on screen at once.
#[component]
pub fn KeySelector() -> Element {
    let mut reference_state = use_context::<Signal<ReferenceState>>();

    rsx! {
        div { class: "key-selector",
            span { class: "reference-label", "key" }
            div { class: "key-chips",
                {
                    practical_keys()
                        .into_iter()
                        .map(|key| {
                            let active_class = if reference_state.read().root == key { "active" } else { "" };
                            rsx! {
                                button {
                                    key: "{key}",
                                    class: "key-chip {active_class}",
                                    onclick: move |_| {
                                        if reference_state.read().root == key {
                                            return;
                                        }
                                        reference_state.write().select_root(key);
                                    },
                                    "{key}"
                                }
                            }
                        })
                }
            }
        }
    }
}
