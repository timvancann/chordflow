use chordflow_shared::practice_state::{self, PracticState};

use dioxus::prelude::*;

#[component]
pub fn PracticeStateDisplay() -> Element {
    let practice_state: Signal<PracticState> = use_context();
    let chord = practice_state.read().current_chord;
    let next_chord = practice_state.read().next_chord;
    rsx! {
        div { class: "flex flex-col items-center justify-center w-full space-y-1 mt-8",
            div { class: "flex text-8xl text-tokyoNight-magenta",
                div { class: "font-semibold", {chord.root.to_string()} }
                div { class: "text-5xl", {chord.quality.to_string()} }
            }
            div { class: "flex text-4xl text-tokyoNight-blue font-sans",
                div { class: "font-semibold", {next_chord.root.to_string()} }
                div { class: "text-2xl", {next_chord.quality.to_string()} }
            }
        }
    }
}
