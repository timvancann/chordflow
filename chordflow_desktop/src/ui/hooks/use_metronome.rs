use std::time::Duration;

use chordflow_audio::audio::{AudioCommand, ChordRequest, AUDIO_COMMAND_CHANNEL};
use chordflow_shared::{
    metronome::{
        calculate_duration_per_bar, MetronomeCommand, MetronomeEvent, METRONOME_COMMAND_CHANNEL,
        METRONOME_EVENT_CHANNEL,
    },
    mode::Mode,
    practice_state::PracticeState,
};
use dioxus::prelude::*;

use crate::MetronomeState;

pub fn use_metronome(
    mut metronome_state: Signal<MetronomeState>,
    mut practice_state: Signal<PracticeState>,
) {
    use_future(move || async move {
        loop {
            while let Ok(event) = METRONOME_EVENT_CHANNEL.1.try_recv() {
                match event {
                    MetronomeEvent::CycleComplete => {
                        if let Mode::Custom(Some(p)) = &practice_state.read().mode {
                            metronome_state.write().bars_per_chord =
                                p.chords[practice_state.read().next_progression_chord_idx].bars;
                        }
                        let _ = METRONOME_COMMAND_CHANNEL
                            .0
                            .try_send(MetronomeCommand::SetBars(
                                metronome_state.read().bars_per_chord,
                            ));
                        let _ = METRONOME_COMMAND_CHANNEL
                            .0
                            .try_send(MetronomeCommand::Reset);
                        practice_state.write().next_chord();
                        metronome_state.write().current_bar = 0;
                        metronome_state.write().current_tick = 0;
                    }
                    MetronomeEvent::BarComplete(b) => {
                        let _ = AUDIO_COMMAND_CHANNEL.sender.send(AudioCommand::PlayChord(
                            ChordRequest {
                                chord: practice_state.read().current_chord,
                                duration: calculate_duration_per_bar(
                                    metronome_state.read().bpm,
                                    metronome_state.read().ticks_per_bar,
                                )
                                .duration_per_bar,
                                ticks_per_bar: metronome_state.read().ticks_per_bar,
                            },
                        ));
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
