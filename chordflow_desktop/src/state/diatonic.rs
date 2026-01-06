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
        self.scale = Scale::new(root, ScaleType::Diatonic);
        self.current_chord = Chord::new(self.scale.root, Quality::Major);
        self.next_chord = self.preview_next_chord();
    }

    fn preview_next_chord(&self) -> Chord {
        let interval = next_diatonic_scale_interval(self.is_random, &self.scale, &Interval::Unison);
        let quality = calculate_chord_quality_in_scale(&self.scale, &interval);

        let next_note = self.scale.root.add_interval(interval);
        Chord::new(next_note, quality)
    }

    pub fn reset(&mut self) {
        self.current_chord = Chord::new(self.scale.root, Quality::Major);
        self.next_chord = self.preview_next_chord();
    }

    pub fn generate_next_chord(&mut self) {
        self.current_chord = self.next_chord;
        let interval =
            next_diatonic_scale_interval(self.is_random, &self.scale, &self.next_scale_interval);
        let quality = calculate_chord_quality_in_scale(&self.scale, &interval);

        let next_note = self.scale.root.add_interval(interval);
        self.next_scale_interval = interval;
        self.next_chord = Chord::new(next_note, quality);
    }

    pub fn get_chords(&self) -> (String, String) {
        (self.current_chord.to_string(), self.next_chord.to_string())
    }
}

impl Default for DiatonicConfig {
    fn default() -> Self {
        let scale = Scale::new(Note::default(), ScaleType::Diatonic);
        DiatonicConfig {
            scale: Scale::new(Note::default(), ScaleType::Diatonic),
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

fn calculate_chord_quality_in_scale(scale: &Scale, interval: &Interval) -> Quality {
    let new_scale_index = scale.intervals.iter().position(|f| f == interval).unwrap() as i32;
    let new_chord_indexes: Vec<i32> =
        vec![new_scale_index, new_scale_index + 2, new_scale_index + 4]
            .into_iter()
            .map(|i| normalize(i, 7))
            .map(|i| scale.intervals[i as usize].to_semitones())
            .collect();
    let zero_based_chord_indexes = new_chord_indexes
        .iter()
        .map(|i| normalize(i - new_chord_indexes[0], 12))
        .collect::<Vec<i32>>();
    Quality::from_intervals(zero_based_chord_indexes)
}

fn normalize(interval: i32, base: i32) -> i32 {
    (interval + base) % base
}
