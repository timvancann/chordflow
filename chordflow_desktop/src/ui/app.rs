use std::time::Duration;

use chordflow_music_theory::chord::Chord;
use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
pub const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
pub const MAIN_CSS: Asset = asset!("/assets/main.css");

use crate::{
    state::{
        diatonic::DiatonicConfig, fourths::FourthsConfig, modes::ModeOption,
        progression::ProgressionConfig, random::RandomConfig, view::View,
    },
    ui::{
        bottom_zone::layout::BottomZone,
        center_stage::layout::CenterStage,
        menu_bar::layout::MenuBar,
        reference::chord_popup::{ChordPopup, SelectedChord},
        reference::layout::{ReferenceScreen, ReferenceState},
        top_zone::layout::TopZone,
        top_zone::subdivision_selector::Subdivision,
    },
    AudioCommand, AudioEvent, ReferenceEvent, AUDIO_CMD, AUDIO_EVT, INITIAL_BPM, REFERENCE_EVT,
};

pub struct MetronomeState {
    pub bars_per_chord: u8,
    pub ticks_per_bar: u8,
    pub bpm: u16,
    pub current_bar: u8,
    pub current_tick: u8,
    pub subdivision: Subdivision,
    pub count_in_enabled: bool,
}

impl Default for MetronomeState {
    fn default() -> Self {
        Self {
            bars_per_chord: 2,
            ticks_per_bar: 4,
            bpm: INITIAL_BPM,
            current_bar: 1,
            current_tick: 0,
            subdivision: Subdivision::default(),
            count_in_enabled: false,
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
    pub view: View,
    /// True while the reference page lives in its own window. The page exists
    /// in one place at a time, so the main window hides it while detached.
    pub reference_detached: bool,
    pub selected_mode: ModeOption,
    pub fourths_config: FourthsConfig,
    pub diatonic_config: DiatonicConfig,
    pub progression_config: ProgressionConfig,
    pub random_config: RandomConfig,
}

impl AppState {
    pub fn get_chords(&self) -> (String, String) {
        match self.selected_mode {
            ModeOption::Fourths => {
                let (current_chord, next_chord) = self.fourths_config.get_chords();
                (current_chord, next_chord)
            }
            ModeOption::Diatonic => {
                let (current_chord, next_chord) = self.diatonic_config.get_chords();
                (current_chord, next_chord)
            }
            ModeOption::Custom => {
                let (current_chord, next_chord) = self.progression_config.get_chords();
                (current_chord, next_chord)
            }
            ModeOption::Random => self.random_config.get_chords(),
        }
    }

    pub fn get_midi_codes_for_chords(&self) -> (Vec<u8>, Vec<u8>) {
        match self.selected_mode {
            ModeOption::Fourths => (
                chord_to_midi(self.fourths_config.current_chord),
                chord_to_midi(self.fourths_config.next_chord),
            ),
            ModeOption::Diatonic => (
                chord_to_midi(self.diatonic_config.current_chord),
                chord_to_midi(self.diatonic_config.next_chord),
            ),
            ModeOption::Custom => (
                self.progression_config
                    .current_chord
                    .clone()
                    .map(|c| c.to_midi_codes())
                    .unwrap_or_default(),
                self.progression_config
                    .next_chord
                    .clone()
                    .map(|c| c.to_midi_codes())
                    .unwrap_or_default(),
            ),

            ModeOption::Random => (
                chord_to_midi(self.random_config.current_chord),
                chord_to_midi(self.random_config.next_chord),
            ),
        }
    }

    pub fn advance(&mut self) {
        match self.selected_mode {
            ModeOption::Fourths => {
                self.fourths_config.generate_next_chord();
            }
            ModeOption::Diatonic => {
                self.diatonic_config.generate_next_chord();
            }
            ModeOption::Custom => {
                self.progression_config.generate_next_chord();
                let mut metronome_state: Signal<MetronomeState> = use_context();
                metronome_state.write().bars_per_chord =
                    self.progression_config.get_bars_per_cycle_current();
            }
            ModeOption::Random => {
                self.random_config.generate_next_chord();
            }
        }
    }

    pub fn restart(&mut self) {
        match self.selected_mode {
            ModeOption::Fourths => {
                self.fourths_config.reset();
            }
            ModeOption::Diatonic => {
                self.diatonic_config.reset();
            }
            ModeOption::Custom => {
                self.progression_config.reset();
                let mut metronome_state: Signal<MetronomeState> = use_context();
                metronome_state.write().bars_per_chord =
                    self.progression_config.get_bars_per_cycle_current();
            }
            ModeOption::Random => {
                self.random_config.reset();
            }
        }
        self.metronome_restart();
        // self.is_playing = false;
    }

    fn metronome_restart(&self) {
        let _ = AUDIO_CMD.0.try_send(AudioCommand::SetChord(Some(
            self.get_midi_codes_for_chords().0,
        )));
        let mut metronome_state: Signal<MetronomeState> = use_context();
        metronome_state.write().current_bar = 1;
        metronome_state.write().current_tick = 0;
        // Send restart to audio stream to sync counters
        let _ = AUDIO_CMD.0.try_send(AudioCommand::Restart);
    }
}

#[component]
pub fn App() -> Element {
    let selected_mode = use_signal(|| ModeOption::Fourths);
    let mut app_state = use_signal(AppState::default);
    let mut metronome_state = use_signal(MetronomeState::default);

    use_context_provider(|| selected_mode);
    use_context_provider(|| app_state);
    use_context_provider(|| metronome_state);
    let mut reference_state = use_signal(ReferenceState::default);
    use_context_provider(|| reference_state);
    use_context_provider(|| Signal::new(SelectedChord::default()));

    use_future(move || async move {
        let _ = AUDIO_CMD.0.try_send(AudioCommand::SetChord(Some(
            app_state.read().get_midi_codes_for_chords().0,
        )));
    });

    use_effect(move || {
        let _ = AUDIO_CMD.0.try_send(AudioCommand::SetBarsPerCycle(
            metronome_state.read().bars_per_chord,
        ));
        let _ = AUDIO_CMD
            .0
            .try_send(AudioCommand::SetBPM(metronome_state.read().bpm));
    });

    let _ = use_future(move || async move {
        loop {
            while let Ok(event) = AUDIO_EVT.1.try_recv() {
                match event {
                    AudioEvent::Tick => {
                        let cycle_done = metronome_state.write().tick();
                        if cycle_done {
                            app_state.write().advance();
                        }

                        let state = metronome_state.read();

                        if state.current_tick + 1 > state.ticks_per_bar {
                            if state.current_bar + 1 > state.bars_per_chord {
                                let _ = AUDIO_CMD.0.try_send(AudioCommand::SetChord(Some(
                                    app_state.read().get_midi_codes_for_chords().1,
                                )));
                            } else {
                                let _ = AUDIO_CMD.0.try_send(AudioCommand::SetChord(Some(
                                    app_state.read().get_midi_codes_for_chords().0,
                                )));
                            }
                        } else {
                            let _ = AUDIO_CMD.0.try_send(AudioCommand::SetChord(None));
                        }
                    }
                }
            }
            while let Ok(event) = REFERENCE_EVT.1.try_recv() {
                match event {
                    // The detached window went away. Dock the page again,
                    // carrying back whatever it was showing.
                    ReferenceEvent::Attached(state) => {
                        *reference_state.write() = state;
                        let mut app = app_state.write();
                        app.reference_detached = false;
                        app.view = View::Reference;
                    }
                }
            }

            tokio::time::sleep(Duration::from_millis(5)).await;
        }
    });

    let mut toggle_play = move || {
        if app_state.read().is_playing {
            app_state.write().is_playing = false;
            // Reset UI state on stop
            metronome_state.write().current_bar = 1;
            metronome_state.write().current_tick = 0;
            let _ = AUDIO_CMD.0.try_send(AudioCommand::Stop);
        } else {
            // Reset UI state on start
            metronome_state.write().current_bar = 1;
            metronome_state.write().current_tick = 0;
            app_state.write().is_playing = true;
            let _ = AUDIO_CMD.0.try_send(AudioCommand::Start);
        }
    };

    rsx! {
        // Global app resources
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        document::Link { rel: "stylesheet", href: MAIN_CSS }


        div {
            class: "app-container",
            tabindex: 0,
            onkeydown: move |e| {
                match e.key() {
                    Key::Character(c) if c == " " => toggle_play(),
                    Key::Character(c) if c.to_lowercase() == "r" => app_state.write().restart(),
                    _ => {}
                }
            },
            // Ambient glow background
            div { class: "ambient-bg" }

            MenuBar {}
            match app_state.read().view {
                View::Practice => rsx! {
                    TopZone {}
                    CenterStage {}
                    BottomZone {}
                },
                View::Reference => rsx! {
                    ReferenceScreen {}
                    ChordPopup {}
                },
            }
        }
    }
}

pub fn chord_to_midi(chord: Chord) -> Vec<u8> {
    chord
        .to_c_based_semitones()
        .into_iter()
        .map(note_to_midi)
        .collect()
}

pub fn note_to_midi(semitones_from_c: i32) -> u8 {
    ((semitones_from_c % 12) + 60) as u8
}
