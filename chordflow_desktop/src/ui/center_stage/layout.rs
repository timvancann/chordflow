#![allow(non_snake_case)]

use dioxus::prelude::*;

use crate::ui::center_stage::{current_chord::CurrentChord, next_chord::NextChord};

pub fn CenterStage() -> Element {
    rsx! {
        div { class: "center-stage",
            div { class: "chord-container",
                CurrentChord {}
                NextChord {}
            }
        }
    }
}
