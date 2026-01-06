use dioxus::prelude::*;

use crate::{ui::app::MetronomeState, AudioCommand, AUDIO_CMD};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Subdivision {
    #[default]
    None,
    Eighth,
    Triplet,
    Sixteenth,
}

impl Subdivision {
    pub fn subdivisions_per_beat(&self) -> u8 {
        match self {
            Subdivision::None => 1,
            Subdivision::Eighth => 2,
            Subdivision::Triplet => 3,
            Subdivision::Sixteenth => 4,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Subdivision::None => "None",
            Subdivision::Eighth => "1/8",
            Subdivision::Triplet => "Triplet",
            Subdivision::Sixteenth => "1/16",
        }
    }

    pub fn all() -> &'static [Subdivision] {
        &[
            Subdivision::None,
            Subdivision::Eighth,
            Subdivision::Triplet,
            Subdivision::Sixteenth,
        ]
    }
}

pub fn SubdivisionSelector() -> Element {
    let mut metronome_state: Signal<MetronomeState> = use_context();

    rsx! {
        div { class: "subdivision-control",
            select {
                class: "subdivision-select",
                value: "{metronome_state.read().subdivision.label()}",
                onchange: move |e| {
                    let subdivision = match e.value().as_str() {
                        "None" => Subdivision::None,
                        "1/8" => Subdivision::Eighth,
                        "Triplet" => Subdivision::Triplet,
                        "1/16" => Subdivision::Sixteenth,
                        _ => Subdivision::None,
                    };
                    // Reset UI state when changing subdivision
                    metronome_state.write().current_bar = 1;
                    metronome_state.write().current_tick = 0;
                    metronome_state.write().subdivision = subdivision;
                    let _ = AUDIO_CMD.0.try_send(AudioCommand::SetSubdivision(
                        subdivision.subdivisions_per_beat(),
                    ));
                },
                for subdivision in Subdivision::all() {
                    option {
                        value: "{subdivision.label()}",
                        selected: *subdivision == metronome_state.read().subdivision,
                        "{subdivision.label()}"
                    }
                }
            }
        }
    }
}
