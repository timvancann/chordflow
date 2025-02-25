use std::{
    borrow::BorrowMut,
    ops::DerefMut,
    sync::mpsc::{Receiver, Sender},
    thread,
    time::{Duration, Instant},
};

use chordflow_audio::audio::{setup_audio, AudioCommand};
use chordflow_shared::{
    metronome::{
        self, calculate_duration_per_bar, setup_metronome, MetronomeCommand, MetronomeEvent,
    },
    mode::{update_mode_from_state, Mode},
    practice_state::{self, ConfigState, PracticState},
    ModeOption,
};
use dioxus::prelude::*;
use strum::IntoEnumIterator;
use tokio::task;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

// orange-400
// stone-500
// zinc-900
// amber-100
// stone-300

fn main() {
    dioxus::launch(App);
}

type MetronomeSignal = Signal<(Sender<MetronomeCommand>, Receiver<MetronomeEvent>)>;

#[derive(PartialEq, Clone, Copy)]
struct MetronomeState {
    bars_per_chord: usize,
    ticks_per_bar: usize,
    bpm: usize,
    current_bar: usize,
    current_tick: usize,
}

#[component]
fn App() -> Element {
    let audio_tx = use_signal(|| setup_audio(None));
    let metronome = use_signal(|| setup_metronome(100, 2, 4, Instant::now));
    let mut practice_state = use_signal(PracticState::default);
    let selected_mode = use_signal(|| ModeOption::Fourths);
    let config_state = use_signal(ConfigState::default);
    let mut metronome_state = use_signal(|| MetronomeState {
        bars_per_chord: 2,
        ticks_per_bar: 4,
        bpm: 100,
        current_bar: 0,
        current_tick: 0,
    });

    use_context_provider(|| audio_tx);
    use_context_provider(|| metronome);
    use_context_provider(|| practice_state);
    use_context_provider(|| selected_mode);
    use_context_provider(|| config_state);
    use_context_provider(|| metronome_state);

    use_future(move || async move {
        let m = metronome.read();

        loop {
            while let Ok(event) = m.1.try_recv() {
                match event {
                    MetronomeEvent::CycleComplete => {
                        if let Mode::Custom(Some(p)) = &practice_state.read().mode {
                            metronome_state.write().bars_per_chord =
                                p.chords[practice_state.read().next_progression_chord_idx].bars;
                        }
                        let _ = m.0.send(MetronomeCommand::SetBars(
                            metronome_state.read().bars_per_chord,
                        ));
                        let _ = m.0.send(MetronomeCommand::Reset);
                        practice_state.write().next_chord();
                        metronome_state.write().current_bar = 0;
                        metronome_state.write().current_tick = 0;
                    }
                    MetronomeEvent::BarComplete(b) => {
                        let _ = audio_tx.read().send(AudioCommand::PlayChord((
                            practice_state.read().current_chord,
                            calculate_duration_per_bar(
                                metronome_state.read().bpm,
                                metronome_state.read().ticks_per_bar,
                            )
                            .duration_per_bar,
                            metronome_state.read().ticks_per_bar,
                        )));
                        metronome_state.write().current_bar = b;
                        metronome_state.write().current_tick = 0;
                    }
                    MetronomeEvent::Tick(t) => metronome_state.write().current_tick = t,
                };
            }

            tokio::time::sleep(Duration::from_millis(5)).await;
        }
    });

    rsx! {
        // Global app resources
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        body { class: "bg-zinc-900 text-stone-300 w-screen h-screen",

            div { class: "container mx-auto space-y-8 bg-zinc-900 text-stone-300",

            ModeSelectionDisplay{}
            MetronomeDisplay{}
                PracticeStateDisplay {}
            }
        }
    }
}

#[component]
fn ModeSelectionDisplay() -> Element {
    let mut metronome_state: Signal<MetronomeState> = use_context();
    let selected_style = "border-2 border-orange-400";
    let mut selected_mode: Signal<ModeOption> = use_context();
    let mut practice_state: Signal<PracticState> = use_context();
    let config_state: Signal<ConfigState> = use_context();
    let metronome: MetronomeSignal = use_context();
    let tx_audio: Signal<Sender<AudioCommand>> = use_context();
    rsx! {
        div { class: "space-y-4",
            div { class: "flex justify-center gap-2",
                for mode in ModeOption::iter() {
                    div {
                        class: format!(
                            "px-2 py-1 cursor-pointer rounded-full bg-stone-500  {}",
                            if mode == selected_mode() { selected_style } else { "" },
                        ),
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
                        {mode.to_string()}
                    }
                }
            }
        }
    }
}

#[component]
fn MetronomeDisplay() -> Element {
    let metronome_state: Signal<MetronomeState> = use_context();

    let m = metronome_state.read();
    rsx! {
        div { class: "space-y-4",
            div { class: "flex justify-center gap-2",
                for bar in 0..metronome_state.read().bars_per_chord {
                    if bar > 0 {
                        span { " | " }
                    }
                    for tick in 0..m.ticks_per_bar {
                        if bar < m.current_bar || (bar == m.current_bar && tick < m.current_tick) {
                            div { class: format!("w-8 h-8 rounded-full transition-colors bg-amber-100") }
                        } else if bar < m.current_bar || (bar == m.current_bar && tick == m.current_tick) {
                            div { class: format!("w-8 h-8 rounded-full transition-colors bg-orange-400") }
                        } else {
                            div { class: format!("w-8 h-8 rounded-full transition-colors bg-stone-300") }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn PracticeStateDisplay() -> Element {
    let practice_state: Signal<PracticState> = use_context();
    rsx! {
        div {
            p { class: "", "Current Chord: {practice_state.read().current_chord}" }
            p { class: "", "Next Chord: {practice_state.read().next_chord}" }
        }
    }
}
