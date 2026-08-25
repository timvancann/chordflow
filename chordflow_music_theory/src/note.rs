use std::fmt::Display;

use itertools::Itertools;
use strum::{AsRefStr, Display, EnumCount, EnumIter, FromRepr, IntoEnumIterator};

use super::{accidental::Accidental, interval::Interval};

#[derive(
    Default, Clone, Copy, Debug, EnumIter, AsRefStr, PartialEq, EnumCount, FromRepr, Eq, Display,
)]
pub enum NoteLetter {
    #[default]
    C,
    D,
    E,
    F,
    G,
    A,
    B,
}

impl NoteLetter {
    pub fn to_index(self) -> i32 {
        match self {
            NoteLetter::C => 0,
            NoteLetter::D => 1,
            NoteLetter::E => 2,
            NoteLetter::F => 3,
            NoteLetter::G => 4,
            NoteLetter::A => 5,
            NoteLetter::B => 6,
        }
    }
    pub fn from_letter_index(idx: i32) -> Self {
        match idx {
            0 => NoteLetter::C,
            1 => NoteLetter::D,
            2 => NoteLetter::E,
            3 => NoteLetter::F,
            4 => NoteLetter::G,
            5 => NoteLetter::A,
            6 => NoteLetter::B,
            _ => panic!("Invalid note index"),
        }
    }
    pub fn to_semitones(self) -> i32 {
        match self {
            NoteLetter::C => 0,
            NoteLetter::D => 2,
            NoteLetter::E => 4,
            NoteLetter::F => 5,
            NoteLetter::G => 7,
            NoteLetter::A => 9,
            NoteLetter::B => 11,
        }
    }

    pub fn from_semitone(semitone: i32) -> Self {
        match semitone % 12 {
            0 => NoteLetter::C,
            2 => NoteLetter::D,
            4 => NoteLetter::E,
            5 => NoteLetter::F,
            7 => NoteLetter::G,
            9 => NoteLetter::A,
            11 => NoteLetter::B,
            _ => panic!("Invalid semitone"),
        }
    }

    pub fn from_string(s: &str) -> NoteLetter {
        match s.to_uppercase().as_str() {
            "C" => NoteLetter::C,
            "D" => NoteLetter::D,
            "E" => NoteLetter::E,
            "F" => NoteLetter::F,
            "G" => NoteLetter::G,
            "A" => NoteLetter::A,
            "B" => NoteLetter::B,
            _ => panic!("Invalid note letter"),
        }
    }
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct Note {
    pub letter: NoteLetter,
    pub accidentals: i32,
}

impl Display for Note {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut accidendal = "".to_string();
        if self.accidentals > 0 {
            accidendal = "♯".repeat(self.accidentals as usize)
        }
        if self.accidentals < 0 {
            accidendal = "♭".repeat(-self.accidentals as usize)
        };
        write!(f, "{}{}", self.letter, accidendal)
    }
}

impl Note {
    pub fn new(letter: NoteLetter, accidentals: i32) -> Note {
        Note {
            letter,
            accidentals,
        }
    }
    pub fn to_semitones(self) -> i32 {
        self.letter.to_semitones() + self.accidentals
    }

    pub fn add_interval(&self, interval: Interval) -> Note {
        let new_semitones =
            (self.letter.to_semitones() + self.accidentals + interval.to_semitones())
                .rem_euclid(12);
        let new_letter_index = (self.letter.to_index() + interval.to_index()) % 7;
        let new_letter = NoteLetter::from_letter_index(new_letter_index);

        let mut remaining_semitones = new_semitones - new_letter.to_semitones();
        if remaining_semitones > 6 {
            remaining_semitones -= 12;
        } else if remaining_semitones < -6 {
            remaining_semitones += 12;
        }
        Note::new(new_letter, remaining_semitones)
    }
}

pub fn generate_all_roots() -> Vec<Note> {
    NoteLetter::iter()
        .cartesian_product(Accidental::iter())
        .filter(|(note, accidental)| {
            let is_b_sharp = note == &NoteLetter::B && accidental == &Accidental::Sharp;
            let is_c_flat = note == &NoteLetter::C && accidental == &Accidental::Flat;
            let is_e_sharp = note == &NoteLetter::E && accidental == &Accidental::Sharp;
            let is_f_flat = note == &NoteLetter::F && accidental == &Accidental::Flat;
            !is_c_flat && !is_e_sharp && !is_b_sharp && !is_f_flat
        })
        .map(|(note, accidental)| Note::new(note, accidental.to_semitones()))
        .collect()
}

/// The twelve keys guitarists actually read in, one spelling per pitch class,
/// ordered around the circle of fifths from C.
///
/// `generate_all_roots` returns seventeen spellings because it is a cartesian
/// product: C# and Db both appear, as do D#, G#, and A#, which produce
/// unreadable scales (A# ionian is A# B# C## D# E# F## G##). This is the
/// curated set for anything a person has to read.
pub fn practical_keys() -> Vec<Note> {
    vec![
        Note::new(NoteLetter::C, 0),
        Note::new(NoteLetter::G, 0),
        Note::new(NoteLetter::D, 0),
        Note::new(NoteLetter::A, 0),
        Note::new(NoteLetter::E, 0),
        Note::new(NoteLetter::B, 0),
        Note::new(NoteLetter::F, 1),
        Note::new(NoteLetter::D, -1),
        Note::new(NoteLetter::A, -1),
        Note::new(NoteLetter::E, -1),
        Note::new(NoteLetter::B, -1),
        Note::new(NoteLetter::F, 0),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_practical_keys_covers_each_pitch_class_once() {
        let keys = practical_keys();
        assert_eq!(keys.len(), 12);

        let mut pitch_classes: Vec<i32> = keys
            .iter()
            .map(|n| n.to_semitones().rem_euclid(12))
            .collect();
        pitch_classes.sort();
        assert_eq!(pitch_classes, (0..12).collect::<Vec<i32>>());
    }

    #[test]
    fn test_practical_keys_are_spelled_the_readable_way() {
        let spelled: Vec<String> = practical_keys().iter().map(|n| n.to_string()).collect();
        assert_eq!(
            spelled,
            vec![
                "C",
                "G",
                "D",
                "A",
                "E",
                "B",
                "F\u{266f}",
                "D\u{266d}",
                "A\u{266d}",
                "E\u{266d}",
                "B\u{266d}",
                "F"
            ]
        );
    }

    #[test]
    fn test_practical_keys_are_a_subset_of_all_roots() {
        let all = generate_all_roots();
        for key in practical_keys() {
            assert!(
                all.contains(&key),
                "{key} is not a root the crate generates"
            );
        }
    }

    #[test]
    fn test_add_interval() {
        let note = Note::new(NoteLetter::C, 0);
        let intervals = Interval::iter();

        let actual_notes = vec![
            Note::new(NoteLetter::C, 0),  // R
            Note::new(NoteLetter::D, -1), // b2
            Note::new(NoteLetter::D, 0),  // 2
            Note::new(NoteLetter::D, 1),  // #2
            Note::new(NoteLetter::E, -1), // b3
            Note::new(NoteLetter::E, 0),  // 3
            Note::new(NoteLetter::F, -1), // b4
            Note::new(NoteLetter::F, 0),  // 4
            Note::new(NoteLetter::F, 1),  // #4
            Note::new(NoteLetter::F, 1),  // tritone, spelled as #4
            Note::new(NoteLetter::G, -1), // b5
            Note::new(NoteLetter::G, 0),  // 5
            Note::new(NoteLetter::G, 1),  // #5
            Note::new(NoteLetter::A, -1), // b6
            Note::new(NoteLetter::A, 0),  // 6
            Note::new(NoteLetter::B, -2), // bb7
            Note::new(NoteLetter::B, -1), // b7
            Note::new(NoteLetter::B, 0),  // 7
        ];

        for (interval, actual) in intervals.zip(actual_notes) {
            let new_note = note.add_interval(interval);
            assert_eq!(new_note, actual);
        }
        assert_eq!(
            Note::new(NoteLetter::F, 1).add_interval(Interval::PerfectFifth),
            Note::new(NoteLetter::C, 1)
        )
    }

    #[test]
    fn test_add_interval_normalises_wraps_that_overshoot_flat() {
        // Gb ionian's 4th degree is a perfect-fourth-shaped step from Gb
        // that used to compute as C########### instead of Cb.
        use crate::scale::{Scale, ScaleType};

        let scale = Scale::new(Note::new(NoteLetter::G, -1), ScaleType::Ionian);
        assert_eq!(
            scale.notes(),
            vec![
                Note::new(NoteLetter::G, -1),
                Note::new(NoteLetter::A, -1),
                Note::new(NoteLetter::B, -1),
                Note::new(NoteLetter::C, -1),
                Note::new(NoteLetter::D, -1),
                Note::new(NoteLetter::E, -1),
                Note::new(NoteLetter::F, 0),
            ]
        );
    }

    #[test]
    fn test_add_interval_normalises_wraps_in_whole_tone() {
        use crate::scale::{Scale, ScaleType};

        let scale = Scale::new(Note::new(NoteLetter::D, -1), ScaleType::WholeTone);
        assert_eq!(
            scale.notes(),
            vec![
                Note::new(NoteLetter::D, -1),
                Note::new(NoteLetter::E, -1),
                Note::new(NoteLetter::F, 0),
                Note::new(NoteLetter::G, 0),
                Note::new(NoteLetter::A, 0),
                Note::new(NoteLetter::C, -1),
            ]
        );
    }

    #[test]
    fn test_add_interval_normalises_wraps_that_overshoot_sharp() {
        // C#'s major seventh is B#, not a letter-C spelling with a pile of
        // sharps: the letter wraps around the B/C boundary the other way.
        let csharp = Note::new(NoteLetter::C, 1);
        assert_eq!(
            csharp.add_interval(Interval::MajorSeventh),
            Note::new(NoteLetter::B, 1)
        );
    }
}
