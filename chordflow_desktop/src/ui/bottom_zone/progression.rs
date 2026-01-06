use dioxus::prelude::*;

use crate::{
    state::progression::ProgressionChord,
    ui::app::{AppState, MetronomeState},
};

pub fn ProgressionSelector() -> Element {
    let mut app_state = use_context::<Signal<AppState>>();
    let mut metronome_state = use_context::<Signal<MetronomeState>>();
    let mut input_value = use_signal(String::new);
    let mut parse_error = use_signal(|| Option::<String>::None);

    let mut parse_progression = move || {
        let input = input_value.read().clone();
        match ProgressionChord::from_string(input) {
            Ok(chords) => {
                app_state.write().progression_config.chords = chords;
                app_state.write().progression_config.reset();
                metronome_state.write().bars_per_chord = app_state
                    .read()
                    .progression_config
                    .get_bars_per_cycle_current();
                app_state.write().restart();
                parse_error.set(None);
            }
            Err(e) => {
                parse_error.set(Some(format!("Parse error: {}", e)));
            }
        }
    };

    rsx! {
        div { class: "progression-container",
            // Input section
            div { class: "progression-input-wrapper",
                input {
                    class: "progression-input",
                    r#type: "text",
                    placeholder: "Fm7, G#dim",
                    value: "{input_value}",
                    oninput: move |e| input_value.set(e.value()),
                    onkeydown: move |e| {
                        if e.key() == Key::Enter {
                            parse_progression();
                        }
                    }
                }
                button {
                    class: "btn-parse-inline",
                    onclick: move |_| parse_progression(),
                    "↵"
                }
            }

            // Error message
            if let Some(error) = parse_error.read().as_ref() {
                div { class: "parse-error", "{error}" }
            }

            // Parsed chords display
            if !app_state.read().progression_config.chords.is_empty() {
                div { class: "progression-chords",
                    for (index, progression_chord) in app_state.read().progression_config.chords.iter().enumerate() {
                        div { class: "chord-card",
                            class: if index == app_state.read().progression_config.current_chord_index { "active" } else { "" },
                            div { class: "chord-name", "{progression_chord.chord.origin}" }
                            div { class: "bars-control",
                                button {
                                    class: "btn-icon btn-small",
                                    onclick: move |_| app_state.write().progression_config.decrements_bars(index),
                                    "−"
                                }
                                span { class: "bars-value", "{progression_chord.bars} b" }
                                button {
                                    class: "btn-icon btn-small",
                                    onclick: move |_| app_state.write().progression_config.increments_bars(index),
                                    "+"
                                }
                            }
                        }
                        if index < app_state.read().progression_config.chords.len() - 1 {
                            div { class: "chord-arrow", "→" }
                        }
                    }
                }
            }
        }
    }
}
