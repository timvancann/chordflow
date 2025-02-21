use std::time::{Duration, Instant};

use chordflow_audio::audio::{play_audio, setup_audio};
use chordflow_music_theory::{
    note::Note,
    quality::Quality,
    scale::{Scale, ScaleType},
};
use chordflow_shared::{
    metronome::Metronome,
    mode::Mode,
    practice_state::{self, PracticState},
    DiatonicOption, ModeOption,
};
use dioxus::prelude::*;
use strum::IntoEnumIterator;

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

#[component]
fn App() -> Element {
    let current_beat = use_signal(|| 0usize);
    let current_bar = use_signal(|| 0usize);
    let practice_state = use_signal(PracticState::default);
    let metronome = use_timer(
        Duration::from_millis(10),
        current_beat,
        current_bar,
        practice_state,
    );

    let selected_mode = use_signal(|| ModeOption::Fourths);

    let num_bars = use_signal(|| metronome.read().num_bars);
    let num_beats = use_signal(|| metronome.read().num_beats);

    rsx! {
        // Global app resources
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        body {
            class: "bg-zinc-900 text-stone-300 w-screen h-screen",

        div { class: "container mx-auto space-y-8 bg-zinc-900 text-stone-300",

            ModeSelectionDisplay { practice_state, selected_mode }
            MetronomeDisplay {
                current_bar: *current_bar.read(),
                current_beat: *current_beat.read(),
                num_bars: *num_bars.read(),
                num_beats: *num_beats.read(),
            }

            PracticeStateDisplay { practice_state: practice_state.read().clone() }
        }
        }
    }
}

pub fn use_timer(
    tick: Duration,
    mut current_beat: Signal<usize>,
    mut current_bar: Signal<usize>,
    mut practice_state: Signal<PracticState>,
) -> Signal<Metronome> {
    let mut metronome = use_signal(|| Metronome::new(100, 2, 4, Instant::now));
    let mut audio = use_signal(|| setup_audio(None));

    use_future(move || async move {
        let mut m = metronome.write();
        m.start();
        play_audio(
            &mut audio.write(),
            practice_state.read().current_chord,
            m.duration_per_bar,
            m.num_beats,
        );
        loop {
            m.tick();
            if m.has_cycle_ended() {
                current_bar.set(m.current_bar);
                current_beat.set(m.current_beat);
                practice_state.write().next_chord();
                m.reset();
            }
            if m.beat_timer.ended {
                current_beat.set(m.current_beat);
            }
            if m.has_bar_ended() {
                play_audio(
                    &mut audio.write(),
                    practice_state.read().current_chord,
                    m.duration_per_bar,
                    m.num_beats,
                );
                current_beat.set(m.current_beat);
                current_bar.set(m.current_bar);
            }
            tokio::time::sleep(tick).await;
        }
    });
    metronome
}

#[derive(PartialEq, Props, Clone)]
struct ModeSelectionDisplayProps {
    practice_state: Signal<PracticState>,
    selected_mode: Signal<ModeOption>,
}

#[component]
fn ModeSelectionDisplay(props: ModeSelectionDisplayProps) -> Element {
    let mut selected_mode = props.selected_mode;
    let mut practice_state = props.practice_state;
    let selected_style = "border-2 border-orange-400";
    rsx! {
        div { class: "space-y-4",
            div { class: "flex justify-center gap-2",
                for mode in ModeOption::iter() {
                    div {
                        class: format!("px-2 py-1 cursor-pointer rounded-full bg-stone-500  {}", if mode == *selected_mode.read() { selected_style } else { "" }),
                        onclick: move |_| {
                            selected_mode.set(mode);
                            match *selected_mode.read() {
                                ModeOption::Fourths => {
                                    practice_state
                                        .write()
                                        .set_mode(
                                            Mode::Fourths(chordflow_music_theory::quality::Quality::Minor),
                                        )
                                }
                                ModeOption::Random => {
                                    practice_state.write().set_mode(Mode::Random(Quality::iter().collect()))
                                }
                                ModeOption::Custom => practice_state.write().set_mode(Mode::Custom(None)),
                                ModeOption::Diatonic => {
                                    practice_state
                                        .write()
                                        .set_mode(
                                            Mode::Diatonic(
                                                Scale::new(
                                                    Note::new(chordflow_music_theory::note::NoteLetter::C, 0),
                                                    ScaleType::Diatonic,
                                                ),
                                                DiatonicOption::Incemental,
                                            ),
                                        )
                                }
                            };
                        },
                        {mode.to_string()}
                    }
                }
            }
        }
    }
}

#[derive(PartialEq, Props, Clone)]
struct MetronomeDisplayProps {
    current_bar: usize,
    current_beat: usize,
    num_bars: usize,
    num_beats: usize,
}

#[component]
fn MetronomeDisplay(props: MetronomeDisplayProps) -> Element {
    rsx! {
        div { class: "space-y-4",
            div { class: "flex justify-center gap-2",
                for bar in 0..props.num_bars {
                    if bar > 0 {
                        span { " | " }
                    }
                    for tick in 0..props.num_beats {
                        if bar < props.current_bar || (bar == props.current_bar && tick < props.current_beat) {
                            div { class: format!("w-8 h-8 rounded-full transition-colors bg-amber-100") }
                        } else if bar < props.current_bar || (bar == props.current_bar && tick == props.current_beat) {
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

#[derive(PartialEq, Props, Clone)]
struct PracticeStateDisplayProps {
    practice_state: PracticState,
}
#[component]
fn PracticeStateDisplay(props: PracticeStateDisplayProps) -> Element {
    rsx! {
        div{
        p { class: "", "Current Chord: {props.practice_state.current_chord}" }
        p { class: "", "Next Chord: {props.practice_state.next_chord}" }
        }
    }
}
