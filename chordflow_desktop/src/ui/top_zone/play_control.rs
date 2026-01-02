use dioxus::prelude::*;
use dioxus_free_icons::{
    icons::fa_solid_icons::{FaPause, FaPlay},
    Icon,
};

use crate::{ui::app::AppState, AudioCommand, AUDIO_CMD};

pub fn PlayControl() -> Element {
    let mut app_state: Signal<AppState> = use_context();
    rsx! {
        if app_state.read().is_playing {
            button {
                class: "btn-primary",
                onclick: move |_| {
                    app_state.write().is_playing = false;
                    let _ = AUDIO_CMD.0.try_send(AudioCommand::Stop);
                },
                Icon { icon: FaPause }
            }
        } else {
            button {
                class: "btn-icon btn-large-icon",
                onclick: move |_| {
                    app_state.write().is_playing = true;
                    let _ = AUDIO_CMD.0.try_send(AudioCommand::Start);
                },
                Icon { icon: FaPlay }
            }
        }
    }
}
