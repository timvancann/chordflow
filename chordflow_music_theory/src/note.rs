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

    pub fn from_string(s: &str) -> NoteLetter {
        match s {
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
            accidendal = "#".repeat(self.accidentals as usize)
        }
        if self.accidentals < 0 {
            accidendal = "b".repeat(-self.accidentals as usize)
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
            (self.letter.to_semitones() + self.accidentals + interval.to_semitones()) % 12;
        let new_letter_index = (self.letter.to_index() + interval.to_index()) % 7;
        let new_letter = NoteLetter::from_letter_index(new_letter_index);

        let remaining_semitones = new_semitones - new_letter.to_semitones();
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_interval() {
        let note = Note::new(NoteLetter::C, 0);
        let intervals = Interval::iter();

        let actual_notes = vec![
            Note::new(NoteLetter::C, 0),
            Note::new(NoteLetter::D, -1),
            Note::new(NoteLetter::D, 0),
            Note::new(NoteLetter::E, -1),
            Note::new(NoteLetter::E, 0),
            Note::new(NoteLetter::F, 0),
            Note::new(NoteLetter::F, 1),
            Note::new(NoteLetter::F, 1),
            Note::new(NoteLetter::G, -1),
            Note::new(NoteLetter::G, 0),
            Note::new(NoteLetter::A, -1),
            Note::new(NoteLetter::A, 0),
            Note::new(NoteLetter::B, -1),
            Note::new(NoteLetter::B, 0),
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
}
