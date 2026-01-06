use chordflow_music_theory::note::generate_all_roots;
use dioxus::prelude::*;

use crate::ui::app::AppState;

pub fn DiatonicSelector() -> Element {
    let mut config_state = use_context::<Signal<AppState>>();

    rsx! {
        div { class: "control-group-right",
            // Root note dropdown
            span { class: "label-small", "Root" }
            select {
                class: "select-styled",
                value: "{config_state.read().diatonic_config.scale.root}",
                onchange: move |e| {
                    if let Some(root) = generate_all_roots().get(e.value().parse::<usize>().unwrap_or(0)) {
                        config_state.write().diatonic_config.set_root(*root);
                    }
                },
                for (i, root) in generate_all_roots().into_iter().enumerate() {
                    option {
                        value: "{i}",
                        selected: root == config_state.read().diatonic_config.scale.root,
                        "{root}"
                    }
                }
            }

            // Random mode checkbox
            span { class: "label-small", "Random" }
            input {
                r#type: "checkbox",
                checked: config_state.read().diatonic_config.is_random,
                onchange: move |e| {
                    config_state.write().diatonic_config.is_random = e.value().parse::<bool>().unwrap_or(false);
                }
            }
        }
    }
}
