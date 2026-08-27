#![allow(non_snake_case)]

use dioxus::prelude::*;

use crate::ui::app::MetronomeState;

/// Whether to play a bar of clicks before the first chord.
///
/// Lives in the top zone with the rest of the metronome settings — tempo,
/// subdivision, bar count — rather than beside the mode controls, which is
/// where it used to sit.
#[component]
pub fn CountIn() -> Element {
    let mut metronome_state: Signal<MetronomeState> = use_context();

    rsx! {
        label { class: "count-in",
            input {
                r#type: "checkbox",
                checked: metronome_state.read().count_in_enabled,
                onchange: move |evt| {
                    metronome_state.write().count_in_enabled = evt.checked();
                }
            }
            span { class: "label-small", "Count-in" }
        }
    }
}
