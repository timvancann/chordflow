use dioxus::prelude::*;

use crate::MetronomeState;

#[component]
pub fn MetronomeDisplay() -> Element {
    let metronome_state: Signal<MetronomeState> = use_context();

    let m = metronome_state.read();
    rsx! {
        div { class: "space-y-4",
            div { class: "flex justify-center gap-2",
                for bar in 0..metronome_state.read().bars_per_chord {
                    if bar > 0 {
                        span { class: "text-tokyoNight-orange", " | " }
                    }
                    for tick in 0..m.ticks_per_bar {
                        if bar < m.current_bar || (bar == m.current_bar && tick < m.current_tick) {
                            div { class: "w-8 h-8 rounded-full transition-colors bg-tokyoNight-blue" }
                        } else if bar < m.current_bar || (bar == m.current_bar && tick == m.current_tick) {
                            div { class: "w-8 h-8 rounded-full transition-colors bg-tokyoNight-orange" }
                        } else {
                            div { class: "w-8 h-8 rounded-full transition-colors bg-tokyoNight-fg_dark" }
                        }
                    }
                }
            }
        }
    }
}
