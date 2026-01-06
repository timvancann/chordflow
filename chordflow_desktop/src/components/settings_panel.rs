use dioxus::prelude::*;

use crate::audio::settings::AUDIO_SETTINGS;

#[component]
pub fn SettingsPanel(show: Signal<bool>) -> Element {
    let mut metronome_accent = use_signal(|| AUDIO_SETTINGS.get_metronome_accent_volume());
    let mut metronome_beat = use_signal(|| AUDIO_SETTINGS.get_metronome_beat_volume());
    let mut metronome_subdivision = use_signal(|| AUDIO_SETTINGS.get_metronome_subdivision_volume());
    let mut chord_volume = use_signal(|| AUDIO_SETTINGS.get_chord_volume());

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
fn VolumeSlider(
    label: String,
    value: Signal<f32>,
    on_change: EventHandler<f32>,
) -> Element {
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
