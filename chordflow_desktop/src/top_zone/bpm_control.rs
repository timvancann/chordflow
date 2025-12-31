
use chordflow_shared::metronome::{MetronomeCommand, METRONOME_COMMAND_CHANNEL};
use dioxus::prelude::*;

use crate::MetronomeState;

pub fn BeatControl() -> Element {
    let mut metronome_state : Signal<MetronomeState>= use_context();
    let state = metronome_state.read();

    rsx! {
        div { class: "bpm-control",
            button {
                class: "btn-icon",
                onclick: move |_| {
                    metronome_state.write().bpm -= 2;
                    let _ = METRONOME_COMMAND_CHANNEL.0.try_send(MetronomeCommand::DecreaseBpm(2));
                },
                "−"
            }
            div { class: "bpm-display",
                span { class: "bpm-value", "{state.bpm}" }
                span { class: "bpm-label", "bpm" }
            }
            button {
                class: "btn-icon",
                onclick: move |_| {
                    metronome_state.write().bpm += 2;
                    let _ = METRONOME_COMMAND_CHANNEL.0.try_send(MetronomeCommand::IncreaseBpm(2));
                },
                "+"
            }
        }
    }
}
