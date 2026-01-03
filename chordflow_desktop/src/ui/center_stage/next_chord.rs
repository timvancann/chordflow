use dioxus::prelude::*;

use crate::state::practice::PracticeState;

pub fn NextChord() -> Element {
    let practice_state: Signal<PracticeState> = use_context();
    let chord = practice_state.read().next_chord;
    let mut accidendal = "".to_string();
    if chord.root.accidentals > 0 {
        accidendal = "♯".repeat(chord.root.accidentals as usize)
    }
    if chord.root.accidentals < 0 {
        accidendal = "♭".repeat(-chord.root.accidentals as usize)
    };
    rsx! {
        div { class: "next-chord-row",
            div { class: "separator-line separator-left" }
            div { class: "next-chord",
                "{chord.root.letter}"
                span { class: "accidental", "{accidendal}" }
                "{chord.quality}"
            }
            div { class: "separator-line separator-right" }
        }
    }
}
