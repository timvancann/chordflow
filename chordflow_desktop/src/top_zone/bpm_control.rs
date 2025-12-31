
use dioxus::prelude::*;

use crate::MetronomeState;

pub fn BeatControl() -> Element {
    let metronome_state : Signal<MetronomeState>= use_context();
    let state = metronome_state.read();

    rsx! {
        div { class: "bpm-control",
            button { class: "btn-icon", "−" }
            div { class: "bpm-display",
                span { class: "bpm-value", "{state.bpm}" }
                span { class: "bpm-label", "bpm" }
            }
            button { class: "btn-icon", "+" }
        }
    }
}
