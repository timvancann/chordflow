use dioxus::prelude::*;

use crate::center_stage::{current_chord::CurrentChord, next_chord::NextChord};

pub fn CenterStage() -> Element {
    rsx!{
        div { class: "center-stage",
            div { class: "chord-container",
                CurrentChord {}
                NextChord {}
            }
        }
    }
}
