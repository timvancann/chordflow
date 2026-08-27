#![allow(non_snake_case)]

use chordflow_music_theory::quality::Notation;
use dioxus::prelude::*;

use crate::ui::app::AppState;

pub fn CurrentChord() -> Element {
    let app_state: Signal<AppState> = use_context();
    let notation = use_context::<Signal<Notation>>();
    let (chord, _) = app_state.read().get_chords(*notation.read());

    rsx! {
        div { class: "current-chord",
            "{chord}"
        }
    }
}
