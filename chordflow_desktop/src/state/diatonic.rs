use chordflow_music_theory::{
    chord::Chord,
    interval::Interval,
    note::{Note, NoteLetter},
    quality::Quality,
    scale::{Scale, ScaleType},
};
use rand::{rng, seq::IndexedRandom};

pub struct DiatonicConfig {
    pub scale: Scale,
    pub is_random: bool,
    next_scale_interval: Interval,
    pub current_chord: Chord,
    pub next_chord: Chord,
}

impl DiatonicConfig {
    pub fn set_root(&mut self, root: Note) {
        self.scale = Scale::new(root, ScaleType::Ionian);
        self.current_chord = Chord::new(self.scale.root, Quality::Major);
        self.next_chord = self.preview_next_chord();
    }

    fn preview_next_chord(&self) -> Chord {
        let interval = next_diatonic_scale_interval(self.is_random, &self.scale, &Interval::Unison);
        self.chord_at(interval)
    }

    pub fn reset(&mut self) {
        self.current_chord = Chord::new(self.scale.root, Quality::Major);
        self.next_chord = self.preview_next_chord();
    }

    pub fn generate_next_chord(&mut self) {
        self.current_chord = self.next_chord;
        let interval =
            next_diatonic_scale_interval(self.is_random, &self.scale, &self.next_scale_interval);
        self.next_scale_interval = interval;
        self.next_chord = self.chord_at(interval);
    }

    pub fn get_chords(&self) -> (String, String) {
        (self.current_chord.to_string(), self.next_chord.to_string())
    }

    /// The diatonic triad on the degree at `interval` within the current scale.
    fn chord_at(&self, interval: Interval) -> Chord {
        let degree = self
            .scale
            .intervals
            .iter()
            .position(|i| *i == interval)
            .expect("interval came from this scale");
        self.scale
            .diatonic_triads()
            .expect("the diatonic practice mode always uses a seven-note scale")[degree]
    }
}

impl Default for DiatonicConfig {
    fn default() -> Self {
        let scale = Scale::new(Note::default(), ScaleType::Ionian);
        DiatonicConfig {
            scale: Scale::new(Note::default(), ScaleType::Ionian),
            is_random: false,
            current_chord: Chord::new(scale.root, Quality::Major),
            next_scale_interval: scale.intervals[0],
            next_chord: Chord::new(Note::new(NoteLetter::D, 0), Quality::Minor),
        }
    }
}

fn next_diatonic_scale_interval(
    is_random: bool,
    scale: &Scale,
    current_scale_interval: &Interval,
) -> Interval {
    let mut rand = rng();
    if is_random {
        *scale.intervals.choose(&mut rand).unwrap()
    } else {
        let index = scale
            .intervals
            .iter()
            .position(|f| f == current_scale_interval)
            .unwrap();
        let next_index = (index + 1) % scale.intervals.len();
        scale.intervals[next_index]
    }
}
