use std::{
    fmt::Display,
    sync::mpsc::{Receiver, Sender},
    time::Instant,
};

use chordflow_audio::audio::setup_audio;
use chordflow_shared::{
    metronome::{setup_metronome, MetronomeCommand, MetronomeEvent},
    practice_state::{ConfigState, PracticState},
    ModeOption,
};
use components::{
    config_state::ConfigStateDisplay,
    header::Header,
    metronome::MetronomeDisplay,
    metronome_settings::MetronomSettingsDisplay,
    mode_selection::ModeSelectionDisplay,
    play_controls::{restart, PlayControls},
    practice_state::PracticeStateDisplay,
};
use dioxus::{
    desktop::{tao::platform::macos::WindowBuilderExtMacOS, Config, LogicalSize, WindowBuilder},
    prelude::*,
};
use hooks::use_metronome::use_metronome;

mod components;
mod hooks;
mod top_zone;

use crate::top_zone::layout::TopZone;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    let window_builder = WindowBuilder::new()
        .with_transparent(true)
        .with_decorations(true)
        .with_focused(true)
        .with_resizable(true)
        .with_title("ChordFlow")
        .with_has_shadow(true)
        .with_movable_by_window_background(true)
        .with_inner_size(LogicalSize {
            height: 910,
            width: 1000,
        })
        .with_always_on_top(false);

    let config = Config::default().with_window(window_builder);

    dioxus::LaunchBuilder::new().with_cfg(config).launch(App);
}

type MetronomeSignal = Signal<(Sender<MetronomeCommand>, Receiver<MetronomeEvent>)>;

#[derive(PartialEq, Clone, Copy)]
struct MetronomeState {
    bars_per_chord: usize,
    ticks_per_bar: usize,
    bpm: usize,
    current_bar: usize,
    current_tick: usize,
}

impl Display for MetronomeState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "BPM: {}, Bar: {}/{} Tick: {}/{}",
            self.bpm, self.current_bar, self.bars_per_chord, self.current_tick, self.ticks_per_bar
        )
    }
}

#[component]
fn App() -> Element {
    let audio_tx = use_signal(|| setup_audio(None));
    let metronome = use_signal(|| setup_metronome(100, 2, 4, Instant::now));
    let practice_state = use_signal(PracticState::default);
    let selected_mode = use_signal(|| ModeOption::Fourths);
    let config_state = use_signal(ConfigState::default);
    let metronome_state = use_signal(|| MetronomeState {
        bars_per_chord: 2,
        ticks_per_bar: 4,
        bpm: 100,
        current_bar: 0,
        current_tick: 0,
    });

    use_context_provider(|| audio_tx);
    use_context_provider(|| metronome);
    use_context_provider(|| practice_state);
    use_context_provider(|| selected_mode);
    use_context_provider(|| config_state);
    use_context_provider(|| metronome_state);

    use_metronome(metronome, metronome_state, practice_state, audio_tx);

    let mut initial_setup = use_signal(|| true);

    use_effect(move || {
        if !*initial_setup.read() {
            return;
        }
        restart();
        initial_setup.set(false);
    });

    rsx! {
        // Global app resources
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        div { class: "app-container",
            // Ambient glow background
            div { class: "ambient-bg" }

            // Fixed metronome zone - always top
            TopZone {}
            // Fixed center stage - always middle
            div { class: "center-stage",
                div { class: "chord-container",
                    // Current chord
                    div { class: "current-chord",
                        "F"
                        span { class: "accidental", "♯" }
                        span { class: "quality", "m7" }
                    }

                    // Next chord indicator
                    div { class: "next-chord-row",
                        div { class: "separator-line separator-left" }
                        div { class: "next-chord",
                            "Cmaj7"
                            span { class: "accidental", "♯" }
                            "9"
                        }
                        div { class: "separator-line separator-right" }
                    }
                }
            }

            // Fixed control zone - always bottom
            div { class: "bottom-zone",
                div { class: "zone-content",
                    // Left: Mode selector
                    div { class: "control-group-left",
                        span { class: "label-small", "Mode" }
                        select { class: "select-styled",
                            option { "Circle of Fourths" }
                            option { "Diatonic Progression" }
                            option { "Random Chords" }
                        }
                    }

                    // Center: Playback controls
                    div { class: "control-group-center",
                        button {
                            class: "btn-icon btn-large-icon",
                            onclick: move |_| restart(),
                            "↻"
                        }
                        button { class: "btn-primary", "▶" }
                    }

                    // Right: Quality selector
                    div { class: "control-group-right",
                        span { class: "label-small", "Quality" }
                        select { class: "select-styled",
                            option { "Major" }
                            option { "Minor" }
                            option { "Dominant 7th" }
                            option { "Diminished" }
                        }
                    }
                }
            }
        }
    }
}
