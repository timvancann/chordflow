use chord::Chord;
use note::{generate_all_roots, Note};
use quality::Quality;
use rand::{seq::IteratorRandom, Rng};
use strum::IntoEnumIterator;

pub mod accidental;
pub mod chord;
pub mod interval;
pub mod note;
pub mod quality;
pub mod scale;

pub fn random_note() -> Note {
    let mut rng = rand::rng();
    *generate_all_roots().iter().choose(&mut rng).unwrap()
}

pub fn random_quality(allowed: Option<Vec<Quality>>) -> Quality {
    let qualities = allowed.unwrap_or(Quality::iter().collect());

    let mut rng = rand::rng();
    qualities[rng.random_range(0..qualities.len())]
}

pub fn random_chord(selected_qualities: Option<Vec<Quality>>) -> Chord {
    Chord {
        root: random_note(),
        quality: random_quality(selected_qualities),
    }
}
