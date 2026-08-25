use std::fmt::Display;

use strum::{AsRefStr, EnumCount, EnumIter, FromRepr, IntoEnumIterator};

use super::{chord::Chord, interval::Interval, note::Note, quality::Quality};

/// The four groupings used on the Scales.pdf poster.
#[derive(Clone, Copy, Debug, EnumIter, AsRefStr, PartialEq, Eq)]
pub enum ScaleFamily {
    Major,
    MelodicMinor,
    HarmonicMinor,
    Other,
}

impl Display for ScaleFamily {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

impl ScaleFamily {
    /// The poster's heading for this family.
    pub fn display_name(self) -> &'static str {
        match self {
            ScaleFamily::Major => "major",
            ScaleFamily::MelodicMinor => "melodic minor",
            ScaleFamily::HarmonicMinor => "harmonic minor",
            ScaleFamily::Other => "other",
        }
    }
}

/// Every scale on the Scales.pdf poster, in the poster's order.
#[derive(
    Default, Clone, Copy, Debug, EnumIter, AsRefStr, PartialEq, EnumCount, FromRepr, Eq,
)]
pub enum ScaleType {
    #[default]
    Ionian,
    Dorian,
    Phrygian,
    Lydian,
    Mixolydian,
    Aeolian,
    Locrian,

    MelodicMinor,
    DorianFlat2,
    LydianAugmented,
    LydianDominant,
    MixolydianFlat6,
    AeolianFlat5,
    /// The only heptatonic scale in the catalog that reuses and skips
    /// letter names: C altered spells `C Db D# E Gb G# Bb`, with two Ds,
    /// two Gs, and no A or F. This is correct and faithful to the poster,
    /// not a spelling bug.
    Altered,

    HarmonicMinor,
    LocrianNatural6,
    IonianAugmented,
    LocrianSharp4,
    PhrygianDominant,
    LydianSharp9,
    SuperlocrianDoubleFlat7,

    MajorBlues,
    MinorBlues,
    WholeTone,
    Augmented,
    DiminishedHalfWhole,
    DiminishedWholeHalf,
}

impl Display for ScaleType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

impl ScaleType {
    pub fn family(self) -> ScaleFamily {
        use ScaleType::*;
        match self {
            Ionian | Dorian | Phrygian | Lydian | Mixolydian | Aeolian | Locrian => {
                ScaleFamily::Major
            }
            MelodicMinor | DorianFlat2 | LydianAugmented | LydianDominant | MixolydianFlat6
            | AeolianFlat5 | Altered => ScaleFamily::MelodicMinor,
            HarmonicMinor | LocrianNatural6 | IonianAugmented | LocrianSharp4
            | PhrygianDominant | LydianSharp9 | SuperlocrianDoubleFlat7 => {
                ScaleFamily::HarmonicMinor
            }
            MajorBlues | MinorBlues | WholeTone | Augmented | DiminishedHalfWhole
            | DiminishedWholeHalf => ScaleFamily::Other,
        }
    }

    /// The name as printed on the poster, kept verbatim.
    pub fn display_name(self) -> &'static str {
        use ScaleType::*;
        match self {
            Ionian => "ionian",
            Dorian => "dorian",
            Phrygian => "phrygian",
            Lydian => "lydian",
            Mixolydian => "mixolydian",
            Aeolian => "aeolian",
            Locrian => "locrian",
            MelodicMinor => "melodic minor",
            DorianFlat2 => "dorian b2",
            LydianAugmented => "lydian augmented",
            LydianDominant => "lydian dominant",
            MixolydianFlat6 => "mixolydian b6",
            AeolianFlat5 => "aeolian b5",
            Altered => "altered",
            HarmonicMinor => "harmonic minor",
            LocrianNatural6 => "locrian natural 6",
            IonianAugmented => "ionian augmented",
            LocrianSharp4 => "locrian #4",
            PhrygianDominant => "phrygian dominant",
            LydianSharp9 => "lydian #9",
            SuperlocrianDoubleFlat7 => "superlocrian bb7",
            MajorBlues => "major blues",
            MinorBlues => "minor blues",
            WholeTone => "whole tone",
            Augmented => "augmented",
            DiminishedHalfWhole => "diminished half whole",
            DiminishedWholeHalf => "diminished whole half",
        }
    }

    /// The scale's degrees, spelled. Spelling matters: `#4` and `b5` are the
    /// same fret but different letters, and only the right one makes
    /// `Note::add_interval` produce the correct note name.
    pub fn formula(self) -> Vec<Interval> {
        use Interval::*;
        use ScaleType::*;
        match self {
            Ionian => vec![Unison, MajorSecond, MajorThird, PerfectFourth, PerfectFifth, MajorSixth, MajorSeventh],
            Dorian => vec![Unison, MajorSecond, MinorThird, PerfectFourth, PerfectFifth, MajorSixth, MinorSeventh],
            Phrygian => vec![Unison, MinorSecond, MinorThird, PerfectFourth, PerfectFifth, MinorSixth, MinorSeventh],
            Lydian => vec![Unison, MajorSecond, MajorThird, AugmentedFourth, PerfectFifth, MajorSixth, MajorSeventh],
            Mixolydian => vec![Unison, MajorSecond, MajorThird, PerfectFourth, PerfectFifth, MajorSixth, MinorSeventh],
            Aeolian => vec![Unison, MajorSecond, MinorThird, PerfectFourth, PerfectFifth, MinorSixth, MinorSeventh],
            Locrian => vec![Unison, MinorSecond, MinorThird, PerfectFourth, DiminishedFifth, MinorSixth, MinorSeventh],

            MelodicMinor => vec![Unison, MajorSecond, MinorThird, PerfectFourth, PerfectFifth, MajorSixth, MajorSeventh],
            DorianFlat2 => vec![Unison, MinorSecond, MinorThird, PerfectFourth, PerfectFifth, MajorSixth, MinorSeventh],
            LydianAugmented => vec![Unison, MajorSecond, MajorThird, AugmentedFourth, AugmentedFifth, MajorSixth, MajorSeventh],
            LydianDominant => vec![Unison, MajorSecond, MajorThird, AugmentedFourth, PerfectFifth, MajorSixth, MinorSeventh],
            MixolydianFlat6 => vec![Unison, MajorSecond, MajorThird, PerfectFourth, PerfectFifth, MinorSixth, MinorSeventh],
            AeolianFlat5 => vec![Unison, MajorSecond, MinorThird, PerfectFourth, DiminishedFifth, MinorSixth, MinorSeventh],
            Altered => vec![Unison, MinorSecond, AugmentedSecond, MajorThird, DiminishedFifth, AugmentedFifth, MinorSeventh],

            HarmonicMinor => vec![Unison, MajorSecond, MinorThird, PerfectFourth, PerfectFifth, MinorSixth, MajorSeventh],
            LocrianNatural6 => vec![Unison, MinorSecond, MinorThird, PerfectFourth, DiminishedFifth, MajorSixth, MinorSeventh],
            IonianAugmented => vec![Unison, MajorSecond, MajorThird, PerfectFourth, AugmentedFifth, MajorSixth, MajorSeventh],
            LocrianSharp4 => vec![Unison, MajorSecond, MinorThird, AugmentedFourth, PerfectFifth, MajorSixth, MinorSeventh],
            PhrygianDominant => vec![Unison, MinorSecond, MajorThird, PerfectFourth, PerfectFifth, MinorSixth, MinorSeventh],
            LydianSharp9 => vec![Unison, AugmentedSecond, MajorThird, AugmentedFourth, PerfectFifth, MajorSixth, MajorSeventh],
            SuperlocrianDoubleFlat7 => vec![Unison, MinorSecond, MinorThird, DiminishedFourth, DiminishedFifth, MinorSixth, DiminishedSeventh],

            MajorBlues => vec![Unison, MajorSecond, MinorThird, MajorThird, PerfectFifth, MajorSixth],
            MinorBlues => vec![Unison, MinorThird, PerfectFourth, DiminishedFifth, PerfectFifth, MinorSeventh],
            WholeTone => vec![Unison, MajorSecond, MajorThird, AugmentedFourth, AugmentedFifth, MinorSeventh],
            Augmented => vec![Unison, AugmentedSecond, MajorThird, PerfectFifth, AugmentedFifth, MajorSeventh],
            DiminishedHalfWhole => vec![Unison, MinorSecond, AugmentedSecond, MajorThird, DiminishedFifth, PerfectFifth, MajorSixth, MinorSeventh],
            DiminishedWholeHalf => vec![Unison, MajorSecond, MinorThird, PerfectFourth, DiminishedFifth, AugmentedFifth, MajorSixth, MajorSeventh],
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Scale {
    pub root: Note,
    pub scale_type: ScaleType,
    pub intervals: Vec<Interval>,
}

impl Display for Scale {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {}", self.root, self.scale_type)
    }
}

impl Scale {
    pub fn new(root: Note, scale_type: ScaleType) -> Scale {
        Scale {
            root,
            scale_type,
            intervals: scale_type.formula(),
        }
    }

    /// The scale's notes, spelled correctly for this root. The same formula
    /// gives different letters per key: G lydian is G A B C# D E F#, while
    /// F# ionian genuinely contains E#.
    pub fn notes(&self) -> Vec<Note> {
        self.intervals
            .iter()
            .map(|interval| self.root.add_interval(*interval))
            .collect()
    }

    /// The triads built by stacking thirds on each degree, or `None` if this
    /// scale does not have seven notes (stacking by scale index has no
    /// accepted meaning for the blues, whole tone, augmented, and diminished
    /// scales).
    pub fn diatonic_triads(&self) -> Option<Vec<Chord>> {
        self.stacked_chords(&[0, 2, 4])
    }

    /// The seventh chords built by stacking thirds on each degree. Same
    /// `None` condition as `diatonic_triads`.
    pub fn diatonic_sevenths(&self) -> Option<Vec<Chord>> {
        self.stacked_chords(&[0, 2, 4, 6])
    }

    /// Builds one chord per degree by taking the scale members at the given
    /// index offsets, or `None` if any degree's interval set names no
    /// `Quality`: a missing chord beats a wrong one, and a partial result
    /// would silently misalign the returned vector against scale degrees.
    /// Every catalog scale currently names a quality at every degree, so
    /// this `None` branch is not reached in practice; it exists as a guard
    /// against silently mislabelling a future scale that doesn't.
    /// When `Some`, the returned vector's index IS the scale degree —
    /// callers may rely on `result[degree]` being the chord for that degree.
    fn stacked_chords(&self, offsets: &[usize]) -> Option<Vec<Chord>> {
        if self.intervals.len() != 7 {
            return None;
        }

        let notes = self.notes();
        (0..7)
            .map(|degree| {
                let members: Vec<i32> = offsets
                    .iter()
                    .map(|offset| self.intervals[(degree + offset) % 7].to_semitones())
                    .collect();
                let root_semitones = members[0];
                let relative: Vec<i32> = members
                    .iter()
                    .map(|s| (s - root_semitones).rem_euclid(12))
                    .collect();
                Quality::from_intervals(relative).map(|q| Chord::new(notes[degree], q))
            })
            .collect()
    }
}

/// Every catalog scale that contains all of `chord`'s notes, in catalog
/// order. This is the reverse of every other lookup in this module: given a
/// chord, what can be played over it?
///
/// Results are one scale per `(scale_type, root pitch class)`. `generate_all_roots`
/// returns 17 spellings rather than 12, so C# and Db both appear; the
/// spelling kept is the one whose notes carry the fewest accidentals, with
/// ties going to the flat. No ranking is applied beyond catalog order:
/// choosing the "best" scale over a chord is the player's call, not the app's.
///
/// Ordering is only a guarantee across scale types (catalog order). Within
/// one scale type, results follow `generate_all_roots()`'s order (`C C#
/// D D# Db E Eb F F# G G# Gb A A# Ab B Bb`), which is neither alphabetical
/// nor circle-of-fifths. Sort if you want a musical order.
pub fn scales_containing(chord: &Chord) -> Vec<Scale> {
    let wanted: Vec<i32> = chord.to_c_based_semitones();

    let mut best: Vec<(Scale, i32)> = Vec::new();

    for scale_type in ScaleType::iter() {
        for root in crate::note::generate_all_roots() {
            let scale = Scale::new(root, scale_type);
            let notes = scale.notes();
            let pitch_classes: Vec<i32> = notes
                .iter()
                .map(|n| n.to_semitones().rem_euclid(12))
                .collect();

            if !wanted.iter().all(|w| pitch_classes.contains(w)) {
                continue;
            }

            let cost = spelling_cost(&notes);
            let pitch_class = root.to_semitones().rem_euclid(12);

            match best
                .iter()
                .position(|(s, _)| s.scale_type == scale_type
                    && s.root.to_semitones().rem_euclid(12) == pitch_class)
            {
                Some(index) if cost < best[index].1 => best[index] = (scale, cost),
                Some(_) => {}
                None => best.push((scale, cost)),
            }
        }
    }

    best.into_iter().map(|(scale, _)| scale).collect()
}

/// Total accidentals across a scale's notes, plus one if the root is sharp.
/// The tiebreaker biases toward flat spellings, which read better for the
/// keys guitarists actually use (Bb mixolydian, not A# mixolydian).
fn spelling_cost(notes: &[Note]) -> i32 {
    let accidentals: i32 = notes.iter().map(|n| n.accidentals.abs()).sum();
    let sharp_root_penalty = i32::from(notes[0].accidentals > 0);
    accidentals * 2 + sharp_root_penalty
}

#[cfg(test)]
mod tests {
    use strum::IntoEnumIterator;

    use crate::note::{Note, NoteLetter};

    use super::{Scale, ScaleFamily, ScaleType};

    /// Every row of Scales.pdf, transcribed. This table is the oracle for the
    /// whole catalog: if a formula is ever mistyped, this test fails.
    fn poster() -> Vec<(ScaleType, ScaleFamily, &'static str, &'static str)> {
        vec![
            (ScaleType::Ionian, ScaleFamily::Major, "ionian", "R 2 3 4 5 6 7"),
            (ScaleType::Dorian, ScaleFamily::Major, "dorian", "R 2 b3 4 5 6 b7"),
            (ScaleType::Phrygian, ScaleFamily::Major, "phrygian", "R b2 b3 4 5 b6 b7"),
            (ScaleType::Lydian, ScaleFamily::Major, "lydian", "R 2 3 #4 5 6 7"),
            (ScaleType::Mixolydian, ScaleFamily::Major, "mixolydian", "R 2 3 4 5 6 b7"),
            (ScaleType::Aeolian, ScaleFamily::Major, "aeolian", "R 2 b3 4 5 b6 b7"),
            (ScaleType::Locrian, ScaleFamily::Major, "locrian", "R b2 b3 4 b5 b6 b7"),

            (ScaleType::MelodicMinor, ScaleFamily::MelodicMinor, "melodic minor", "R 2 b3 4 5 6 7"),
            (ScaleType::DorianFlat2, ScaleFamily::MelodicMinor, "dorian b2", "R b2 b3 4 5 6 b7"),
            (ScaleType::LydianAugmented, ScaleFamily::MelodicMinor, "lydian augmented", "R 2 3 #4 #5 6 7"),
            (ScaleType::LydianDominant, ScaleFamily::MelodicMinor, "lydian dominant", "R 2 3 #4 5 6 b7"),
            (ScaleType::MixolydianFlat6, ScaleFamily::MelodicMinor, "mixolydian b6", "R 2 3 4 5 b6 b7"),
            (ScaleType::AeolianFlat5, ScaleFamily::MelodicMinor, "aeolian b5", "R 2 b3 4 b5 b6 b7"),
            (ScaleType::Altered, ScaleFamily::MelodicMinor, "altered", "R b2 #2 3 b5 #5 b7"),

            (ScaleType::HarmonicMinor, ScaleFamily::HarmonicMinor, "harmonic minor", "R 2 b3 4 5 b6 7"),
            (ScaleType::LocrianNatural6, ScaleFamily::HarmonicMinor, "locrian natural 6", "R b2 b3 4 b5 6 b7"),
            (ScaleType::IonianAugmented, ScaleFamily::HarmonicMinor, "ionian augmented", "R 2 3 4 #5 6 7"),
            (ScaleType::LocrianSharp4, ScaleFamily::HarmonicMinor, "locrian #4", "R 2 b3 #4 5 6 b7"),
            (ScaleType::PhrygianDominant, ScaleFamily::HarmonicMinor, "phrygian dominant", "R b2 3 4 5 b6 b7"),
            (ScaleType::LydianSharp9, ScaleFamily::HarmonicMinor, "lydian #9", "R #2 3 #4 5 6 7"),
            (ScaleType::SuperlocrianDoubleFlat7, ScaleFamily::HarmonicMinor, "superlocrian bb7", "R b2 b3 b4 b5 b6 bb7"),

            // major blues: the poster prints "R 2 b3 b3 5 6"; confirmed typo,
            // the flat third is followed by the natural third.
            (ScaleType::MajorBlues, ScaleFamily::Other, "major blues", "R 2 b3 3 5 6"),
            (ScaleType::MinorBlues, ScaleFamily::Other, "minor blues", "R b3 4 b5 5 b7"),
            (ScaleType::WholeTone, ScaleFamily::Other, "whole tone", "R 2 3 #4 #5 b7"),
            (ScaleType::Augmented, ScaleFamily::Other, "augmented", "R #2 3 5 #5 7"),
            (ScaleType::DiminishedHalfWhole, ScaleFamily::Other, "diminished half whole", "R b2 #2 3 b5 5 6 b7"),
            (ScaleType::DiminishedWholeHalf, ScaleFamily::Other, "diminished whole half", "R 2 b3 4 b5 #5 6 7"),
        ]
    }

    #[test]
    fn test_formulas_match_the_poster() {
        for (scale_type, _, name, formula) in poster() {
            let actual: Vec<&str> = scale_type
                .formula()
                .iter()
                .map(|i| i.degree_label())
                .collect();
            assert_eq!(actual.join(" "), formula, "{name} formula");
        }
    }

    #[test]
    fn test_families_and_names_match_the_poster() {
        for (scale_type, family, name, _) in poster() {
            assert_eq!(scale_type.family(), family, "{name} family");
            assert_eq!(scale_type.display_name(), name, "{scale_type:?} name");
        }
    }

    #[test]
    fn test_catalog_is_complete_and_has_no_extras() {
        let listed: Vec<ScaleType> = poster().into_iter().map(|(t, _, _, _)| t).collect();
        let all: Vec<ScaleType> = ScaleType::iter().collect();
        assert_eq!(all.len(), 27, "the poster has 27 scales");
        assert_eq!(all, listed, "ScaleType order must match the poster's order");
    }

    #[test]
    fn test_scale_new_records_the_given_root_and_type_with_a_nonempty_formula() {
        // `Scale::new` literally assigns `intervals: scale_type.formula()`, so
        // asserting `intervals == formula()` would be tautological. Instead
        // assert what actually matters: the constructor records what it was
        // given, and every catalog variant produces a real formula.
        let root = Note::new(NoteLetter::C, 0);
        let scale = Scale::new(root, ScaleType::Lydian);
        assert_eq!(scale.root, root);
        assert_eq!(scale.scale_type, ScaleType::Lydian);

        for scale_type in ScaleType::iter() {
            assert!(
                !Scale::new(root, scale_type).intervals.is_empty(),
                "{scale_type} should have a non-empty formula"
            );
        }
    }

    #[test]
    fn test_display_uses_the_poster_name() {
        let scale = Scale::new(Note::new(NoteLetter::G, 0), ScaleType::LydianDominant);
        assert_eq!(scale.to_string(), "G lydian dominant");
    }

    #[test]
    fn test_scale_family_display_uses_the_poster_heading() {
        assert_eq!(ScaleFamily::Major.to_string(), "major");
        assert_eq!(ScaleFamily::MelodicMinor.to_string(), "melodic minor");
        assert_eq!(ScaleFamily::HarmonicMinor.to_string(), "harmonic minor");
        assert_eq!(ScaleFamily::Other.to_string(), "other");
    }

    fn spell(root: (NoteLetter, i32), scale_type: ScaleType) -> String {
        Scale::new(Note::new(root.0, root.1), scale_type)
            .notes()
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    }

    #[test]
    fn test_spelling_in_easy_keys() {
        assert_eq!(spell((NoteLetter::C, 0), ScaleType::Ionian), "C D E F G A B");
        assert_eq!(spell((NoteLetter::G, 0), ScaleType::Lydian), "G A B C♯ D E F♯");
        assert_eq!(spell((NoteLetter::D, 0), ScaleType::Dorian), "D E F G A B C");
        assert_eq!(spell((NoteLetter::A, 0), ScaleType::HarmonicMinor), "A B C D E F G♯");
    }

    #[test]
    fn test_spelling_needs_sharps_that_look_wrong_but_are_right() {
        // F# major really does contain E#, not F.
        assert_eq!(spell((NoteLetter::F, 1), ScaleType::Ionian), "F♯ G♯ A♯ B C♯ D♯ E♯");
    }

    #[test]
    fn test_spelling_needs_double_flats() {
        // superlocrian bb7 is the scale that forces a doubly-flattened seventh.
        assert_eq!(
            spell((NoteLetter::C, 0), ScaleType::SuperlocrianDoubleFlat7),
            "C D♭ E♭ F♭ G♭ A♭ B♭♭"
        );
    }

    #[test]
    fn test_spelling_db_aeolian_flat5_spells_the_flat5_as_abb() {
        // Db aeolian b5 is the other scale that forces a double flat.
        assert_eq!(
            spell((NoteLetter::D, -1), ScaleType::AeolianFlat5),
            "D♭ E♭ F♭ G♭ A♭♭ B♭♭ C♭"
        );
    }

    #[test]
    fn test_every_scale_note_is_the_right_pitch_class_and_sanely_spelled() {
        for root in crate::note::generate_all_roots() {
            for scale_type in ScaleType::iter() {
                let scale = Scale::new(root, scale_type);
                let root_pitch_class = root.to_semitones().rem_euclid(12);

                for (note, interval) in scale.notes().iter().zip(scale_type.formula()) {
                    let expected_pitch_class =
                        (root_pitch_class + interval.to_semitones()).rem_euclid(12);
                    assert_eq!(
                        note.to_semitones().rem_euclid(12),
                        expected_pitch_class,
                        "{root} {scale_type}: {note} should be pitch class {expected_pitch_class}"
                    );
                    assert!(
                        note.accidentals.abs() <= 2,
                        "{root} {scale_type}: {note} has an unreasonable number of accidentals"
                    );
                }
            }
        }
    }

    fn chord_symbols(chords: Option<Vec<crate::chord::Chord>>) -> String {
        chords
            .expect("expected a heptatonic scale")
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    }

    #[test]
    fn test_triads_of_c_major() {
        let scale = Scale::new(Note::new(NoteLetter::C, 0), ScaleType::Ionian);
        assert_eq!(chord_symbols(scale.diatonic_triads()), "C D- E- F G A- Bo");
    }

    #[test]
    fn test_sevenths_of_c_major() {
        let scale = Scale::new(Note::new(NoteLetter::C, 0), ScaleType::Ionian);
        assert_eq!(chord_symbols(scale.diatonic_sevenths()), "CΔ D-7 E-7 FΔ G7 A-7 Bø");
    }

    #[test]
    fn test_harmonic_minor_third_degree_is_augmented() {
        // This is the case that panics under the pre-Task-2 from_intervals.
        let scale = Scale::new(Note::new(NoteLetter::C, 0), ScaleType::HarmonicMinor);
        let triads = scale.diatonic_triads().expect("heptatonic");
        assert_eq!(triads[2].quality, crate::quality::Quality::Augmented);
        assert_eq!(triads[2].root, Note::new(NoteLetter::E, -1));
    }

    #[test]
    fn test_the_three_new_qualities_have_producers() {
        use crate::quality::Quality;

        let melodic = Scale::new(Note::new(NoteLetter::C, 0), ScaleType::MelodicMinor);
        assert_eq!(
            melodic.diatonic_sevenths().expect("heptatonic")[0].quality,
            Quality::MinorMajorSeventh
        );

        let harmonic = Scale::new(Note::new(NoteLetter::C, 0), ScaleType::HarmonicMinor);
        assert_eq!(
            harmonic.diatonic_sevenths().expect("heptatonic")[6].quality,
            Quality::DiminishedSeventh
        );

        let lydian_aug = Scale::new(Note::new(NoteLetter::C, 0), ScaleType::LydianAugmented);
        assert_eq!(
            lydian_aug.diatonic_sevenths().expect("heptatonic")[0].quality,
            Quality::AugmentedMajorSeventh
        );
    }

    #[test]
    fn test_non_heptatonic_scales_have_no_diatonic_chords() {
        let non_heptatonic = [
            ScaleType::MajorBlues,
            ScaleType::MinorBlues,
            ScaleType::WholeTone,
            ScaleType::Augmented,
            ScaleType::DiminishedHalfWhole,
            ScaleType::DiminishedWholeHalf,
        ];
        for scale_type in non_heptatonic {
            let scale = Scale::new(Note::new(NoteLetter::C, 0), scale_type);
            assert!(scale.diatonic_triads().is_none(), "{scale_type} triads");
            assert!(scale.diatonic_sevenths().is_none(), "{scale_type} sevenths");
        }
    }

    #[test]
    fn test_every_heptatonic_scale_derives_chords_without_panicking() {
        for scale_type in ScaleType::iter().filter(|t| t.formula().len() == 7) {
            let scale = Scale::new(Note::new(NoteLetter::C, 0), scale_type);
            let triads = scale.diatonic_triads();
            let sevenths = scale.diatonic_sevenths();
            assert_eq!(
                triads.as_ref().map(Vec::len),
                Some(7),
                "{scale_type} triads should name a quality at every degree"
            );
            assert_eq!(
                sevenths.as_ref().map(Vec::len),
                Some(7),
                "{scale_type} sevenths should name a quality at every degree"
            );
        }
    }

    use crate::chord::Chord;
    use crate::quality::Quality;

    use super::scales_containing;

    fn contains_scale(scales: &[Scale], root: (NoteLetter, i32), scale_type: ScaleType) -> bool {
        scales
            .iter()
            .any(|s| s.root == Note::new(root.0, root.1) && s.scale_type == scale_type)
    }

    #[test]
    fn test_finds_the_obvious_homes_of_c_major_seventh() {
        let cmaj7 = Chord::new(Note::new(NoteLetter::C, 0), Quality::MajorSeventh);
        let scales = scales_containing(&cmaj7);

        assert!(contains_scale(&scales, (NoteLetter::C, 0), ScaleType::Ionian));
        assert!(contains_scale(&scales, (NoteLetter::C, 0), ScaleType::Lydian));
        assert!(contains_scale(&scales, (NoteLetter::G, 0), ScaleType::Ionian));
    }

    #[test]
    fn test_excludes_scales_that_do_not_contain_the_chord() {
        let cmaj7 = Chord::new(Note::new(NoteLetter::C, 0), Quality::MajorSeventh);
        let scales = scales_containing(&cmaj7);

        // C phrygian has Eb and Bb; it cannot host a C major seventh.
        assert!(!contains_scale(&scales, (NoteLetter::C, 0), ScaleType::Phrygian));
    }

    #[test]
    fn test_results_are_deduplicated_by_pitch_class() {
        let cmaj7 = Chord::new(Note::new(NoteLetter::C, 0), Quality::MajorSeventh);
        let scales = scales_containing(&cmaj7);

        let mut seen: Vec<(ScaleType, i32)> = scales
            .iter()
            .map(|s| (s.scale_type, s.root.to_semitones().rem_euclid(12)))
            .collect();
        let before = seen.len();
        seen.sort_by_key(|(t, pc)| (*t as usize, *pc));
        seen.dedup();
        assert_eq!(before, seen.len(), "no scale should appear twice");
    }

    #[test]
    fn test_prefers_the_simpler_spelling() {
        // F7 is F A C Eb, so it lives in Bb ionian (F is its fifth degree).
        // A# ionian is the same pitch classes spelled with far more
        // accidentals, so the tiebreak must reject it.
        let f7 = Chord::new(Note::new(NoteLetter::F, 0), Quality::Dominant);
        let scales = scales_containing(&f7);

        assert!(contains_scale(&scales, (NoteLetter::B, -1), ScaleType::Ionian));
        assert!(!contains_scale(&scales, (NoteLetter::A, 1), ScaleType::Ionian));
    }

    #[test]
    fn test_prefers_flat_when_accidental_counts_tie() {
        // F# major is F# A# C#. Both F# ionian and Gb ionian cost 6
        // accidentals; only sharp_root_penalty breaks the tie, and it must
        // favour the flat spelling.
        let f_sharp_major = Chord::new(Note::new(NoteLetter::F, 1), Quality::Major);
        let scales = scales_containing(&f_sharp_major);

        assert!(contains_scale(&scales, (NoteLetter::G, -1), ScaleType::Ionian));
        assert!(!contains_scale(&scales, (NoteLetter::F, 1), ScaleType::Ionian));
    }

    #[test]
    fn test_never_empty_for_any_chord_the_practice_modes_can_produce() {
        for root in crate::note::generate_all_roots() {
            for quality in Quality::iter() {
                let chord = Chord::new(root, quality);
                assert!(
                    !scales_containing(&chord).is_empty(),
                    "follow mode would render an empty panel for {chord}"
                );
            }
        }
    }

    #[test]
    fn test_results_come_back_in_catalog_order() {
        let c = Chord::new(Note::new(NoteLetter::C, 0), Quality::Major);
        let scales = scales_containing(&c);
        let order: Vec<usize> = scales.iter().map(|s| s.scale_type as usize).collect();
        let mut sorted = order.clone();
        sorted.sort();
        assert_eq!(order, sorted, "results must be grouped by catalog position");
    }
}
