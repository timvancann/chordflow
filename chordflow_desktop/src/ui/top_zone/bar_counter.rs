use dioxus::prelude::*;

use crate::ui::app::MetronomeState;

pub fn BarCounter() -> Element {
    let metronome_state: Signal<MetronomeState> = use_context();
    let state = metronome_state.read();
    rsx! {
        div { class: "bar-counter", "Bar {state.current_bar}/{state.bars_per_chord}" }
    }
}
