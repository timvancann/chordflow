use std::fmt::Display;

use strum::{AsRefStr, Display, EnumCount, EnumIter, FromRepr};

use super::{interval::Interval, note::Note};

#[derive(
    Default, Clone, Copy, Debug, EnumIter, AsRefStr, PartialEq, EnumCount, FromRepr, Eq, Display,
)]
pub enum ScaleType {
    #[default]
    Diatonic,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Scale {
    pub root: Note,
    pub scale_type: ScaleType,
    pub intervals: Vec<Interval>,
}

impl Display for Scale {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {}", self.root, self.scale_type)
    }
}

impl Scale {
    pub fn new(root: Note, scale_type: ScaleType) -> Scale {
        let intervals = match scale_type {
            ScaleType::Diatonic => vec![
                Interval::Unison,
                Interval::MajorSecond,
                Interval::MajorThird,
                Interval::PerfectFourth,
                Interval::PerfectFifth,
                Interval::MajorSixth,
                Interval::MajorSeventh,
            ],
        };

        Scale {
            root,
            scale_type,
            intervals,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_c_major_notes() {
        let scale = Scale::new(
            Note::new(crate::music::note::NoteLetter::C, 0),
            ScaleType::Diatonic,
        );

        let actual_notes = vec![
            Note::new(crate::music::note::NoteLetter::C, 0),
            Note::new(crate::music::note::NoteLetter::D, 0),
            Note::new(crate::music::note::NoteLetter::E, 0),
            Note::new(crate::music::note::NoteLetter::F, 0),
            Note::new(crate::music::note::NoteLetter::G, 0),
            Note::new(crate::music::note::NoteLetter::A, 0),
            Note::new(crate::music::note::NoteLetter::B, 0),
        ];

        for (interval, note) in scale.intervals.into_iter().zip(actual_notes) {
            assert_eq!(scale.root.add_interval(interval), note);
        }
    }
}
