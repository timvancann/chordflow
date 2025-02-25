use std::fmt::{self, Display};

use chordflow_music_theory::{
    quality::Quality,
    scale::{Scale, ScaleType},
};

use crate::{
    practice_state::{ConfigState, PracticState},
    progression::Progression,
    DiatonicOption, ModeOption,
};

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

pub fn update_mode_from_state(
    selected_mode: &ModeOption,
    practice_state: &mut PracticState,
    config_state: &ConfigState,
) -> bool {
    match selected_mode {
        ModeOption::Fourths => {
            practice_state.set_mode(Mode::Fourths(config_state.fourths_selected_quality))
        }
        ModeOption::Random => {
            practice_state.set_mode(Mode::Random(config_state.random_selected_qualities.clone()))
        }
        ModeOption::Custom => {
            practice_state.set_mode(Mode::Custom(config_state.progression.clone()))
        }
        ModeOption::Diatonic => practice_state.set_mode(Mode::Diatonic(
            Scale::new(config_state.diatonic_root, ScaleType::Diatonic),
            config_state.diatonic_option,
        )),
    }
}
