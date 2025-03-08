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

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

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
        if *initial_setup.read() {
            restart();
            initial_setup.set(false);
        }
    });

    rsx! {
        // Global app resources
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        body { class: " bg-tokyoNight-bg text-tokyoNight-fg w-screen h-screen",

            Header {}
            div { class: "m-2 p-2 flex-col space-y-4",


                div { class: "flex space-x-4",
                    div { class: " bg-tokyoNight-bg_highlight/70 p-4 rounded-md",
                        ModeSelectionDisplay {}
                    }
                    div { class: " bg-tokyoNight-bg_highlight/70 flex-1 p-4 rounded-md flex-col space-y-4",
                        div { class: "", MetronomeDisplay {} }
                        div { class: "", MetronomSettingsDisplay {} }
                        div { class: "", PracticeStateDisplay {} }
                        div { class: "", PlayControls {} }
                    }
                }
                div { class: "flex-1 space-x-4",
                    div { class: " bg-tokyoNight-bg_highlight/70 p-4 rounded-md", ConfigStateDisplay {} }
                }
            }
        }
    }
}
