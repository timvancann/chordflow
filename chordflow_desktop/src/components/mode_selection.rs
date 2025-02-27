use std::{borrow::BorrowMut, sync::mpsc::Sender};

use chordflow_audio::audio::AudioCommand;
use chordflow_shared::{
    metronome::{calculate_duration_per_bar, MetronomeCommand},
    mode::{update_mode_from_state, Mode},
    practice_state::{ConfigState, PracticState},
    ModeOption,
};
use dioxus::prelude::*;
use dioxus_free_icons::{icons::fa_solid_icons::FaSquareCheck, Icon};
use strum::IntoEnumIterator;

use crate::{MetronomeSignal, MetronomeState};

#[component]
pub fn ModeOptionDisplay(
    mode: ModeOption,
    is_selected: bool,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
        div {
            class: format!(
                "flex items-center space-x-2 {}",
                if is_selected { "selected-button " } else { "button" },
            ),
            onclick,
            if is_selected {
                Icon { icon: FaSquareCheck }
            }
            span { {mode.to_string()} }
        }
    }
}

#[component]
pub fn ModeSelectionDisplay() -> Element {
    let mut metronome_state: Signal<MetronomeState> = use_context();
    let mut selected_mode: Signal<ModeOption> = use_context();
    let mut practice_state: Signal<PracticState> = use_context();
    let config_state: Signal<ConfigState> = use_context();
    let metronome: MetronomeSignal = use_context();
    let tx_audio: Signal<Sender<AudioCommand>> = use_context();
    rsx! {
        div { class: "space-y-4 w-60",
            p { class: "text-tokyoNight-blue font-bold text-xl", "Practice Mode" }
            div { class: "flex-col justify-center space-y-2",
                for mode in ModeOption::iter() {
                    ModeOptionDisplay {
                        mode,
                        is_selected: mode == selected_mode(),
                        onclick: move |_| {
                            selected_mode.set(mode);
                            let has_changed = update_mode_from_state(
                                &selected_mode(),
                                practice_state.write().borrow_mut(),
                                &config_state(),
                            );
                            if let Mode::Custom(Some(p)) = practice_state().mode {
                                metronome_state.write().bars_per_chord = p
                                    .chords[practice_state().next_progression_chord_idx]
                                    .bars;
                            }
                            let _ = metronome
                                .read()
                                .0
                                .send(MetronomeCommand::SetBars(metronome_state.read().bars_per_chord));
                            if has_changed {
                                let _ = metronome.read().0.send(MetronomeCommand::Reset);
                                metronome_state.write().current_bar = 0;
                                metronome_state.write().current_tick = 0;
                                let _ = tx_audio
                                    .read()
                                    .send(
                                        AudioCommand::PlayChord((
                                            practice_state.read().current_chord,
                                            calculate_duration_per_bar(
                                                    metronome_state.read().bpm,
                                                    metronome_state.read().ticks_per_bar,
                                                )
                                                .duration_per_bar,
                                            metronome_state.read().ticks_per_bar,
                                        )),
                                    );
                            }
                        },
                    }
                }
            }
        }
    }
}
