use chordflow_audio::audio::{AudioCommand, ChordRequest, AUDIO_COMMAND_CHANNEL};
use chordflow_shared::{
    metronome::{calculate_duration_per_bar, MetronomeCommand, METRONOME_COMMAND_CHANNEL},
    practice_state::PracticeState,
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
    MetronomeState,
};

#[component]
pub fn PlayControls() -> Element {
    // tx_audio removal
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
                onclick: move |_| {
                    // AudioCommand::Play is removed. MetronomeCommand handles it.
                    let _ = METRONOME_COMMAND_CHANNEL.0.try_send(MetronomeCommand::Play);
                },
                icon: rsx! {
                    Icon { icon: FaPlay }
                },
            }
            Button {
                onclick: move |_| {
                    // AudioCommand::Pause is removed. MetronomeCommand handles it.
                    let _ = METRONOME_COMMAND_CHANNEL.0.try_send(MetronomeCommand::Pause);
                },
                icon: rsx! {
                    Icon { icon: FaPause }
                },
            }
        }
    }
}
