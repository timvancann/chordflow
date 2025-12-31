
use dioxus::prelude::*;

use crate::MetronomeState;

pub fn BeatFraction() -> Element {
    let metronome_state : Signal<MetronomeState>= use_context();
    let state = metronome_state.read();
    rsx! {
        div { class: "beat-fraction", "{state.current_tick + 1}/{state.ticks_per_bar}" }
    }
}
