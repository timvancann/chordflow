use std::fmt::Display;

use strum::{AsRefStr, EnumCount, EnumIter, FromRepr};

use super::interval::Interval;

#[derive(Default, Clone, Copy, Debug, EnumIter, AsRefStr, PartialEq, EnumCount, FromRepr, Eq)]
pub enum Quality {
    #[default]
    Major,
    Minor,
    Diminished,
    Augmented,
    Dominant,
    MajorSeventh,
    MinorSeventh,
    HalfDiminished,
    DiminishedSeventh,
    MinorMajorSeventh,
    AugmentedMajorSeventh,
}

/// How chord symbols are written.
///
/// The same chord has several conventional spellings, and which one reads
/// naturally depends on where you learned. This is a display concern only:
/// nothing about a chord's identity changes with it.
#[derive(Default, Clone, Copy, Debug, EnumIter, AsRefStr, PartialEq, Eq)]
pub enum Notation {
    /// C, Cm, C°, C+, C7, C△7, Cm7, Cø7, C°7, Cm△7, C+△7
    #[default]
    Symbolic,
    /// C, Cm, Cdim, Caug, C7, Cmaj7, Cm7, Cm7♭5, Cdim7, CmMaj7, Cmaj7♯5
    Common,
}

impl Notation {
    pub fn display_name(self) -> &'static str {
        match self {
            Notation::Symbolic => "symbolic",
            Notation::Common => "common",
        }
    }
}

impl Display for Notation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

/// Renders as the default notation. Anything the user sees should call
/// `symbol` with their chosen notation instead; this exists for logs, map
/// keys and tests, where a stable canonical form is what you want.
impl Display for Quality {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.symbol(Notation::default()))
    }
}

impl Quality {
    /// The suffix written after the root, e.g. the `m7` of `Cm7`.
    ///
    /// A major triad has no suffix in either notation, which is why callers
    /// that show a bare symbol use a worked example on C instead.
    pub fn symbol(self, notation: Notation) -> &'static str {
        match notation {
            Notation::Symbolic => match self {
                Quality::Major => "",
                Quality::Minor => "m",
                Quality::Diminished => "\u{b0}",
                Quality::Augmented => "+",
                Quality::Dominant => "7",
                Quality::MajorSeventh => "\u{25b3}7",
                Quality::MinorSeventh => "m7",
                Quality::HalfDiminished => "\u{f8}7",
                Quality::DiminishedSeventh => "\u{b0}7",
                Quality::MinorMajorSeventh => "m\u{25b3}7",
                Quality::AugmentedMajorSeventh => "+\u{25b3}7",
            },
            Notation::Common => match self {
                Quality::Major => "",
                Quality::Minor => "m",
                Quality::Diminished => "dim",
                Quality::Augmented => "aug",
                Quality::Dominant => "7",
                Quality::MajorSeventh => "maj7",
                Quality::MinorSeventh => "m7",
                Quality::HalfDiminished => "m7\u{266d}5",
                Quality::DiminishedSeventh => "dim7",
                Quality::MinorMajorSeventh => "mMaj7",
                Quality::AugmentedMajorSeventh => "maj7\u{266f}5",
            },
        }
    }

    pub fn from_string(quality: &str) -> Quality {
        match quality {
            "" => Quality::Major,
            "m" => Quality::Minor,
            "-" => Quality::Minor,
            "o" => Quality::Diminished,
            "dim" => Quality::Diminished,
            "+" => Quality::Augmented,
            "aug" => Quality::Augmented,
            "7" => Quality::Dominant,
            "maj7" => Quality::MajorSeventh,
            "m7" => Quality::MinorSeventh,
            "m7b5" => Quality::HalfDiminished,
            "dim7" => Quality::DiminishedSeventh,
            "o7" => Quality::DiminishedSeventh,
            "mMaj7" => Quality::MinorMajorSeventh,
            "maj7#5" => Quality::AugmentedMajorSeventh,
            _ => Quality::Major,
        }
    }

    pub fn from_name(name: &str) -> Quality {
        match name {
            "Major" => Quality::Major,
            "Minor" => Quality::Minor,
            "Diminished" => Quality::Diminished,
            "Augmented" => Quality::Augmented,
            "Dominant" => Quality::Dominant,
            "Minor Seventh" => Quality::MinorSeventh,
            "Major Seventh" => Quality::MajorSeventh,
            "Half Diminished" => Quality::HalfDiminished,
            "Diminished Seventh" => Quality::DiminishedSeventh,
            "Minor Major Seventh" => Quality::MinorMajorSeventh,
            "Augmented Major Seventh" => Quality::AugmentedMajorSeventh,
            _ => Quality::Major,
        }
    }

    pub fn name(&self) -> String {
        match self {
            Quality::Major => "Major",
            Quality::Minor => "Minor",
            Quality::Diminished => "Diminished",
            Quality::Augmented => "Augmented",
            Quality::Dominant => "Dominant",
            Quality::MinorSeventh => "Minor Seventh",
            Quality::MajorSeventh => "Major Seventh",
            Quality::HalfDiminished => "Half Diminished",
            Quality::DiminishedSeventh => "Diminished Seventh",
            Quality::MinorMajorSeventh => "Minor Major Seventh",
            Quality::AugmentedMajorSeventh => "Augmented Major Seventh",
        }
        .into()
    }

    pub fn to_intervals(self) -> Vec<Interval> {
        use Interval::*;
        match self {
            Quality::Major => vec![Unison, MajorThird, PerfectFifth],
            Quality::Minor => vec![Unison, MinorThird, PerfectFifth],
            Quality::Diminished => vec![Unison, MinorThird, DiminishedFifth],
            Quality::Augmented => vec![Unison, MajorThird, AugmentedFifth],
            Quality::Dominant => vec![Unison, MajorThird, PerfectFifth, MinorSeventh],
            Quality::MajorSeventh => vec![Unison, MajorThird, PerfectFifth, MajorSeventh],
            Quality::MinorSeventh => vec![Unison, MinorThird, PerfectFifth, MinorSeventh],
            Quality::HalfDiminished => vec![Unison, MinorThird, DiminishedFifth, MinorSeventh],
            Quality::DiminishedSeventh => {
                vec![Unison, MinorThird, DiminishedFifth, DiminishedSeventh]
            }
            Quality::MinorMajorSeventh => vec![Unison, MinorThird, PerfectFifth, MajorSeventh],
            Quality::AugmentedMajorSeventh => {
                vec![Unison, MajorThird, AugmentedFifth, MajorSeventh]
            }
        }
    }

    /// Names a root-relative semitone set, or `None` if no conventional
    /// chord symbol describes it. Returning `None` rather than guessing is
    /// deliberate: stacking thirds through the exotic scales produces sets
    /// that no symbol names, and a wrong chord symbol is worse than none.
    pub fn from_intervals(intervals: Vec<i32>) -> Option<Quality> {
        match intervals[..] {
            [0, 4, 7] => Some(Quality::Major),
            [0, 3, 7] => Some(Quality::Minor),
            [0, 3, 6] => Some(Quality::Diminished),
            [0, 4, 8] => Some(Quality::Augmented),
            [0, 4, 7, 10] => Some(Quality::Dominant),
            [0, 3, 7, 10] => Some(Quality::MinorSeventh),
            [0, 4, 7, 11] => Some(Quality::MajorSeventh),
            [0, 3, 6, 10] => Some(Quality::HalfDiminished),
            [0, 3, 6, 9] => Some(Quality::DiminishedSeventh),
            [0, 3, 7, 11] => Some(Quality::MinorMajorSeventh),
            [0, 4, 8, 11] => Some(Quality::AugmentedMajorSeventh),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use strum::IntoEnumIterator;

    use super::Quality;

    #[test]
    fn test_from_string_maps_every_symbol_it_claims_to_support() {
        let cases = [
            ("", Quality::Major),
            ("m", Quality::Minor),
            ("-", Quality::Minor),
            ("o", Quality::Diminished),
            ("dim", Quality::Diminished),
            ("+", Quality::Augmented),
            ("aug", Quality::Augmented),
            ("7", Quality::Dominant),
            ("maj7", Quality::MajorSeventh),
            // Regression: "m7" used to map to MajorSeventh.
            ("m7", Quality::MinorSeventh),
            ("m7b5", Quality::HalfDiminished),
            ("dim7", Quality::DiminishedSeventh),
            ("o7", Quality::DiminishedSeventh),
            ("mMaj7", Quality::MinorMajorSeventh),
            ("maj7#5", Quality::AugmentedMajorSeventh),
        ];
        for (symbol, quality) in cases {
            assert_eq!(Quality::from_string(symbol), quality, "symbol {symbol:?}");
        }
    }

    #[test]
    fn test_new_qualities_round_trip_through_intervals() {
        let cases = [
            (Quality::DiminishedSeventh, vec![0, 3, 6, 9]),
            (Quality::MinorMajorSeventh, vec![0, 3, 7, 11]),
            (Quality::AugmentedMajorSeventh, vec![0, 4, 8, 11]),
        ];
        for (quality, semitones) in cases {
            assert_eq!(
                Quality::from_intervals(semitones.clone()),
                Some(quality),
                "{semitones:?} should name {quality:?}"
            );
            let round_tripped: Vec<i32> = quality
                .to_intervals()
                .iter()
                .map(|i| i.to_semitones())
                .collect();
            assert_eq!(round_tripped, semitones, "{quality:?} to_intervals");
        }
    }

    #[test]
    fn test_augmented_triad_is_recognised() {
        // Regression: this used to be mapped from [0, 5, 7], which is a sus4,
        // and to_intervals disagreed with from_intervals as a result.
        assert_eq!(
            Quality::from_intervals(vec![0, 4, 8]),
            Some(Quality::Augmented)
        );
        let augmented: Vec<i32> = Quality::Augmented
            .to_intervals()
            .iter()
            .map(|i| i.to_semitones())
            .collect();
        assert_eq!(augmented, vec![0, 4, 8]);
    }

    #[test]
    fn test_unnameable_sets_return_none_instead_of_panicking() {
        assert_eq!(Quality::from_intervals(vec![0, 5, 7]), None);
        assert_eq!(Quality::from_intervals(vec![0, 1, 4, 6]), None);
        assert_eq!(Quality::from_intervals(vec![]), None);
    }

    #[test]
    fn test_to_intervals_semitones_are_unchanged_for_every_quality() {
        // Guards against the correctly-spelled interval lists silently
        // drifting away from the semitone sets the audio path relies on.
        let cases = [
            (Quality::Major, vec![0, 4, 7]),
            (Quality::Minor, vec![0, 3, 7]),
            (Quality::Diminished, vec![0, 3, 6]),
            (Quality::Augmented, vec![0, 4, 8]),
            (Quality::Dominant, vec![0, 4, 7, 10]),
            (Quality::MajorSeventh, vec![0, 4, 7, 11]),
            (Quality::MinorSeventh, vec![0, 3, 7, 10]),
            (Quality::HalfDiminished, vec![0, 3, 6, 10]),
            (Quality::DiminishedSeventh, vec![0, 3, 6, 9]),
            (Quality::MinorMajorSeventh, vec![0, 3, 7, 11]),
            (Quality::AugmentedMajorSeventh, vec![0, 4, 8, 11]),
        ];
        assert_eq!(
            cases.len(),
            Quality::iter().count(),
            "every Quality variant is covered"
        );
        for (quality, expected) in cases {
            let actual: Vec<i32> = quality
                .to_intervals()
                .iter()
                .map(|i| i.to_semitones())
                .collect();
            assert_eq!(actual, expected, "{quality:?} to_intervals semitones");
        }
    }

    #[test]
    fn test_existing_qualities_still_map() {
        assert_eq!(Quality::from_intervals(vec![0, 4, 7]), Some(Quality::Major));
        assert_eq!(Quality::from_intervals(vec![0, 3, 7]), Some(Quality::Minor));
        assert_eq!(
            Quality::from_intervals(vec![0, 3, 6]),
            Some(Quality::Diminished)
        );
        assert_eq!(
            Quality::from_intervals(vec![0, 4, 7, 10]),
            Some(Quality::Dominant)
        );
        assert_eq!(
            Quality::from_intervals(vec![0, 3, 7, 10]),
            Some(Quality::MinorSeventh)
        );
        assert_eq!(
            Quality::from_intervals(vec![0, 4, 7, 11]),
            Some(Quality::MajorSeventh)
        );
        assert_eq!(
            Quality::from_intervals(vec![0, 3, 6, 10]),
            Some(Quality::HalfDiminished)
        );
    }
}
