use std::fmt::{self, Display};

use chordflow_music_theory::{quality::Quality, scale::Scale};

use crate::{progression::Progression, DiatonicOption};

#[derive(Debug, PartialEq, Clone)]
pub enum Mode {
    Fourths(Quality),
    Random(Vec<Quality>),
    Custom(Option<Progression>),
    Diatonic(Scale, DiatonicOption),
}

impl Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Mode::Fourths(q) => write!(f, "Fourths - {}", q.name()),
            Mode::Random(_) => write!(f, "Random"),
            Mode::Custom(p) => match p {
                Some(progression) => write!(f, "Custom: {}", progression),
                None => write!(f, "Custom"),
            },
            Mode::Diatonic(chord, option) => write!(f, "Diatonic: {} - {}", chord, option),
        }
    }
}

impl Default for Mode {
    fn default() -> Self {
        Mode::Fourths(Quality::Major)
    }
}
