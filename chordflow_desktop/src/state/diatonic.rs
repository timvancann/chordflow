use chordflow_music_theory::{
    chord::Chord,
    interval::Interval,
    note::Note,
    scale::{ParentScale, Scale},
};
use rand::{rng, seq::IndexedRandom};

/// Walk the chords of a key, degree by degree.
///
/// The key can be major, harmonic minor or melodic minor — the three parents
/// whose modes the catalog is built from — so this drills all seven modes of
/// whichever one you pick.
pub struct DiatonicConfig {
    pub scale: Scale,
    pub parent: ParentScale,
    pub is_random: bool,
    /// Drill seventh chords instead of triads. A seventh contains its triad,
    /// so this is strictly more information, not a different progression.
    pub use_sevenths: bool,
    next_scale_interval: Interval,
    pub current_chord: Chord,
    pub next_chord: Chord,
}

impl DiatonicConfig {
    pub fn set_root(&mut self, root: Note) {
        self.scale = Scale::new(root, self.parent.scale_type());
        self.reset();
    }

    pub fn set_use_sevenths(&mut self, use_sevenths: bool) {
        self.use_sevenths = use_sevenths;
        self.reset();
    }

    pub fn set_parent(&mut self, parent: ParentScale) {
        self.parent = parent;
        self.scale = Scale::new(self.scale.root, parent.scale_type());
        self.reset();
    }

    pub fn reset(&mut self) {
        // Degree I, whatever it is: a major triad in a major key, but minor in
        // both minor parents. Asking the scale rather than assuming is what
        // makes harmonic and melodic minor correct.
        self.current_chord = self.chord_at(Interval::Unison);
        self.advance_to(Interval::Unison);
    }

    pub fn generate_next_chord(&mut self) {
        self.current_chord = self.next_chord;
        self.advance_to(self.next_scale_interval);
    }

    /// Pick the chord that follows `from` and record which degree it sits on.
    ///
    /// Keeping `next_scale_interval` in step with `next_chord` is the whole
    /// point: they used to drift after a reset, so the walk played degree two
    /// twice before continuing.
    fn advance_to(&mut self, from: Interval) {
        let interval = next_diatonic_scale_interval(self.is_random, &self.scale, &from);
        self.next_scale_interval = interval;
        self.next_chord = self.chord_at(interval);
    }

    pub fn get_chords(&self) -> (String, String) {
        (self.current_chord.to_string(), self.next_chord.to_string())
    }

    /// The diatonic triad on the degree at `interval` within the current scale.
    fn chord_at(&self, interval: Interval) -> Chord {
        let degree = self
            .scale
            .intervals
            .iter()
            .position(|i| *i == interval)
            .expect("interval came from this scale");
        let chords = if self.use_sevenths {
            self.scale.diatonic_sevenths()
        } else {
            self.scale.diatonic_triads()
        };

        chords.expect("the diatonic practice mode always uses a seven-note scale")[degree]
    }
}

impl Default for DiatonicConfig {
    fn default() -> Self {
        let parent = ParentScale::default();
        let scale = Scale::new(Note::default(), parent.scale_type());
        let mut config = DiatonicConfig {
            scale,
            parent,
            is_random: false,
            use_sevenths: false,
            next_scale_interval: Interval::Unison,
            current_chord: Chord::new(
                Note::default(),
                chordflow_music_theory::quality::Quality::Major,
            ),
            next_chord: Chord::new(
                Note::default(),
                chordflow_music_theory::quality::Quality::Major,
            ),
        };
        config.reset();
        config
    }
}

fn next_diatonic_scale_interval(
    is_random: bool,
    scale: &Scale,
    current_scale_interval: &Interval,
) -> Interval {
    let mut rand = rng();
    if is_random {
        *scale.intervals.choose(&mut rand).unwrap()
    } else {
        let index = scale
            .intervals
            .iter()
            .position(|f| f == current_scale_interval)
            .unwrap();
        let next_index = (index + 1) % scale.intervals.len();
        scale.intervals[next_index]
    }
}

#[cfg(test)]
mod tests {
    use chordflow_music_theory::{note::NoteLetter, quality::Quality};

    use super::*;

    fn walk(parent: ParentScale, root: Note) -> Vec<String> {
        let mut config = DiatonicConfig {
            parent,
            ..Default::default()
        };
        config.set_root(root);

        let mut chords = vec![config.current_chord.to_string()];
        for _ in 0..6 {
            config.generate_next_chord();
            chords.push(config.current_chord.to_string());
        }
        chords
    }

    #[test]
    fn test_walking_a_major_key_gives_its_diatonic_triads() {
        assert_eq!(
            walk(ParentScale::Major, Note::new(NoteLetter::C, 0)).join(" "),
            "C D- E- F G A- Bo"
        );
    }

    #[test]
    fn test_walking_harmonic_minor_gives_its_own_chords() {
        // The point of the feature: an augmented third degree and a
        // diminished seventh, neither of which occur in a major key.
        assert_eq!(
            walk(ParentScale::HarmonicMinor, Note::new(NoteLetter::C, 0)).join(" "),
            "C- Do E\u{266d}+ F- G A\u{266d} Bo"
        );
    }

    #[test]
    fn test_walking_melodic_minor_gives_its_own_chords() {
        assert_eq!(
            walk(ParentScale::MelodicMinor, Note::new(NoteLetter::C, 0)).join(" "),
            "C- D- E\u{266d}+ F G Ao Bo"
        );
    }

    #[test]
    fn test_sevenths_toggle_swaps_the_whole_progression() {
        let mut config = DiatonicConfig::default();
        config.set_root(Note::new(NoteLetter::C, 0));

        let mut triads = vec![config.current_chord.to_string()];
        for _ in 0..6 {
            config.generate_next_chord();
            triads.push(config.current_chord.to_string());
        }
        assert_eq!(triads.join(" "), "C D- E- F G A- Bo");

        config.set_use_sevenths(true);
        let mut sevenths = vec![config.current_chord.to_string()];
        for _ in 0..6 {
            config.generate_next_chord();
            sevenths.push(config.current_chord.to_string());
        }
        assert_eq!(
            sevenths.join(" "),
            "C\u{394} D-7 E-7 F\u{394} G7 A-7 B\u{f8}"
        );
    }

    #[test]
    fn test_sevenths_work_for_every_parent() {
        use strum::IntoEnumIterator;

        for parent in ParentScale::iter() {
            let mut config = DiatonicConfig {
                parent,
                use_sevenths: true,
                ..Default::default()
            };
            config.set_root(Note::new(NoteLetter::C, 0));

            for _ in 0..7 {
                config.generate_next_chord();
            }
        }
    }

    #[test]
    fn test_the_opening_chord_is_degree_one_of_the_chosen_parent() {
        // It used to be hardcoded to a major triad, which is only correct for
        // a major key.
        let minor_parents = [ParentScale::HarmonicMinor, ParentScale::MelodicMinor];
        for parent in minor_parents {
            let mut config = DiatonicConfig::default();
            config.set_parent(parent);
            assert_eq!(
                config.current_chord.quality,
                Quality::Minor,
                "{parent} starts on a minor triad"
            );
        }

        let mut config = DiatonicConfig::default();
        config.set_parent(ParentScale::Major);
        assert_eq!(config.current_chord.quality, Quality::Major);
    }

    #[test]
    fn test_switching_parent_keeps_the_root() {
        let mut config = DiatonicConfig::default();
        config.set_root(Note::new(NoteLetter::E, -1));
        config.set_parent(ParentScale::HarmonicMinor);

        assert_eq!(config.scale.root, Note::new(NoteLetter::E, -1));
        assert_eq!(config.parent, ParentScale::HarmonicMinor);
    }

    #[test]
    fn test_every_parent_walks_without_panicking_in_every_key() {
        use chordflow_music_theory::note::practical_keys;
        use strum::IntoEnumIterator;

        for parent in ParentScale::iter() {
            for root in practical_keys() {
                let chords = walk(parent, root);
                assert_eq!(chords.len(), 7, "{root} {parent}");
            }
        }
    }
}
