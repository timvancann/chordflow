use strum::{AsRefStr, EnumCount, EnumIter, FromRepr};

#[derive(Default, Clone, Copy, Debug, EnumIter, AsRefStr, PartialEq, EnumCount, FromRepr, Eq)]
pub enum Accidental {
    #[default]
    #[strum(to_string = "")]
    Natural,
    #[strum(to_string = "#")]
    Sharp,
    #[strum(to_string = "b")]
    Flat,
}

impl Accidental {
    pub fn from_string(accidental: &str) -> Accidental {
        match accidental {
            "#" => Accidental::Sharp,
            "b" => Accidental::Flat,
            _ => Accidental::Natural,
        }
    }

    pub fn to_semitones(self) -> i32 {
        match self {
            Accidental::Natural => 0,
            Accidental::Sharp => 1,
            Accidental::Flat => -1,
        }
    }
}
