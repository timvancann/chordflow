use std::ops::Not;

use chordflow_music_theory::{
    chord::Chord,
    interval::Interval,
    note::{generate_all_roots, Note, NoteLetter},
    quality::Quality,
    scale::Scale,
    util::random_chord,
};
use rand::{rng, seq::IndexedRandom};
use strum::IntoEnumIterator;

use crate::{mode::Mode, progression::Progression, DiatonicOption};

#[derive(Clone, PartialEq)]
pub struct ConfigState {
    pub fourths_selected_quality: Quality,
    pub progression: Option<Progression>,
    pub random_selected_qualities: Vec<Quality>,
    pub diatonic_root: Note,
    pub diatonic_option: DiatonicOption,
}
impl Default for ConfigState {
    fn default() -> Self {
        ConfigState {
            fourths_selected_quality: Quality::default(),
            progression: Option::default(),
            random_selected_qualities: Quality::iter().collect(),
            diatonic_root: Note::default(),
            diatonic_option: DiatonicOption::default(),
        }
    }
}

#[derive(PartialEq, Clone)]
pub struct PracticState {
    pub current_chord: Chord,
    pub next_chord: Chord,
    pub mode: Mode,
    pub next_scale_interval: Interval,
    pub next_progression_chord_idx: usize,
}

impl PracticState {
    pub fn set_mode(&mut self, mode: Mode) -> bool {
        if mode == self.mode {
            return false;
        }
        self.mode = mode;
        self.reset();
        true
    }

    pub fn next_chord(&mut self) {
        self.current_chord = self.next_chord;
        self.next_chord = self
            .generate_next_chord(self.current_chord, self.mode.clone())
            .unwrap();
    }

    pub fn generate_next_chord(&mut self, chord: Chord, mode: Mode) -> Option<Chord> {
        match mode {
            Mode::Fourths(_) => {
                let mut next_note = chord.root.add_interval(Interval::PerfectFourth);
                if next_note == Note::new(NoteLetter::G, -1) {
                    next_note = Note::new(NoteLetter::F, 1);
                }
                Some(Chord::new(next_note, chord.quality))
            }
            Mode::Random(qualities) => {
                if qualities.is_empty() {
                    Some(random_chord(None))
                } else {
                    Some(random_chord(Some(qualities)))
                }
            }
            Mode::Custom(Some(progression)) => {
                self.next_progression_chord_idx =
                    (self.next_progression_chord_idx + 1) % progression.chords.len();
                Some(progression.chords[self.next_progression_chord_idx].chord)
            }
            Mode::Custom(None) => None,
            Mode::Diatonic(scale, option) => {
                let interval =
                    next_diatonic_scale_interval(&option, &scale, &self.next_scale_interval);
                let quality = calculate_chord_quality_in_scale(&scale, &interval);

                let next_note = scale.root.add_interval(interval);
                self.next_scale_interval = interval;
                Some(Chord::new(next_note, quality))
            }
        }
    }

    pub fn reset(&mut self) {
        let mut rand = rng();
        self.current_chord = match &self.mode {
            Mode::Fourths(q) => Chord::new(Note::new(NoteLetter::B, 0), *q),
            Mode::Random(qs) => {
                let note = *generate_all_roots().choose(&mut rand).unwrap();
                Chord::new(note, *qs.choose(&mut rand).unwrap())
            }
            Mode::Diatonic(scale, _) => Chord::new(scale.root, Quality::Major),
            Mode::Custom(p) => p.clone().expect("woops").chords[0].chord,
        };
        self.next_chord = self
            .generate_next_chord(self.current_chord, self.mode.clone())
            .unwrap();
    }
}

fn next_diatonic_scale_interval(
    option: &DiatonicOption,
    scale: &Scale,
    current_scale_interval: &Interval,
) -> Interval {
    let mut rand = rng();
    match option {
        DiatonicOption::Incemental => {
            let index = scale
                .intervals
                .iter()
                .position(|f| f == current_scale_interval)
                .unwrap();
            let next_index = (index + 1) % scale.intervals.len();
            scale.intervals[next_index]
        }
        DiatonicOption::Random => *scale.intervals.choose(&mut rand).unwrap(),
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

impl Default for PracticState {
    fn default() -> Self {
        let mode = Mode::Fourths(Quality::Major);
        let current_chord = Chord::new(Note::new(NoteLetter::B, 0), Quality::Major);
        let next_chord = Chord::new(Note::new(NoteLetter::E, 0), Quality::Major);
        PracticState {
            mode,
            current_chord,
            next_chord,
            next_scale_interval: Interval::Unison,
            next_progression_chord_idx: 0,
        }
    }
}
#[cfg(test)]
mod tests {

    use chordflow_music_theory::scale::ScaleType;

    use super::*;

    #[test]
    fn test_calculate_chord_quality_in_scale() {
        assert_eq!(
            calculate_chord_quality_in_scale(
                &Scale::new(Note::new(NoteLetter::C, 0), ScaleType::Diatonic),
                &Interval::MajorThird
            ),
            Quality::Minor
        );
        assert_eq!(
            calculate_chord_quality_in_scale(
                &Scale::new(Note::new(NoteLetter::F, 1), ScaleType::Diatonic),
                &Interval::PerfectFourth
            ),
            Quality::Major
        );

        let c_major = Scale::new(Note::new(NoteLetter::C, 0), ScaleType::Diatonic);
        let real_qualities = vec![
            Quality::Major,
            Quality::Minor,
            Quality::Minor,
            Quality::Major,
            Quality::Major,
            Quality::Minor,
            Quality::Diminished,
        ];
        for (interval, quality) in c_major.intervals.iter().zip(real_qualities) {
            assert_eq!(
                calculate_chord_quality_in_scale(&c_major, interval),
                quality
            );
        }
    }

    #[test]
    fn test_next_diatonic_chord() {
        let c_major = Scale::new(Note::new(NoteLetter::C, 0), ScaleType::Diatonic);

        let actual_intervals = vec![
            Interval::MajorSecond,
            Interval::MajorThird,
            Interval::PerfectFourth,
            Interval::PerfectFifth,
            Interval::MajorSixth,
            Interval::MajorSeventh,
            Interval::Unison,
        ];
        for (interval, real_interval) in c_major.intervals.iter().zip(actual_intervals) {
            assert_eq!(
                next_diatonic_scale_interval(&DiatonicOption::Incemental, &c_major, interval),
                real_interval
            )
        }
    }
}
