#![allow(non_snake_case)]

use chordflow_music_theory::quality::Notation;
use dioxus::prelude::*;

use crate::ui::app::AppState;

pub fn NextChord() -> Element {
    let app_state: Signal<AppState> = use_context();
    let notation = use_context::<Signal<Notation>>();
    let (_, chord) = app_state.read().get_chords(*notation.read());

    rsx! {
        div { class: "next-chord-row",
            div { class: "separator-line separator-left" }
            div { class: "next-chord",
                "{chord}"
            }
            div { class: "separator-line separator-right" }
        }
    }
}
