use strum::{AsRefStr, Display, EnumCount, EnumIter, FromRepr};

pub mod cli;
pub mod metronome;
pub mod mode;
pub mod practice_state;
pub mod progression;

#[derive(Clone, Copy, Debug, EnumIter, Display, AsRefStr, PartialEq, EnumCount, FromRepr)]
pub enum ModeOption {
    Fourths,
    Diatonic,
    Random,
    Custom,
}

#[derive(
    Clone, Copy, Debug, EnumIter, Display, AsRefStr, PartialEq, EnumCount, FromRepr, Default,
)]
pub enum DiatonicOption {
    #[default]
    Incemental,
    Random,
}
