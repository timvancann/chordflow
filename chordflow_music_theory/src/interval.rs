use std::fmt::Display;

use strum::{AsRefStr, EnumCount, EnumIter, FromRepr};

#[derive(
    Default, Clone, Copy, Debug, EnumIter, AsRefStr, PartialEq, EnumCount, FromRepr, Eq,
)]
pub enum Interval {
    #[default]
    Unison,
    MinorSecond,
    MajorSecond,
    AugmentedSecond,
    MinorThird,
    MajorThird,
    DiminishedFourth,
    PerfectFourth,
    AugmentedFourth,
    Tritone,
    DiminishedFifth,
    PerfectFifth,
    AugmentedFifth,
    MinorSixth,
    MajorSixth,
    DiminishedSeventh,
    MinorSeventh,
    MajorSeventh,
    Octave,
}

impl Interval {
    pub fn to_semitones(self) -> i32 {
        match self {
            Interval::Unison => 0,
            Interval::MinorSecond => 1,
            Interval::MajorSecond => 2,
            Interval::AugmentedSecond => 3,
            Interval::MinorThird => 3,
            Interval::MajorThird => 4,
            Interval::DiminishedFourth => 4,
            Interval::PerfectFourth => 5,
            Interval::AugmentedFourth => 6,
            Interval::Tritone => 6,
            Interval::DiminishedFifth => 6,
            Interval::PerfectFifth => 7,
            Interval::AugmentedFifth => 8,
            Interval::MinorSixth => 8,
            Interval::MajorSixth => 9,
            Interval::DiminishedSeventh => 9,
            Interval::MinorSeventh => 10,
            Interval::MajorSeventh => 11,
            Interval::Octave => 12,
        }
    }
    pub fn from_semitone(semitone: i32) -> Self {
        match semitone {
            0 => Interval::Unison,
            1 => Interval::MinorSecond,
            2 => Interval::MajorSecond,
            3 => Interval::MinorThird,
            4 => Interval::MajorThird,
            5 => Interval::PerfectFourth,
            6 => Interval::Tritone,
            7 => Interval::PerfectFifth,
            8 => Interval::MinorSixth,
            9 => Interval::MajorSixth,
            10 => Interval::MinorSeventh,
            11 => Interval::MajorSeventh,
            _ => panic!("Invalid semitone"),
        }
    }

    pub fn to_index(self) -> i32 {
        match self {
            Interval::Unison => 0,
            Interval::MinorSecond => 1,
            Interval::MajorSecond => 1,
            Interval::AugmentedSecond => 1,
            Interval::MinorThird => 2,
            Interval::MajorThird => 2,
            Interval::DiminishedFourth => 3,
            Interval::PerfectFourth => 3,
            Interval::AugmentedFourth => 3,
            Interval::Tritone => 3,
            Interval::DiminishedFifth => 4,
            Interval::PerfectFifth => 4,
            Interval::AugmentedFifth => 4,
            Interval::MinorSixth => 5,
            Interval::MajorSixth => 5,
            Interval::DiminishedSeventh => 6,
            Interval::MinorSeventh => 6,
            Interval::MajorSeventh => 6,
            _ => panic!("Invalid interval"),
        }
    }

    /// The scale-degree notation used on the Scales.pdf poster.
    ///
    /// `Octave` labels as `"R"` here, but it must not be passed to
    /// scale/spelling code: `to_index` panics on it, so
    /// `Note::add_interval(Octave)` panics too.
    pub fn degree_label(self) -> &'static str {
        match self {
            Interval::Unison | Interval::Octave => "R",
            Interval::MinorSecond => "b2",
            Interval::MajorSecond => "2",
            Interval::AugmentedSecond => "#2",
            Interval::MinorThird => "b3",
            Interval::MajorThird => "3",
            Interval::DiminishedFourth => "b4",
            Interval::PerfectFourth => "4",
            Interval::AugmentedFourth | Interval::Tritone => "#4",
            Interval::DiminishedFifth => "b5",
            Interval::PerfectFifth => "5",
            Interval::AugmentedFifth => "#5",
            Interval::MinorSixth => "b6",
            Interval::MajorSixth => "6",
            Interval::DiminishedSeventh => "bb7",
            Interval::MinorSeventh => "b7",
            Interval::MajorSeventh => "7",
        }
    }

    pub fn from_semitones(semitones: Vec<i32>) -> Vec<Self> {
        semitones
            .iter()
            .map(|&x| Interval::from_semitone(x))
            .collect()
    }
}

impl Display for Interval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.degree_label())
    }
}

#[cfg(test)]
mod tests {
    use super::Interval;

    #[test]
    fn test_new_variants_semitones_and_letter_steps() {
        let cases = [
            (Interval::AugmentedSecond, 3, 1),
            (Interval::DiminishedFourth, 4, 3),
            (Interval::AugmentedFifth, 8, 4),
            (Interval::DiminishedSeventh, 9, 6),
        ];
        for (interval, semitones, letter_step) in cases {
            assert_eq!(interval.to_semitones(), semitones, "{interval:?} semitones");
            assert_eq!(interval.to_index(), letter_step, "{interval:?} letter step");
        }
    }

    #[test]
    fn test_existing_variants_are_unchanged() {
        assert_eq!(Interval::MinorThird.to_semitones(), 3);
        assert_eq!(Interval::MinorThird.to_index(), 2);
        assert_eq!(Interval::AugmentedFourth.to_semitones(), 6);
        assert_eq!(Interval::AugmentedFourth.to_index(), 3);
        assert_eq!(Interval::DiminishedFifth.to_semitones(), 6);
        assert_eq!(Interval::DiminishedFifth.to_index(), 4);
        assert_eq!(Interval::MinorSixth.to_semitones(), 8);
        assert_eq!(Interval::MinorSixth.to_index(), 5);
    }

    #[test]
    fn test_degree_labels_match_the_poster() {
        let cases = [
            (Interval::Unison, "R"),
            (Interval::MinorSecond, "b2"),
            (Interval::MajorSecond, "2"),
            (Interval::AugmentedSecond, "#2"),
            (Interval::MinorThird, "b3"),
            (Interval::MajorThird, "3"),
            (Interval::DiminishedFourth, "b4"),
            (Interval::PerfectFourth, "4"),
            (Interval::AugmentedFourth, "#4"),
            (Interval::DiminishedFifth, "b5"),
            (Interval::PerfectFifth, "5"),
            (Interval::AugmentedFifth, "#5"),
            (Interval::MinorSixth, "b6"),
            (Interval::MajorSixth, "6"),
            (Interval::DiminishedSeventh, "bb7"),
            (Interval::MinorSeventh, "b7"),
            (Interval::MajorSeventh, "7"),
        ];
        for (interval, label) in cases {
            assert_eq!(interval.degree_label(), label, "{interval:?}");
        }
    }

    #[test]
    fn test_display_delegates_to_degree_label() {
        assert_eq!(Interval::MajorThird.to_string(), "3");
        assert_eq!(Interval::AugmentedFifth.to_string(), "#5");
        assert_eq!(Interval::DiminishedSeventh.to_string(), "bb7");
    }
}
