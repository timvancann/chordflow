use std::sync::mpsc::Sender;

use chordflow_audio::audio::AudioCommand;
use chordflow_shared::{
    metronome::{calculate_duration_per_bar, MetronomeCommand},
    practice_state::{self, PracticState},
};
use dioxus::prelude::*;
use dioxus_free_icons::{icons::io_icons::IoReloadCircle, Icon};

use crate::{MetronomeSignal, MetronomeState};

#[component]
pub fn PlayControls() -> Element {
    rsx! {

        div{
            class: "flex justify-center",
            div {

            class: "flex button space-x-1 items-center",
            onclick: |_|{
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
                    calculate_duration_per_bar(metronome_state.read().bpm, metronome_state.read().ticks_per_bar).duration_per_bar,
                    metronome_state.read().ticks_per_bar,
                )));

            },
                Icon { icon: IoReloadCircle}
            span{
             "Restart"
            }
        }
        }

    }
}
