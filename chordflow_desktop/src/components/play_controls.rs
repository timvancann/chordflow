use std::sync::mpsc::Sender;

use chordflow_audio::audio::AudioCommand;
use chordflow_shared::{
    metronome::{calculate_duration_per_bar, MetronomeCommand},
    practice_state::PracticState,
};
use dioxus::prelude::*;
use dioxus_free_icons::{
    icons::{
        fa_solid_icons::{FaPause, FaPlay},
        io_icons::{IoReloadCircle, IoSaveSharp},
    },
    Icon,
};

use crate::{
    components::{apply_selected_changes, buttons::Button},
    MetronomeSignal, MetronomeState,
};

pub fn restart() {
    let mut practice_state: Signal<PracticState> = use_context();
    let mut metronome_state: Signal<MetronomeState> = use_context();
    let metronome: MetronomeSignal = use_context();
    let tx_audio: Signal<Sender<AudioCommand>> = use_context();
    practice_state.write().reset();
    metronome_state.write().current_bar = 0;
    metronome_state.write().current_tick = 0;
    let _ = metronome.read().0.send(MetronomeCommand::Reset);
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

#[component]
pub fn PlayControls() -> Element {
    rsx! {
        div { class: "flex justify-center items-center space-x-4",
            Button {
                onclick: |_| restart(),
                icon: rsx! {
                    Icon { icon: IoReloadCircle }
                },
                text: "Restart",
            }
            Button {
                onclick: |_| {
                    apply_selected_changes();
                },
                icon: rsx! {
                    Icon { icon: IoSaveSharp }
                },
                text: "Apply Changes",
            }
            Button {
                onclick: |_| {
                    let metronome: MetronomeSignal = use_context();
                    let tx_audio: Signal<Sender<AudioCommand>> = use_context();
                    let _ = tx_audio.read().send(AudioCommand::Play);
                    let _ = metronome.read().0.send(MetronomeCommand::Play);
                },
                icon: rsx! {
                    Icon { icon: FaPlay }
                },
            }
            Button {
                onclick: |_| {
                    let metronome: MetronomeSignal = use_context();
                    let tx_audio: Signal<Sender<AudioCommand>> = use_context();
                    let _ = metronome.read().0.send(MetronomeCommand::Pause);
                    let _ = tx_audio.read().send(AudioCommand::Pause);
                },
                icon: rsx! {
                    Icon { icon: FaPause }
                },
            }
        }
    }
}
