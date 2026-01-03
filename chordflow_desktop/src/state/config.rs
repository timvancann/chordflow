use chordflow_music_theory::{note::Note, quality::Quality};
use strum::IntoEnumIterator;

use crate::{progression::Progression, state::options::DiatonicOption};

#[derive(Clone, PartialEq)]
pub struct ConfigState {
    pub fourths_selected_quality: Quality,
    pub progression: Option<Progression>,
    pub random_selected_qualities: Vec<Quality>,
    pub diatonic_root: Note,
    pub diatonic_option: DiatonicOption,
}
impl Default for ConfigState {
    fn default() -> Self {
        ConfigState {
            fourths_selected_quality: Quality::default(),
            progression: Option::default(),
            random_selected_qualities: Quality::iter().collect(),
            diatonic_root: Note::default(),
            diatonic_option: DiatonicOption::default(),
        }
    }
}
