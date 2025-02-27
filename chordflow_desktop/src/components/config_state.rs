use chordflow_music_theory::{note::generate_all_roots, quality::Quality};
use chordflow_shared::{practice_state::ConfigState, DiatonicOption};
use dioxus::prelude::*;
use strum::IntoEnumIterator;

use crate::components::buttons::ToggleButton;

#[component]
pub fn ConfigStateDisplay() -> Element {
    let mut config_state: Signal<ConfigState> = use_context();
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
                                    onclick: move |_| {
                                        let mut c: Signal<ConfigState> = use_context();
                                        c.write().diatonic_option = q;
                                    },
                                    is_selected: q == config_state.read().diatonic_option,
                                    text: q.to_string(),
                                }
                            }
                        }
                        span { " | " }
                        select {
                            class: "select h-9",
                            onchange: |e| {
                                let index = e.value().parse::<usize>().unwrap();
                                let mut c: Signal<ConfigState> = use_context();
                                c.write().diatonic_root = generate_all_roots()[index];
                            },
                            for (i , root) in generate_all_roots().into_iter().enumerate() {
                                option {
                                    label: root.to_string(),
                                    value: i,
                                    selected: root == config_state.read().diatonic_root,
                                }
                            }
                        }
                    }
                },
            }

            SingleConfigStateDisplay {
                title: "Random Chords",
                children: rsx! {
                    div { class: "flex space-x-2 text-sm",
                        for q in Quality::iter() {
                            ToggleButton {
                                onclick: move |_| {
                                    if config_state.read().random_selected_qualities.contains(&q) {
                                        config_state.write().random_selected_qualities.retain(|s| *s != q);
                                    } else {
                                        config_state.write().random_selected_qualities.push(q);
                                    }
                                },
                                is_selected: config_state.read().random_selected_qualities.contains(&q),
                                text: q.name(),
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
