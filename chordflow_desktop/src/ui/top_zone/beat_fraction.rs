use dioxus::prelude::*;

use crate::ui::app::MetronomeState;

pub fn BeatFraction() -> Element {
    let metronome_state: Signal<MetronomeState> = use_context();
    let state = metronome_state.read();
    rsx! {
        div { class: "beat-fraction", "{state.current_tick}/{state.ticks_per_bar}" }
    }
}
