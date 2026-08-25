use strum::{AsRefStr, Display, EnumCount, EnumIter, FromRepr};

use super::interval::Interval;

#[derive(
    Default, Clone, Copy, Debug, EnumIter, AsRefStr, PartialEq, EnumCount, FromRepr, Eq, Display,
)]
pub enum Quality {
    #[default]
    #[strum(to_string = "")]
    Major,
    #[strum(to_string = "-")]
    Minor,
    #[strum(to_string = "o")]
    Diminished,
    #[strum(to_string = "+")]
    Augmented,
    #[strum(to_string = "7")]
    Dominant,
    #[strum(to_string = "Δ")]
    MajorSeventh,
    #[strum(to_string = "-7")]
    MinorSeventh,
    #[strum(to_string = "ø")]
    HalfDiminished,
    #[strum(to_string = "o7")]
    DiminishedSeventh,
    #[strum(to_string = "-Δ")]
    MinorMajorSeventh,
    #[strum(to_string = "+Δ")]
    AugmentedMajorSeventh,
}

impl Quality {
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
            "m7" => Quality::MajorSeventh,
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
        match self {
            Quality::Major => Interval::from_semitones([0, 4, 7].to_vec()),
            Quality::Minor => Interval::from_semitones([0, 3, 7].to_vec()),
            Quality::Diminished => Interval::from_semitones([0, 3, 6].to_vec()),
            Quality::Augmented => Interval::from_semitones([0, 4, 8].to_vec()),
            Quality::Dominant => Interval::from_semitones([0, 4, 7, 10].to_vec()),
            Quality::MinorSeventh => Interval::from_semitones([0, 3, 7, 10].to_vec()),
            Quality::MajorSeventh => Interval::from_semitones([0, 4, 7, 11].to_vec()),
            Quality::HalfDiminished => Interval::from_semitones([0, 3, 6, 10].to_vec()),
            Quality::DiminishedSeventh => Interval::from_semitones([0, 3, 6, 9].to_vec()),
            Quality::MinorMajorSeventh => Interval::from_semitones([0, 3, 7, 11].to_vec()),
            Quality::AugmentedMajorSeventh => Interval::from_semitones([0, 4, 8, 11].to_vec()),
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
    use super::Quality;

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
            let round_tripped: Vec<i32> =
                quality.to_intervals().iter().map(|i| i.to_semitones()).collect();
            assert_eq!(round_tripped, semitones, "{quality:?} to_intervals");
        }
    }

    #[test]
    fn test_augmented_triad_is_recognised() {
        // Regression: this used to be mapped from [0, 5, 7], which is a sus4,
        // and to_intervals disagreed with from_intervals as a result.
        assert_eq!(Quality::from_intervals(vec![0, 4, 8]), Some(Quality::Augmented));
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
    fn test_existing_qualities_still_map() {
        assert_eq!(Quality::from_intervals(vec![0, 4, 7]), Some(Quality::Major));
        assert_eq!(Quality::from_intervals(vec![0, 3, 7]), Some(Quality::Minor));
        assert_eq!(Quality::from_intervals(vec![0, 3, 6]), Some(Quality::Diminished));
        assert_eq!(Quality::from_intervals(vec![0, 4, 7, 10]), Some(Quality::Dominant));
        assert_eq!(Quality::from_intervals(vec![0, 3, 7, 10]), Some(Quality::MinorSeventh));
        assert_eq!(Quality::from_intervals(vec![0, 4, 7, 11]), Some(Quality::MajorSeventh));
        assert_eq!(Quality::from_intervals(vec![0, 3, 6, 10]), Some(Quality::HalfDiminished));
    }
}
