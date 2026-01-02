use std::default;

use strum::{AsRefStr, Display, EnumCount, EnumIter, FromRepr};

pub mod cli;
pub mod metronome;
pub mod mode;
pub mod practice_state;
pub mod progression;

#[derive(
    Clone, Copy, Debug, EnumIter, Display, AsRefStr, PartialEq, EnumCount, FromRepr, Default,
)]
pub enum ModeOption {
    #[default]
    #[strum(to_string = "Circle of Fourths")]
    Fourths,
    #[strum(to_string = "Diatonic Progression")]
    Diatonic,
    #[strum(to_string = "Random Chords")]
    Random,
    #[strum(to_string = "Custom Progression")]
    Custom,
}

#[derive(
    Clone, Copy, Debug, EnumIter, Display, AsRefStr, PartialEq, EnumCount, FromRepr, Default,
)]
pub enum DiatonicOption {
    #[default]
    Incemental,
    #[strum(to_string = "Random Chord")]
    Random,
}
