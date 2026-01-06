use chordflow_music_theory::{
    quality::Quality,
    scale::{Scale, ScaleType},
};

use crate::{
    progression::Progression,
    state::{
        config::ConfigState,
        modes::{DiatonicOption, ModeOption},
        practice::PracticeState,
    },
};

#[derive(Debug, PartialEq, Clone)]
pub enum Mode {
    Fourths(Quality),
    Diatonic(Scale, DiatonicOption),
    Random(Vec<Quality>),
    Custom(Option<Progression>),
}

impl Default for Mode {
    fn default() -> Self {
        Mode::Fourths(Quality::Major)
    }
}

pub fn update_mode_from_state(
    selected_mode: &ModeOption,
    practice_state: &mut PracticeState,
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
