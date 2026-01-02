use dioxus::prelude::*;
use dioxus_free_icons::{
    icons::{
        fa_solid_icons::{FaPause, FaPlay},
        ld_icons::LdUndo,
    },
    Icon,
};

use crate::{components::play_controls::restart, AppState, AudioCommand, AUDIO_CMD};

pub fn PlayControls() -> Element {
    let mut app_state = use_context::<Signal<AppState>>();
    let state = app_state.read().clone();
    rsx! {
        div { class: "control-group-center",
            button { class: "btn-icon btn-large-icon", onclick: move |_| restart(),
                Icon { width: 20, height: 20, icon: LdUndo }
            }
            if state.is_playing {
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
}
