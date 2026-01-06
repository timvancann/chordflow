use dioxus::prelude::*;
use dioxus_free_icons::{
    icons::{
        fa_solid_icons::{FaPause, FaPlay},
        ld_icons::LdUndo,
    },
    Icon,
};

use crate::{ui::app::{AppState, MetronomeState}, AudioCommand, AUDIO_CMD};

pub fn PlayControl() -> Element {
    let mut app_state: Signal<AppState> = use_context();
    let mut metronome_state: Signal<MetronomeState> = use_context();

    rsx! {
        button {
            class: "btn-icon btn-large-icon",
            onclick: move |_| app_state.write().restart(),
            Icon { width: 20, height: 20, icon: LdUndo }
        }
        if app_state.read().is_playing {
            button {
                class: "btn-primary",
                onclick: move |_| {
                    app_state.write().is_playing = false;
                    // Reset UI state on stop
                    metronome_state.write().current_bar = 1;
                    metronome_state.write().current_tick = 0;
                    let _ = AUDIO_CMD.0.try_send(AudioCommand::Stop);
                },
                Icon { icon: FaPause }
            }
        } else {
            button {
                class: "btn-icon btn-large-icon",
                onclick: move |_| {
                    // Reset UI state on start
                    metronome_state.write().current_bar = 1;
                    metronome_state.write().current_tick = 0;
                    app_state.write().is_playing = true;
                    let _ = AUDIO_CMD.0.try_send(AudioCommand::Start);
                },
                Icon { icon: FaPlay }
            }
        }
    }
}
