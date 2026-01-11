#![allow(non_snake_case)]

use dioxus::prelude::*;

use crate::ui::app::AppState;

pub fn NextChord() -> Element {
    let app_state: Signal<AppState> = use_context();
    let (_, chord) = app_state.read().get_chords();

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
