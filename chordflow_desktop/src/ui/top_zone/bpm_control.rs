#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_free_icons::{
    icons::fa_solid_icons::{FaPause, FaPlay},
    Icon,
};

use crate::{
    ui::app::{AppState, MetronomeState},
    AudioCommand, AUDIO_CMD,
};

pub fn BeatControl() -> Element {
    let mut state: Signal<MetronomeState> = use_context();

    rsx! {
        div { class: "bpm-control",
            button {
                class: "btn-icon",
                onclick: move |_| {
                    let _ = AUDIO_CMD.0.try_send(AudioCommand::SetBPM(state.read().bpm - 2));
                    state.write().bpm -= 2;
                },
                "−"
            }
            div { class: "bpm-display",
                span { class: "bpm-value", "{state.read().bpm}" }
                span { class: "bpm-label", "bpm" }
            }
            button {
                class: "btn-icon",
                onclick: move |_| {
                    let _ = AUDIO_CMD.0.try_send(AudioCommand::SetBPM(state.read().bpm + 2));
                    state.write().bpm += 2;
                },
                "+"
            }
        }
    }
}
