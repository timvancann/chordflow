use chordflow_music_theory::{
    chord::Chord,
    note::Note,
    scale::{Scale, ScaleFamily, ScaleType},
};
use strum::IntoEnumIterator;

/// One rendered line of the reference table. Everything here is already a
/// string: the components that display a `ScaleRow` do no music theory, they
/// only lay out text. That keeps the logic in this file, where it is tested.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScaleRow {
    pub scale_type: ScaleType,
    /// The poster's name, e.g. "lydian dominant".
    pub name: &'static str,
    /// Degree notation, e.g. "R 2 3 #4 5 6 b7".
    pub formula: String,
    /// The degrees spelled in the chosen key, e.g. "G A B C♯ D E F".
    pub notes: String,
    /// The diatonic seventh chords, kept as `Chord` rather than a rendered
    /// string so each one can be clicked, played, and looked up.
    ///
    /// Sevenths only, no triads: a seventh already contains its triad, so
    /// showing both is redundant and the sevenths are harmonically complete.
    ///
    /// `None` for the six non-heptatonic scales, where stacking thirds by
    /// scale index has no accepted meaning.
    pub sevenths: Option<Vec<Chord>>,
}

pub fn build_row(scale_type: ScaleType, root: Note) -> ScaleRow {
    let scale = Scale::new(root, scale_type);

    ScaleRow {
        scale_type,
        name: scale_type.display_name(),
        formula: join(scale.intervals.iter().map(|i| i.degree_label().to_string())),
        notes: join(scale.notes().iter().map(|n| n.to_string())),
        sevenths: scale.diatonic_sevenths(),
    }
}

/// Every scale in one poster family, in the poster's order.
pub fn rows_for_family(family: ScaleFamily, root: Note) -> Vec<ScaleRow> {
    ScaleType::iter()
        .filter(|t| t.family() == family)
        .map(|t| build_row(t, root))
        .collect()
}

fn join(parts: impl Iterator<Item = String>) -> String {
    parts.collect::<Vec<String>>().join(" ")
}

#[cfg(test)]
mod tests {
    use chordflow_music_theory::note::NoteLetter;

    use super::*;

    fn g() -> Note {
        Note::new(NoteLetter::G, 0)
    }

    #[test]
    fn test_row_carries_formula_notes_and_chords() {
        let row = build_row(ScaleType::Dorian, g());

        assert_eq!(row.name, "dorian");
        assert_eq!(row.formula, "R 2 ♭3 4 5 6 ♭7");
        assert_eq!(row.notes, "G A B\u{266d} C D E F");

        let sevenths: Vec<String> = row
            .sevenths
            .expect("dorian is heptatonic")
            .iter()
            .map(|c| c.to_string())
            .collect();
        assert_eq!(
            sevenths,
            vec![
                "G-7",
                "A-7",
                "B\u{266d}\u{394}",
                "C7",
                "D-7",
                "E\u{f8}",
                "F\u{394}"
            ]
        );
    }

    #[test]
    fn test_row_respells_per_key() {
        assert_eq!(
            build_row(ScaleType::Ionian, Note::new(NoteLetter::F, 1)).notes,
            "F\u{266f} G\u{266f} A\u{266f} B C\u{266f} D\u{266f} E\u{266f}"
        );
        assert_eq!(
            build_row(ScaleType::Ionian, Note::new(NoteLetter::G, -1)).notes,
            "G\u{266d} A\u{266d} B\u{266d} C\u{266d} D\u{266d} E\u{266d} F"
        );
    }

    #[test]
    fn test_non_heptatonic_scales_have_no_chord_strings() {
        for scale_type in [
            ScaleType::MajorBlues,
            ScaleType::MinorBlues,
            ScaleType::WholeTone,
            ScaleType::Augmented,
            ScaleType::DiminishedHalfWhole,
            ScaleType::DiminishedWholeHalf,
        ] {
            let row = build_row(scale_type, g());
            assert!(
                row.sevenths.is_none(),
                "{} should have no diatonic sevenths",
                row.name
            );
            // The formula and notes still work.
            assert!(!row.formula.is_empty());
            assert!(!row.notes.is_empty());
        }
    }

    #[test]
    fn test_families_partition_the_catalog() {
        let counts: Vec<usize> = ScaleFamily::iter()
            .map(|f| rows_for_family(f, g()).len())
            .collect();

        assert_eq!(counts, vec![7, 7, 7, 6]);
        assert_eq!(counts.iter().sum::<usize>(), ScaleType::iter().count());
    }

    #[test]
    fn test_family_rows_keep_poster_order() {
        let names: Vec<&str> = rows_for_family(ScaleFamily::Major, g())
            .iter()
            .map(|r| r.name)
            .collect();

        assert_eq!(
            names,
            vec![
                "ionian",
                "dorian",
                "phrygian",
                "lydian",
                "mixolydian",
                "aeolian",
                "locrian"
            ]
        );
    }
}
