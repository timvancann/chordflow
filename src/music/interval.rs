use strum::{AsRefStr, Display, EnumCount, EnumIter, FromRepr};

#[derive(
    Default, Clone, Copy, Debug, EnumIter, AsRefStr, PartialEq, EnumCount, FromRepr, Eq, Display,
)]
pub enum Interval {
    #[default]
    Unison,
    MinorSecond,
    MajorSecond,
    MinorThird,
    MajorThird,
    PerfectFourth,
    AugmentedFourth,
    Tritone,
    DiminishedFifth,
    PerfectFifth,
    MinorSixth,
    MajorSixth,
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
            Interval::MinorThird => 3,
            Interval::MajorThird => 4,
            Interval::PerfectFourth => 5,
            Interval::AugmentedFourth => 6,
            Interval::Tritone => 6,
            Interval::DiminishedFifth => 6,
            Interval::PerfectFifth => 7,
            Interval::MinorSixth => 8,
            Interval::MajorSixth => 9,
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
            Interval::MinorThird => 2,
            Interval::MajorThird => 2,
            Interval::PerfectFourth => 3,
            Interval::AugmentedFourth => 3,
            Interval::Tritone => 3,
            Interval::DiminishedFifth => 4,
            Interval::PerfectFifth => 4,
            Interval::MinorSixth => 5,
            Interval::MajorSixth => 5,
            Interval::MinorSeventh => 6,
            Interval::MajorSeventh => 6,
            _ => panic!("Invalid interval"),
        }
    }

    pub fn from_semitones(semitones: Vec<i32>) -> Vec<Self> {
        semitones
            .iter()
            .map(|&x| Interval::from_semitone(x))
            .collect()
    }
}
