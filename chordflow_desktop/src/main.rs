use std::{
    fmt::Display,
    time::Instant,
};

use chordflow_audio::audio::setup_audio;
use chordflow_shared::{
    metronome::{setup_metronome},
    practice_state::{ConfigState, PracticeState},
    ModeOption,
};
use components::{
    play_controls::{restart, PlayControls},
};
use dioxus::{
    desktop::{tao::platform::macos::WindowBuilderExtMacOS, Config, LogicalSize, WindowBuilder},
    prelude::*,
};
use hooks::use_metronome::use_metronome;

mod bottom_zone;
mod center_stage;
mod components;
mod hooks;
mod top_zone;

use crate::{bottom_zone::layout::BottomZone, center_stage::layout::CenterStage, top_zone::layout::TopZone};

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    let window_builder = WindowBuilder::new()
        .with_transparent(false)
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

#[derive(PartialEq, Clone, Copy)]
struct MetronomeState {
    bars_per_chord: usize,
    ticks_per_bar: usize,
    bpm: usize,
    current_bar: usize,
    current_tick: usize,
}

#[derive(PartialEq, Clone, Copy)]
struct AppState {
    is_playing: bool,
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
    let practice_state = use_signal(PracticeState::default);
    let selected_mode = use_signal(|| ModeOption::Fourths);
    let config_state = use_signal(ConfigState::default);
    let metronome_state = use_signal(|| MetronomeState {
        bars_per_chord: 2,
        ticks_per_bar: 4,
        bpm: 100,
        current_bar: 0,
        current_tick: 0,
    });
    let app_state = use_signal(|| AppState {
        is_playing: true
    });

    use_context_provider(|| audio_tx);
    use_context_provider(|| metronome);
    use_context_provider(|| practice_state);
    use_context_provider(|| selected_mode);
    use_context_provider(|| config_state);
    use_context_provider(|| metronome_state);
    use_context_provider(|| app_state);

    use_metronome(metronome_state, practice_state, audio_tx);

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

            TopZone {}
            CenterStage {}
            BottomZone {}
        }
    }
}
