use std::fmt::{self, Display};

use super::{note::Note, quality::Quality};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Chord {
    pub root: Note,
    pub quality: Quality,
}

impl Display for Chord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.root, self.quality)
    }
}

impl Chord {
    pub fn new(root: Note, quality: Quality) -> Chord {
        Chord { root, quality }
    }

    pub fn to_c_based_semitones(self) -> Vec<i32> {
        let root_semitones = self.root.to_semitones();
        let mut semitones = vec![];

        for interval in self.quality.to_intervals().iter().map(|i| i.to_semitones()) {
            semitones.push(root_semitones + interval);
        }

        semitones
            .into_iter()
            .map(normalize_semitone_within_octave)
            .collect()
    }
}

fn normalize_semitone_within_octave(i: i32) -> i32 {
    if i < 0 {
        return normalize_semitone_within_octave(i + 12);
    }

    if i > 0 {
        return i % 12;
    }

    0
}
