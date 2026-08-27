use chordflow_music_theory::{
    chord::Chord,
    note::{Note, NoteLetter},
    quality::{Notation, Quality},
};
use dioxus::prelude::*;
use strum::IntoEnumIterator;

use crate::audio::settings::AUDIO_SETTINGS;

#[component]
pub fn SettingsPanel(show: Signal<bool>) -> Element {
    let mut metronome_accent = use_signal(|| AUDIO_SETTINGS.get_metronome_accent_volume());
    let mut metronome_beat = use_signal(|| AUDIO_SETTINGS.get_metronome_beat_volume());
    let mut metronome_subdivision =
        use_signal(|| AUDIO_SETTINGS.get_metronome_subdivision_volume());
    let mut chord_volume = use_signal(|| AUDIO_SETTINGS.get_chord_volume());
    let mut notation = use_context::<Signal<Notation>>();

    if !show() {
        return rsx! { div {} };
    }

    rsx! {
        div {
            class: "settings-overlay",
            onclick: move |_| show.set(false),

            div {
                class: "settings-panel",
                onclick: move |e| e.stop_propagation(),

                div { class: "settings-header",
                    h2 { class: "settings-title", "Settings" }
                    button {
                        class: "settings-close",
                        onclick: move |_| show.set(false),
                        "✕"
                    }
                }

                div { class: "settings-content",
                    // Chord notation
                    div { class: "settings-section",
                        h3 { class: "section-title", "Chord symbols" }
                        p { class: "detail-note",
                            "How chord symbols are written, everywhere in the app."
                        }
                        div { class: "chip-row", style: "margin-top: 10px",
                            {
                                Notation::iter()
                                    .map(|option| {
                                        let active_class = if *notation.read() == option { "active" } else { "" };
                                        // Show the notation's name beside a
                                        // worked example, since the names alone
                                        // do not tell you what you will get.
                                        let c = Note::new(NoteLetter::C, 0);
                                        let example = format!(
                                            "{}  \u{2014}  C  {}  {}",
                                            option.display_name(),
                                            Chord::new(c, Quality::Minor).symbol(option),
                                            Chord::new(c, Quality::MajorSeventh).symbol(option),
                                        );
                                        rsx! {
                                            button {
                                                key: "{option}",
                                                class: "key-chip {active_class}",
                                                onclick: move |_| notation.set(option),
                                                "{example}"
                                            }
                                        }
                                    })
                            }
                        }
                    }

                    // Volume Controls Section
                    div { class: "settings-section",
                        h3 { class: "section-title", "Volume Controls" }

                        VolumeSlider {
                            label: "Metronome Accent (Downbeat)",
                            value: metronome_accent,
                            on_change: move |val: f32| {
                                metronome_accent.set(val);
                                AUDIO_SETTINGS.set_metronome_accent_volume(val);
                            }
                        }

                        VolumeSlider {
                            label: "Metronome Beat",
                            value: metronome_beat,
                            on_change: move |val: f32| {
                                metronome_beat.set(val);
                                AUDIO_SETTINGS.set_metronome_beat_volume(val);
                            }
                        }

                        VolumeSlider {
                            label: "Metronome Subdivision",
                            value: metronome_subdivision,
                            on_change: move |val: f32| {
                                metronome_subdivision.set(val);
                                AUDIO_SETTINGS.set_metronome_subdivision_volume(val);
                            }
                        }

                        VolumeSlider {
                            label: "Chord Volume",
                            value: chord_volume,
                            on_change: move |val: f32| {
                                chord_volume.set(val);
                                AUDIO_SETTINGS.set_chord_volume(val);
                            }
                        }
                    }

                    // Keyboard Shortcuts Section
                    div { class: "settings-section",
                        h3 { class: "section-title", "Keyboard Shortcuts" }

                        div { class: "shortcuts-grid",
                            KeyboardShortcut {
                                keys: "Space",
                                description: "Play / Pause"
                            }
                            KeyboardShortcut {
                                keys: "R",
                                description: "Restart"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn VolumeSlider(label: String, value: Signal<f32>, on_change: EventHandler<f32>) -> Element {
    let percentage = (value() * 100.0) as i32;

    rsx! {
        div { class: "volume-control",
            label { class: "volume-label",
                span { "{label}" }
                span { class: "volume-value", "{percentage}%" }
            }
            input {
                r#type: "range",
                class: "volume-slider",
                min: "0",
                max: "100",
                value: "{percentage}",
                oninput: move |e| {
                    if let Ok(val) = e.value().parse::<f32>() {
                        on_change.call(val / 100.0);
                    }
                }
            }
        }
    }
}

#[component]
fn KeyboardShortcut(keys: String, description: String) -> Element {
    rsx! {
        div { class: "shortcut-item",
            div { class: "shortcut-keys",
                for key in keys.split('+') {
                    kbd { class: "key", "{key.trim()}" }
                }
            }
            div { class: "shortcut-description", "{description}" }
        }
    }
}
