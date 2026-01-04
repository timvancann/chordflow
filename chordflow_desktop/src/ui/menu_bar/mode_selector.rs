use chordflow_music_theory::scale::{Scale, ScaleType};
use dioxus::prelude::*;
use strum::IntoEnumIterator;

use crate::{
    state::{mode::Mode, options::ModeOption, practice::PracticeState},
    ui::app::AppState,
};

pub fn ModeSelector() -> Element {
    let mut app_state = use_context::<Signal<AppState>>();
    let mut practice_state = use_context::<Signal<PracticeState>>();

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
                                        app_state.write().selected_mode = mode;
                                        match app_state.read().selected_mode {
                                            ModeOption::Fourths => {
                                                practice_state.write().set_mode(Mode::Fourths(app_state.read().fourths_quality));
                                            },
                                            ModeOption::Diatonic => {
                                                let root = app_state.read().diatonic_root;
                                                let option = app_state.read().diatonic_option;
                                                practice_state.write().set_mode(Mode::Diatonic(Scale::new(root, ScaleType::Diatonic), option ));
                                            },
                                            _ => {}
                                        }
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
