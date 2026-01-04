use chordflow_music_theory::{
    note::generate_all_roots,
    scale::{Scale, ScaleType},
};
use dioxus::prelude::*;
use strum::IntoEnumIterator;

use crate::{
    state::{config::ConfigState, mode::Mode, options::DiatonicOption, practice::PracticeState},
    ui::app::AppState,
};

pub fn DiatonicSelector() -> Element {
    let mut config_state = use_context::<Signal<AppState>>();
    let mut practice_state = use_context::<Signal<PracticeState>>();

    rsx! {
        div { class: "control-group-right",
            // Root note dropdown
            span { class: "label-small", "Root" }
            select {
                class: "select-styled",
                value: "{config_state.read().diatonic_root}",
                onchange: move |e| {
                    if let Some(root) = generate_all_roots().get(e.value().parse::<usize>().unwrap_or(0)) {
                        config_state.write().diatonic_root = *root;
                        practice_state.write().set_mode(Mode::Diatonic(Scale::new(*root, ScaleType::Diatonic), config_state.read().diatonic_option));
                    }
                },
                for (i, root) in generate_all_roots().into_iter().enumerate() {
                    option {
                        value: "{i}",
                        selected: root == config_state.read().diatonic_root,
                        "{root}"
                    }
                }
            }

            // Diatonic option dropdown (Incremental/Random)
            span { class: "label-small", "Mode" }
            select {
                class: "select-styled",
                value: "{config_state.read().diatonic_option}",
                onchange: move |e| {
                    if let Some(option) = DiatonicOption::iter().nth(e.value().parse::<usize>().unwrap_or(0)) {
                        config_state.write().diatonic_option = option;
                        practice_state.write().set_mode(Mode::Diatonic(Scale::new(config_state.read().diatonic_root, ScaleType::Diatonic), option));
                    }
                },
                for (i, option) in DiatonicOption::iter().enumerate() {
                    option {
                        value: "{i}",
                        selected: option == config_state.read().diatonic_option,
                        "{option}"
                    }
                }
            }
        }
    }
}
