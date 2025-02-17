use std::time::Instant;

use crate::timer;

pub struct Metronome {
    pub beat_timer: timer::Timer,
    pub duration_per_bar: u64,
    pub bpm: usize,

    pub current_bar: usize,
    pub num_bars: usize,
    pub num_beats: usize,
    pub current_beat: usize,
}

impl Metronome {
    pub fn new(
        bpm: usize,
        num_bars: usize,
        num_beats: usize,
        timer_source: impl Fn() -> Instant + 'static,
    ) -> Self {
        let duration_per_quarter_note = 60_000u64 / bpm as u64;
        let duration_per_bar = duration_per_quarter_note * num_beats as u64;

        Metronome {
            beat_timer: timer::Timer::new(duration_per_quarter_note as f64, timer_source),
            bpm,
            duration_per_bar,
            current_bar: 0,
            current_beat: 0,
            num_bars,
            num_beats,
        }
    }

    pub fn reset(&mut self) {
        self.current_bar = 0;
        self.current_beat = 0;
        self.sync_timers_with_bpm();
    }

    fn sync_timers_with_bpm(&mut self) {
        let duration_per_quarter_note = 60000u64 / self.bpm as u64;
        let duration_per_bar = duration_per_quarter_note * self.num_beats as u64;

        self.duration_per_bar = duration_per_bar;
        self.beat_timer.duration_ms = duration_per_quarter_note as f64;
    }

    pub fn start(&mut self) {
        self.beat_timer.start();
    }

    pub fn has_bar_ended(&self) -> bool {
        self.current_beat % self.num_beats == 0 && self.beat_timer.ended
    }

    pub fn has_cycle_ended(&self) -> bool {
        self.has_bar_ended() && self.current_bar % self.num_bars == 0 && self.beat_timer.ended
    }

    pub fn tick(&mut self) {
        self.beat_timer.tick();
        if self.beat_timer.ended {
            self.current_beat = (self.current_beat + 1) % self.num_beats;
        }
        if self.has_bar_ended() {
            self.current_bar = (self.current_bar + 1) % self.num_bars;
        }
    }

    pub fn toggle(&mut self) {
        self.beat_timer.toggle();
    }

    pub fn reset_timers(&mut self) {
        self.beat_timer.reset();
    }

    pub fn increase_bpm(&mut self, delta: usize) {
        self.bpm += delta;
    }
    pub fn decrease_bpm(&mut self, delta: usize) {
        self.bpm -= delta;
    }
}
