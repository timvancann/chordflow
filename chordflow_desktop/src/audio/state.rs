use chordflow_music_theory::chord::Chord;
use parking_lot::Mutex;
use rustysynth::Synthesizer;
use std::sync::{
    atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering},
    Arc,
};

#[derive(Clone)]
pub struct AudioState {
    pub bpm: Arc<AtomicU32>,
    pub is_playing: Arc<AtomicBool>,
    pub ticks_per_bar: Arc<AtomicU32>,

    // Playback state
    pub current_tick: Arc<AtomicU64>,
    pub samples_processed: Arc<AtomicU64>,

    // Command triggers
    pub play_request: Arc<Mutex<Option<PlayRequest>>>,

    // Audio engine
    pub synth: Arc<Mutex<Synthesizer>>,
}

#[derive(Clone)]
pub struct PlayRequest {
    pub chord: Chord,
    pub duration_ms: u64,
}

impl AudioState {
    pub fn new(synth: Synthesizer) -> Self {
        Self {
            bpm: Arc::new(AtomicU32::new(120)),
            is_playing: Arc::new(AtomicBool::new(false)),
            ticks_per_bar: Arc::new(AtomicU32::new(4)),
            current_tick: Arc::new(AtomicU64::new(0)),
            samples_processed: Arc::new(AtomicU64::new(0)),
            play_request: Arc::new(Mutex::new(None)),
            synth: Arc::new(Mutex::new(synth)),
        }
    }

    pub fn reset(&self) {
        self.current_tick.store(0, Ordering::Relaxed);
        self.samples_processed.store(0, Ordering::Relaxed);
    }
}
