use strum::{AsRefStr, Display, EnumCount, EnumIter, FromRepr};

use super::interval::Interval;

#[derive(
    Default, Clone, Copy, Debug, EnumIter, AsRefStr, PartialEq, EnumCount, FromRepr, Eq, Display,
)]
pub enum Quality {
    #[default]
    #[strum(to_string = "")]
    Major,
    #[strum(to_string = "m")]
    Minor,
    #[strum(to_string = "dim")]
    Diminished,
    #[strum(to_string = "aug")]
    Augmented,
    #[strum(to_string = "7")]
    Dominant,
    #[strum(to_string = "maj7")]
    MajorSeventh,
    #[strum(to_string = "m7")]
    MinorSeventh,
    #[strum(to_string = "m7b5")]
    HalfDiminished,
}

impl Quality {
    pub fn from_str(quality: &str) -> Quality {
        match quality {
            "" => Quality::Major,
            "m" => Quality::Minor,
            "dim" => Quality::Diminished,
            "aug" => Quality::Augmented,
            "7" => Quality::Dominant,
            "maj7" => Quality::MajorSeventh,
            "m7" => Quality::MajorSeventh,
            "m7b5" => Quality::HalfDiminished,
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
        }
    }

    pub fn from_intervals(intervals: Vec<i32>) -> Quality {
        match intervals[..] {
            [0, 4, 7] => Quality::Major,
            [0, 3, 7] => Quality::Minor,
            [0, 3, 6] => Quality::Diminished,
            [0, 5, 7] => Quality::Augmented,
            [0, 4, 7, 10] => Quality::Dominant,
            [0, 3, 7, 10] => Quality::MinorSeventh,
            [0, 4, 7, 11] => Quality::MajorSeventh,
            [0, 3, 6, 10] => Quality::HalfDiminished,
            _ => panic!("Invalid intervals"),
        }
    }
}
