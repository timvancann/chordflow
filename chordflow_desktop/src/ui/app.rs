use std::time::Duration;

use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
const MAIN_CSS: Asset = asset!("/assets/main.css");

use crate::{
    state::{config::ConfigState, options::ModeOption, practice::PracticeState},
    ui::{
        bottom_zone::layout::BottomZone, center_stage::layout::CenterStage,
        top_zone::layout::TopZone,
    },
    AudioCommand, AudioEvent, AUDIO_CMD, AUDIO_EVT, INITIAL_BPM,
};

pub struct MetronomeState {
    pub bars_per_chord: u8,
    pub ticks_per_bar: u8,
    pub bpm: u16,
    pub current_bar: u8,
    pub current_tick: u8,
}

impl Default for MetronomeState {
    fn default() -> Self {
        Self {
            bars_per_chord: 2,
            ticks_per_bar: 4,
            bpm: INITIAL_BPM,
            current_bar: 1,
            current_tick: 0,
        }
    }
}

impl MetronomeState {
    fn tick(&mut self) -> bool {
        self.current_tick += 1;
        let mut cycle_done = false;

        if self.current_tick > self.ticks_per_bar {
            self.current_tick = 1;
            self.current_bar += 1;

            if self.current_bar > self.bars_per_chord {
                self.current_bar = 1;
                cycle_done = true;
            }
        }
        cycle_done
    }
}

#[derive(Default)]
pub struct AppState {
    pub is_playing: bool,
}

#[component]
pub fn App() -> Element {
    let selected_mode = use_signal(|| ModeOption::Fourths);
    let config_state = use_signal(ConfigState::default);
    let app_state = use_signal(AppState::default);
    let mut practice_state = use_signal(PracticeState::default);
    let mut metronome_state = use_signal(MetronomeState::default);

    // let _audio_engine = use_signal(|| setup_audio(default_metronome_state, None).expect("Failed to setup audio"));

    use_context_provider(|| practice_state);
    use_context_provider(|| selected_mode);
    use_context_provider(|| config_state);

    // use_metronome(metronome_state, practice_state);
    use_context_provider(|| app_state);
    use_context_provider(|| practice_state);
    use_context_provider(|| metronome_state);

    use_future(move || async move {
        let _ = AUDIO_CMD.0.try_send(AudioCommand::SetChord(Some(
            practice_state.read().current_chord,
        )));
    });

    let _ = use_future(move || async move {
        loop {
            while let Ok(event) = AUDIO_EVT.1.try_recv() {
                match event {
                    AudioEvent::Tick => {
                        let cycle_done = metronome_state.write().tick();
                        if cycle_done {
                            practice_state.write().next_chord();
                        }

                        let state = metronome_state.read();

                        if state.current_tick + 1 > state.ticks_per_bar {
                            if state.current_bar + 1 > state.bars_per_chord {
                                let _ = AUDIO_CMD.0.try_send(AudioCommand::SetChord(Some(
                                    practice_state.read().next_chord,
                                )));
                            } else {
                                let _ = AUDIO_CMD.0.try_send(AudioCommand::SetChord(Some(
                                    practice_state.read().current_chord,
                                )));
                            }
                        } else {
                            let _ = AUDIO_CMD.0.try_send(AudioCommand::SetChord(None));
                        }
                    }
                }
            }
            tokio::time::sleep(Duration::from_millis(5)).await;
        }
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
