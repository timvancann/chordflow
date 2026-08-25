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

    /// The chord's tones, spelled. A C augmented triad is `C E G♯`, not
    /// `C E A♭`, because `Quality::to_intervals` names an augmented fifth
    /// rather than a minor sixth.
    pub fn notes(self) -> Vec<Note> {
        self.quality
            .to_intervals()
            .into_iter()
            .map(|interval| self.root.add_interval(interval))
            .collect()
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

#[cfg(test)]
mod tests {
    use crate::{
        note::{Note, NoteLetter},
        quality::Quality,
    };

    use super::Chord;

    fn spell(letter: NoteLetter, accidentals: i32, quality: Quality) -> String {
        Chord::new(Note::new(letter, accidentals), quality)
            .notes()
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    }

    #[test]
    fn test_plain_chords_spell_the_obvious_way() {
        assert_eq!(spell(NoteLetter::C, 0, Quality::Major), "C E G");
        assert_eq!(spell(NoteLetter::C, 0, Quality::Minor), "C E\u{266d} G");
        assert_eq!(
            spell(NoteLetter::C, 0, Quality::Dominant),
            "C E G B\u{266d}"
        );
    }

    #[test]
    fn test_altered_fifths_and_sevenths_spell_correctly() {
        // The augmented triad's fifth is a sharp five, not a flat six.
        assert_eq!(spell(NoteLetter::C, 0, Quality::Augmented), "C E G\u{266f}");
        // The diminished seventh's seventh is doubly flattened.
        assert_eq!(
            spell(NoteLetter::C, 0, Quality::DiminishedSeventh),
            "C E\u{266d} G\u{266d} B\u{266d}\u{266d}"
        );
        assert_eq!(
            spell(NoteLetter::C, 0, Quality::HalfDiminished),
            "C E\u{266d} G\u{266d} B\u{266d}"
        );
    }

    #[test]
    fn test_spelling_follows_the_root() {
        assert_eq!(
            spell(NoteLetter::F, 1, Quality::MajorSeventh),
            "F\u{266f} A\u{266f} C\u{266f} E\u{266f}"
        );
    }
}
