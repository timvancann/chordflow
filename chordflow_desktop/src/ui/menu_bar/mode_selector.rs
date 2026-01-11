#![allow(non_snake_case)]

use chordflow_music_theory::scale::{Scale, ScaleType};
use dioxus::prelude::*;
use strum::IntoEnumIterator;

use crate::{state::modes::ModeOption, ui::app::AppState};

pub fn ModeSelector() -> Element {
    let mut app_state = use_context::<Signal<AppState>>();

    rsx! {
        div { class: "mode-selector-menu",
            div { class: "segmented-control",
                {
                    ModeOption::iter()
                        .map(|mode| {
                            let active_class = if app_state.read().selected_mode == mode { "active" } else { "" };
                            rsx! {
                                button {
                                    key: "{mode}",
                                    class: "segment {active_class}",
                                    onclick: move |_| {
                                        if app_state.read().selected_mode == mode {
                                            return;
                                        }
                                        app_state.write().selected_mode = mode;
                                    },
                                    "{mode}"
                                }
                            }
                        })
                }
            }
        }
    }
}
