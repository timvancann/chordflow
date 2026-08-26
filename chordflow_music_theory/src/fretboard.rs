use std::collections::HashMap;

use super::{
    note::{Note, NoteLetter},
    scale::Scale,
};

/// Standard tuning, thinnest string first, so index 0 is the high E and index
/// 5 is the low E. That order matches how a fretboard diagram is drawn.
///
/// Only pitch classes matter here: `Note` carries no octave, and a fret's
/// scale membership does not depend on which E it is.
pub fn standard_tuning() -> Vec<Note> {
    vec![
        Note::new(NoteLetter::E, 0),
        Note::new(NoteLetter::B, 0),
        Note::new(NoteLetter::G, 0),
        Note::new(NoteLetter::D, 0),
        Note::new(NoteLetter::A, 0),
        Note::new(NoteLetter::E, 0),
    ]
}

/// One place a scale degree falls under the hand.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FretPosition {
    /// 0 is the high E string.
    pub string: usize,
    /// 0 is the open string.
    pub fret: usize,
    /// Degree notation, e.g. "R" or "b3".
    pub degree: &'static str,
    /// True for thirds and fifths, however altered — the triad sitting inside
    /// the scale. Marked out on the poster, and the reason a shape is
    /// recognisable at a glance.
    pub is_third_or_fifth: bool,
}

/// Every place `scale` falls on a standard-tuned fretboard, from the open
/// string up to and including `frets`.
///
/// Ordered by string then fret, so a renderer can walk it straight through.
pub fn positions(scale: &Scale, frets: usize) -> Vec<FretPosition> {
    // Pitch class of each degree, so a fret can be looked up directly.
    let by_pitch_class: HashMap<i32, (&'static str, bool)> = scale
        .intervals
        .iter()
        .map(|interval| {
            let pitch_class = scale
                .root
                .add_interval(*interval)
                .to_semitones()
                .rem_euclid(12);
            (
                pitch_class,
                (interval.degree_label(), is_third_or_fifth(*interval)),
            )
        })
        .collect();

    let tuning = standard_tuning();
    let mut found = Vec::new();

    for (string, open) in tuning.iter().enumerate() {
        let open_pitch_class = open.to_semitones().rem_euclid(12);

        for fret in 0..=frets {
            let pitch_class = (open_pitch_class + fret as i32).rem_euclid(12);

            if let Some((degree, is_third_or_fifth)) = by_pitch_class.get(&pitch_class) {
                found.push(FretPosition {
                    string,
                    fret,
                    degree,
                    is_third_or_fifth: *is_third_or_fifth,
                });
            }
        }
    }

    found
}

/// A third or a fifth, whatever its alteration. Derived from the interval's
/// letter-step rather than a list of variants, so `b3`, `3`, `b5`, `5` and
/// `#5` all qualify while `#4` and `#2` do not — which is exactly the
/// distinction the poster draws.
fn is_third_or_fifth(interval: super::interval::Interval) -> bool {
    matches!(interval.to_index(), 2 | 4)
}

#[cfg(test)]
mod tests {
    use strum::IntoEnumIterator;

    use crate::scale::ScaleType;

    use super::*;

    fn at_nut(scale: &Scale) -> Vec<&'static str> {
        let mut open: Vec<(usize, &'static str)> = positions(scale, 12)
            .into_iter()
            .filter(|p| p.fret == 0)
            .map(|p| (p.string, p.degree))
            .collect();
        open.sort_by_key(|(string, _)| *string);
        open.into_iter().map(|(_, degree)| degree).collect()
    }

    #[test]
    fn test_g_ionian_open_strings_match_the_poster() {
        // The poster's fretboard is G, and its nut column reads 6 3 R 5 2 6
        // from the high E down to the low E.
        let g_ionian = Scale::new(Note::new(NoteLetter::G, 0), ScaleType::Ionian);
        assert_eq!(at_nut(&g_ionian), vec!["6", "3", "R", "5", "2", "6"]);
    }

    #[test]
    fn test_e_minor_pentatonic_shape_is_where_a_guitarist_expects() {
        // E aeolian: the low E string should give the root open, and again at
        // the twelfth fret.
        let e_aeolian = Scale::new(Note::new(NoteLetter::E, 0), ScaleType::Aeolian);
        let low_e: Vec<(usize, &str)> = positions(&e_aeolian, 12)
            .into_iter()
            .filter(|p| p.string == 5)
            .map(|p| (p.fret, p.degree))
            .collect();

        assert_eq!(low_e.first(), Some(&(0, "R")));
        assert_eq!(low_e.last(), Some(&(12, "R")));
    }

    #[test]
    fn test_thirds_and_fifths_are_marked_however_altered() {
        let marked = |scale_type: ScaleType| -> Vec<&'static str> {
            let scale = Scale::new(Note::new(NoteLetter::C, 0), scale_type);
            let mut degrees: Vec<&'static str> = scale
                .intervals
                .iter()
                .filter(|i| is_third_or_fifth(**i))
                .map(|i| i.degree_label())
                .collect();
            degrees.dedup();
            degrees
        };

        assert_eq!(marked(ScaleType::Ionian), vec!["3", "5"]);
        assert_eq!(marked(ScaleType::Locrian), vec!["b3", "b5"]);
        // The awkward cases: a sharp fourth is not a fifth, a sharp second is
        // not a third, and a scale can have more than one of each.
        assert_eq!(marked(ScaleType::WholeTone), vec!["3", "#5"]);
        assert_eq!(marked(ScaleType::Augmented), vec!["3", "5", "#5"]);
        assert_eq!(marked(ScaleType::MajorBlues), vec!["b3", "3", "5"]);
    }

    #[test]
    fn test_every_scale_has_a_distinct_pitch_class_per_degree() {
        // The pitch-class lookup would silently drop a degree if two shared a
        // pitch class. No catalog scale does, and this pins that.
        for scale_type in ScaleType::iter() {
            let scale = Scale::new(Note::new(NoteLetter::C, 0), scale_type);
            let mut pitch_classes: Vec<i32> = scale
                .notes()
                .iter()
                .map(|n| n.to_semitones().rem_euclid(12))
                .collect();
            let before = pitch_classes.len();
            pitch_classes.sort();
            pitch_classes.dedup();
            assert_eq!(
                before,
                pitch_classes.len(),
                "{scale_type} repeats a pitch class"
            );
        }
    }

    #[test]
    fn test_every_catalog_scale_covers_the_whole_neck() {
        for scale_type in ScaleType::iter() {
            let scale = Scale::new(Note::new(NoteLetter::G, 0), scale_type);
            let found = positions(&scale, 14);

            assert!(!found.is_empty(), "{scale_type} produced no positions");
            assert!(
                found.iter().all(|p| p.string < 6 && p.fret <= 14),
                "{scale_type} produced an off-board position"
            );
            // An octave of frets must repeat: whatever is open is also at 12.
            for string in 0..6 {
                let open: Vec<&str> = found
                    .iter()
                    .filter(|p| p.string == string && p.fret == 0)
                    .map(|p| p.degree)
                    .collect();
                let twelfth: Vec<&str> = found
                    .iter()
                    .filter(|p| p.string == string && p.fret == 12)
                    .map(|p| p.degree)
                    .collect();
                assert_eq!(open, twelfth, "{scale_type} string {string}");
            }
        }
    }
}
