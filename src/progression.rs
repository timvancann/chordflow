use std::fmt::{self, Display};

use regex::Regex;

use anyhow::Result;

use crate::music::{
    accidental::Accidental,
    chord::Chord,
    note::{Note, NoteLetter},
    quality::Quality,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Progression {
    pub chords: Vec<ProgressionChord>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProgressionChord {
    pub chord: Chord,
    pub bars: usize,
}

impl Display for Progression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.chords
                .iter()
                .map(|c| format!("{}x{}", c.bars, c.chord))
                .collect::<Vec<_>>()
                .join(" ")
        )
    }
}

impl ProgressionChord {
    pub fn new(chord: Chord, bars: usize) -> Self {
        Self { chord, bars }
    }

    /// Parse a string into a list of ProgressionChord, tne string should be in the repeated format of
    /// <bars><note><accidental><quality>, where accidental and quality are optional
    /// Examples:
    /// 3C 2Bm 1F#aug
    pub fn from_str(str: String) -> Result<Vec<ProgressionChord>> {
        let re =
            Regex::new(r"(?<n>\d)(?<l>[ABCDEFGabcdefgh])(?<a>[#b])?(?<q>M|m|aug|dim|maj7|7|m7b5)?")
                .unwrap();

        let mut results = vec![];
        for res in re.captures_iter(&str) {
            results.push((
                res.name("n").unwrap().as_str().parse::<usize>().unwrap(),
                res.name("l")
                    .map_or(NoteLetter::C, |l| NoteLetter::from_str(l.as_str())),
                res.name("a")
                    .map_or(Accidental::Natural, |a| Accidental::from_str(a.as_str())),
                res.name("q")
                    .map_or(Quality::Major, |q| Quality::from_str(q.as_str())),
            ));
        }

        if results.is_empty() {
            return Err(anyhow::anyhow!("Invalid progression"));
        }
        Ok(results
            .into_iter()
            .map(|(n, note, accidental, quality)| {
                let note = Note::new(note, accidental.to_semitones());
                ProgressionChord::new(Chord::new(note, quality), n)
            })
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_progression_chord_from_str() {
        let results = ProgressionChord::from_str("3C 2Bm 1F#aug".into());
        assert!(results.is_ok());
        assert_eq!(
            results.unwrap(),
            vec![
                ProgressionChord::new(Chord::new(Note::new(NoteLetter::C, 0), Quality::Major), 3),
                ProgressionChord::new(Chord::new(Note::new(NoteLetter::B, 0), Quality::Minor), 2),
                ProgressionChord::new(
                    Chord::new(Note::new(NoteLetter::F, 1), Quality::Augmented),
                    1
                ),
            ]
        );
    }
}
