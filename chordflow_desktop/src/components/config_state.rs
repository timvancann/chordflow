use chordflow_music_theory::{note::generate_all_roots, quality::Quality};
use chordflow_shared::{practice_state::ConfigState, DiatonicOption};
use dioxus::prelude::*;
use strum::IntoEnumIterator;

use crate::components::buttons::ToggleButton;

#[component]
pub fn ConfigStateDisplay() -> Element {
    let config_state: Signal<ConfigState> = use_context();
    rsx! {
        div { class: "flex-col space-y-4",
            SingleConfigStateDisplay {
                title: "Circle of Fourths",
                children: rsx! {
                    div { class: "flex space-x-2 text-sm",
                        for q in Quality::iter() {
                            ToggleButton {
                                onclick: move |_| {
                                    let mut c: Signal<ConfigState> = use_context();
                                    c.write().fourths_selected_quality = q;
                                },
                                is_selected: q == config_state.read().fourths_selected_quality,
                                text: q.name(),
                            }
                        }
                    }
                },
            }
            SingleConfigStateDisplay {
                title: "Diatonic Progression",
                children: rsx! {
                    div { class: "flex space-x-4 text-sm items-center",
                        div { class: "flex space-x-2",
                            for q in DiatonicOption::iter() {
                                ToggleButton {
                                    onclick: |_| {},
                                    is_selected: q == config_state.read().diatonic_option,
                                    text: q.to_string(),
                                }
                            }
                        }
                        span { " | " }
                        select { class: "select h-9", onchange: |_| {},
                            for root in generate_all_roots() {
                                option {
                                    label: root.to_string(),
                                    value: root.to_string(),
                                    selected: root == config_state.read().diatonic_root,
                                }
                            }
                        }
                    }
                },
            }
        }
    }
}

#[component]
pub fn SingleConfigStateDisplay(title: String, children: Element) -> Element {
    rsx! {
        div { class: "flex-col bg-tokyoNight-bg p-2 space-y-2 rounded-md",
            div { class: "font-semibold tracking-wide", {title} }
            {children}
        }
    }
}
