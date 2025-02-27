use std::{sync::mpsc::Sender, time::Duration};

use chordflow_audio::audio::AudioCommand;
use chordflow_shared::{
    metronome::{calculate_duration_per_bar, MetronomeCommand, MetronomeEvent},
    mode::Mode,
    practice_state::PracticState,
};
use dioxus::prelude::*;

use crate::{MetronomeSignal, MetronomeState};

pub fn use_metronome(
    metronome: MetronomeSignal,
    mut metronome_state: Signal<MetronomeState>,
    mut practice_state: Signal<PracticState>,
    audio_tx: Signal<Sender<AudioCommand>>,
) {
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
}
