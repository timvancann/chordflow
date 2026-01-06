use atomic_float::AtomicF32;
use std::sync::{atomic::Ordering, LazyLock};

/// Global audio volume settings accessible from both UI and audio thread
pub static AUDIO_SETTINGS: LazyLock<AudioSettings> = LazyLock::new(AudioSettings::default);

pub struct AudioSettings {
    /// Volume for metronome accent (first beat of bar) - range 0.0 to 1.0
    pub metronome_accent_volume: AtomicF32,
    /// Volume for metronome normal beats - range 0.0 to 1.0
    pub metronome_beat_volume: AtomicF32,
    /// Volume for metronome subdivision ticks - range 0.0 to 1.0
    pub metronome_subdivision_volume: AtomicF32,
    /// Volume for chord playback - range 0.0 to 1.0
    pub chord_volume: AtomicF32,
}

impl Default for AudioSettings {
    fn default() -> Self {
        Self {
            metronome_accent_volume: AtomicF32::new(1.0),
            metronome_beat_volume: AtomicF32::new(0.8),
            metronome_subdivision_volume: AtomicF32::new(0.5),
            chord_volume: AtomicF32::new(0.7),
        }
    }
}

impl AudioSettings {
    /// Get metronome accent volume (0.0-1.0)
    pub fn get_metronome_accent_volume(&self) -> f32 {
        self.metronome_accent_volume.load(Ordering::Relaxed)
    }

    /// Set metronome accent volume (0.0-1.0)
    pub fn set_metronome_accent_volume(&self, volume: f32) {
        self.metronome_accent_volume
            .store(volume.clamp(0.0, 1.0), Ordering::Relaxed);
    }

    /// Get metronome beat volume (0.0-1.0)
    pub fn get_metronome_beat_volume(&self) -> f32 {
        self.metronome_beat_volume.load(Ordering::Relaxed)
    }

    /// Set metronome beat volume (0.0-1.0)
    pub fn set_metronome_beat_volume(&self, volume: f32) {
        self.metronome_beat_volume
            .store(volume.clamp(0.0, 1.0), Ordering::Relaxed);
    }

    /// Get metronome subdivision volume (0.0-1.0)
    pub fn get_metronome_subdivision_volume(&self) -> f32 {
        self.metronome_subdivision_volume.load(Ordering::Relaxed)
    }

    /// Set metronome subdivision volume (0.0-1.0)
    pub fn set_metronome_subdivision_volume(&self, volume: f32) {
        self.metronome_subdivision_volume
            .store(volume.clamp(0.0, 1.0), Ordering::Relaxed);
    }

    /// Get chord volume (0.0-1.0)
    pub fn get_chord_volume(&self) -> f32 {
        self.chord_volume.load(Ordering::Relaxed)
    }

    /// Set chord volume (0.0-1.0)
    pub fn set_chord_volume(&self, volume: f32) {
        self.chord_volume
            .store(volume.clamp(0.0, 1.0), Ordering::Relaxed);
    }
}
