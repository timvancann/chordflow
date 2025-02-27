use std::{borrow::BorrowMut, sync::mpsc::Sender};

use chordflow_audio::audio::AudioCommand;
use chordflow_shared::{
    metronome::{calculate_duration_per_bar, MetronomeCommand},
    mode::{update_mode_from_state, Mode},
    practice_state::{ConfigState, PracticState},
    ModeOption,
};
use dioxus::prelude::*;

use crate::{MetronomeSignal, MetronomeState};

pub mod buttons;
pub mod config_state;
pub mod header;
pub mod metronome;
pub mod metronome_settings;
pub mod mode_selection;
pub mod play_controls;
pub mod practice_state;

pub fn apply_selected_changes() {
    let mut metronome_state: Signal<MetronomeState> = use_context();
    let selected_mode: Signal<ModeOption> = use_context();
    let mut practice_state: Signal<PracticState> = use_context();
    let config_state: Signal<ConfigState> = use_context();
    let metronome: MetronomeSignal = use_context();
    let tx_audio: Signal<Sender<AudioCommand>> = use_context();
    let has_changed = update_mode_from_state(
        &selected_mode(),
        practice_state.write().borrow_mut(),
        &config_state(),
    );
    if let Mode::Custom(Some(p)) = practice_state().mode {
        metronome_state.write().bars_per_chord =
            p.chords[practice_state().next_progression_chord_idx].bars;
    }
    let _ = metronome.read().0.send(MetronomeCommand::SetBars(
        metronome_state.read().bars_per_chord,
    ));
    if has_changed {
        let _ = metronome.read().0.send(MetronomeCommand::Reset);
        metronome_state.write().current_bar = 0;
        metronome_state.write().current_tick = 0;
        let _ = tx_audio.read().send(AudioCommand::PlayChord((
            practice_state.read().current_chord,
            calculate_duration_per_bar(
                metronome_state.read().bpm,
                metronome_state.read().ticks_per_bar,
            )
            .duration_per_bar,
            metronome_state.read().ticks_per_bar,
        )));
    }
}
