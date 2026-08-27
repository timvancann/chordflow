use chordflow_music_theory::{
    chord::Chord,
    note::{practical_keys, Note},
    quality::{Notation, Quality},
};
use rand::{rng, seq::IndexedRandom};

/// Random-chord practice: draw a chord from the roots and qualities you have
/// chosen, and keep drawing.
///
/// Roots come from `practical_keys` rather than `generate_all_roots`, which
/// yields seventeen spellings including A♯ and D♯. Nobody writes an A♯
/// augmented triad on a chart, so drilling one is wasted repetition.
pub struct RandomConfig {
    pub roots: Vec<Note>,
    pub qualities: Vec<Quality>,
    pub current_chord: Chord,
    pub next_chord: Chord,
}

impl Default for RandomConfig {
    fn default() -> Self {
        // Major and minor are the least surprising place to start; the panel
        // widens it from there.
        Self::new(practical_keys(), vec![Quality::Major, Quality::Minor])
    }
}

impl RandomConfig {
    /// Build a config with chords already drawn from the given pool.
    ///
    /// Constructing rather than assigning the fields matters: chords drawn
    /// from a previous pool would otherwise survive the change and be shown
    /// as though they came from the new one.
    pub fn new(roots: Vec<Note>, qualities: Vec<Quality>) -> Self {
        let current_chord = draw(&roots, &qualities, None);
        let next_chord = draw(&roots, &qualities, Some(current_chord));

        Self {
            roots,
            qualities,
            current_chord,
            next_chord,
        }
    }

    /// Toggle a root, refusing to remove the last one. An empty pool has
    /// nothing to draw from, so the UI cannot be allowed to create one.
    pub fn toggle_root(&mut self, root: Note) {
        toggle(&mut self.roots, root);
        self.reset();
    }

    pub fn toggle_quality(&mut self, quality: Quality) {
        toggle(&mut self.qualities, quality);
        self.reset();
    }

    pub fn reset(&mut self) {
        self.current_chord = draw(&self.roots, &self.qualities, None);
        self.next_chord = draw(&self.roots, &self.qualities, Some(self.current_chord));
    }

    pub fn generate_next_chord(&mut self) {
        self.current_chord = self.next_chord;
        self.next_chord = draw(&self.roots, &self.qualities, Some(self.current_chord));
    }

    pub fn get_chords(&self, notation: Notation) -> (String, String) {
        (
            self.current_chord.symbol(notation),
            self.next_chord.symbol(notation),
        )
    }
}

/// Add or remove `value`, but never empty the list.
fn toggle<T: PartialEq>(values: &mut Vec<T>, value: T) {
    match values.iter().position(|existing| *existing == value) {
        Some(index) if values.len() > 1 => {
            values.remove(index);
        }
        Some(_) => {}
        None => values.push(value),
    }
}

/// Draw a chord, avoiding `previous` so the same chord never comes up twice
/// running — a repeat wastes a whole cycle of practice.
///
/// If the pool is a single chord there is nothing else to draw, so the repeat
/// is accepted rather than looping forever.
fn draw(roots: &[Note], qualities: &[Quality], previous: Option<Chord>) -> Chord {
    let mut rand = rng();
    let pool_size = roots.len() * qualities.len();

    for _ in 0..32 {
        let chord = Chord::new(
            *roots.choose(&mut rand).expect("roots is never empty"),
            *qualities
                .choose(&mut rand)
                .expect("qualities is never empty"),
        );

        if pool_size == 1 || Some(chord) != previous {
            return chord;
        }
    }

    // Vanishingly unlikely, but bounded: fall back rather than spin.
    Chord::new(roots[0], qualities[0])
}

#[cfg(test)]
mod tests {
    use chordflow_music_theory::note::NoteLetter;

    use super::*;

    #[test]
    fn test_draws_only_from_the_selected_pool() {
        let mut config = RandomConfig::new(
            vec![
                Note::new(NoteLetter::C, 0),
                Note::new(NoteLetter::F, 1),
                Note::new(NoteLetter::E, -1),
            ],
            vec![Quality::Dominant, Quality::HalfDiminished],
        );

        for _ in 0..300 {
            config.generate_next_chord();
            let chord = config.current_chord;
            assert!(
                config.qualities.contains(&chord.quality),
                "{chord} has a quality outside the pool"
            );
            assert!(
                config.roots.contains(&chord.root),
                "{chord} has a root outside the pool"
            );
        }
    }

    #[test]
    fn test_never_repeats_the_previous_chord() {
        let mut config = RandomConfig::default();

        for _ in 0..300 {
            let previous = config.current_chord;
            config.generate_next_chord();
            assert_ne!(
                config.current_chord, previous,
                "the same chord twice running wastes a cycle"
            );
        }
    }

    #[test]
    fn test_default_roots_are_the_twelve_practical_keys() {
        let config = RandomConfig::default();
        assert_eq!(config.roots, practical_keys());
        assert_eq!(config.qualities, vec![Quality::Major, Quality::Minor]);
    }

    #[test]
    fn test_toggling_adds_and_removes() {
        let mut config = RandomConfig::default();

        config.toggle_quality(Quality::Dominant);
        assert!(config.qualities.contains(&Quality::Dominant));

        config.toggle_quality(Quality::Dominant);
        assert!(!config.qualities.contains(&Quality::Dominant));
    }

    #[test]
    fn test_the_last_selection_cannot_be_removed() {
        let mut config = RandomConfig::new(vec![Note::new(NoteLetter::C, 0)], vec![Quality::Major]);

        config.toggle_quality(Quality::Major);
        config.toggle_root(Note::new(NoteLetter::C, 0));

        assert_eq!(
            config.qualities,
            vec![Quality::Major],
            "an empty pool has nothing to draw from"
        );
        assert_eq!(config.roots, vec![Note::new(NoteLetter::C, 0)]);
    }

    #[test]
    fn test_a_single_chord_pool_does_not_hang() {
        // With one root and one quality there is no second chord to draw, so
        // the no-repeat rule has to give way rather than loop.
        let mut config = RandomConfig::new(vec![Note::new(NoteLetter::C, 0)], vec![Quality::Major]);

        config.generate_next_chord();
        assert_eq!(config.current_chord.root, Note::new(NoteLetter::C, 0));
        assert_eq!(config.current_chord.quality, Quality::Major);
    }
}
