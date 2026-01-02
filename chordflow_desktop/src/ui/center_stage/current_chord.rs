use chordflow_shared::practice_state::PracticeState;
use dioxus::prelude::*;

pub fn CurrentChord() -> Element {
    let practice_state: Signal<PracticeState> = use_context();
    let chord = practice_state.read().current_chord;
    let mut accidendal = "".to_string();
    if chord.root.accidentals > 0 {
        accidendal = "♯".repeat(chord.root.accidentals as usize)
    }
    if chord.root.accidentals < 0 {
        accidendal = "♭".repeat(-chord.root.accidentals as usize)
    };

    rsx! {
        div { class: "current-chord",
            "{chord.root.letter}"
            span { class: "accidental", "{accidendal}" }
            span { class: "quality", "{chord.quality}" }
        }
    }
}
