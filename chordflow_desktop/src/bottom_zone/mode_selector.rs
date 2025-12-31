use chordflow_shared::ModeOption;
use dioxus::prelude::*;

pub fn ModeSelector() -> Element {
    let mut selected_mode_sig = use_context::<Signal<ModeOption>>();
    let selected_mode = *selected_mode_sig.read();

    rsx! {
        div { class: "mode-selector",
            span { class: "label-small", "MODE" }
            div { class: "segmented-control",
                {
                    [ModeOption::Fourths, ModeOption::Diatonic, ModeOption::Random, ModeOption::Custom].into_iter().map(|mode| {
                        let label = match mode {
                            ModeOption::Fourths => "Circle",
                            ModeOption::Diatonic => "Diatonic",
                            ModeOption::Random => "Random",
                            ModeOption::Custom => "Custom",
                        };
                        let active_class = if selected_mode == mode { "active" } else { "" };

                        rsx! {
                            button {
                                key: "{label}",
                                class: "segment {active_class}",
                                onclick: move |_| selected_mode_sig.set(mode),
                                "{label}"
                            }
                        }
                    })
                }
            }
        }
    }
}
