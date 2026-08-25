#![allow(non_snake_case)]

use chordflow_music_theory::scale::ScaleFamily;
use dioxus::prelude::*;
use strum::IntoEnumIterator;

use crate::ui::reference::layout::ReferenceState;

/// The poster's four groupings. One family is shown at a time.
#[component]
pub fn FamilyTabs() -> Element {
    let mut reference_state = use_context::<Signal<ReferenceState>>();

    rsx! {
        div { class: "family-tabs",
            {
                ScaleFamily::iter()
                    .map(|family| {
                        let active_class = if reference_state.read().family == family { "active" } else { "" };
                        rsx! {
                            button {
                                key: "{family}",
                                class: "family-tab {active_class}",
                                onclick: move |_| {
                                    if reference_state.read().family == family {
                                        return;
                                    }
                                    reference_state.write().select_family(family);
                                },
                                "{family}"
                            }
                        }
                    })
            }
        }
    }
}
