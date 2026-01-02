use dioxus::prelude::*;

use crate::ui::app::MetronomeState;

pub fn BeatViz() -> Element {
    let metronome_state: Signal<MetronomeState> = use_context();
    let state = metronome_state.read();
    rsx! {
        div { class: "beat-viz",
            {
                (1..=state.ticks_per_bar)

                    .map(|i| {
                        let is_active = i <= state.current_tick;
                        let is_current = i == state.current_tick;
                        let active_class = if is_active { "active" } else { "" };
                        let current_class = if is_current { "current" } else { "" };
                        rsx! {
                            div { class: "beat-block {active_class} {current_class}",
                                if is_current {
                                    div { class: "beat-block-inner-border" }
                                }
                            }
                        }
                    })
            }
        }
    }
}
