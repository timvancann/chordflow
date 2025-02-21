use std::time::{Duration, Instant};

use chordflow_audio::audio::{play_audio, setup_audio};
use chordflow_shared::{metronome::Metronome, practice_state::PracticState};
use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

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

    let num_bars = use_signal(|| metronome.read().num_bars);
    let num_beats = use_signal(|| metronome.read().num_beats);

    rsx! {
        // Global app resources
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        div{
            class: "container mx-auto mt-4",
        MetronomeDisplay {
            current_bar: *current_bar.read(),
            current_beat: *current_beat.read(),
            num_bars: *num_bars.read(),
            num_beats: *num_beats.read(),
        }

        PracticeStateDisplay {
            practice_state: practice_state.read().clone()
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
                            div { class: format!("w-8 h-8 rounded-full transition-colors bg-blue-500") }
                        } else if bar < props.current_bar || (bar == props.current_bar && tick == props.current_beat) {
                            div { class: format!("w-8 h-8 rounded-full transition-colors bg-blue-800") }
                            }
                        else {
                            div { class: format!("w-8 h-8 rounded-full transition-colors bg-blue-100") }
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
        p {
            class: "text-red-500",
            "Current Chord: {props.practice_state.current_chord}"
        }
        p {
            class: "text-red-500",
            "Next Chord: {props.practice_state.next_chord}"
        }
    }
}
