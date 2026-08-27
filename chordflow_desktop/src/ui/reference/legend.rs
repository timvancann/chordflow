#![allow(non_snake_case)]

use chordflow_music_theory::{
    chord::Chord,
    note::{Note, NoteLetter},
    quality::{Notation, Quality},
};
use dioxus::prelude::*;
use strum::IntoEnumIterator;

/// One legend entry: a worked example and what it means.
///
/// Examples rather than bare symbols, because the major triad's symbol is the
/// empty string. "C = major" reads; a blank cell next to "major" does not.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LegendEntry {
    pub example: String,
    pub meaning: String,
}

/// Every chord quality the app can name, as a worked example on C.
///
/// Derived from `Quality::iter()` rather than written out, so a quality added
/// to the theory crate shows up here without anyone remembering to.
pub fn chord_legend(notation: Notation) -> Vec<LegendEntry> {
    let c = Note::new(NoteLetter::C, 0);

    Quality::iter()
        .map(|quality| LegendEntry {
            example: Chord::new(c, quality).symbol(notation),
            meaning: quality.name().to_lowercase(),
        })
        .collect()
}

/// The formula column's notation. Unlike the chord symbols these are not a
/// closed set in code, so this list is written out.
pub fn degree_legend() -> Vec<LegendEntry> {
    [
        ("R", "the root"),
        ("2", "a major second above it"),
        ("♭3", "flattened, a semitone down"),
        ("♯4", "sharpened, a semitone up"),
        ("♭♭7", "flattened twice"),
    ]
    .into_iter()
    .map(|(example, meaning)| LegendEntry {
        example: example.to_string(),
        meaning: meaning.to_string(),
    })
    .collect()
}

#[component]
pub fn ReferenceLegend() -> Element {
    let notation = use_context::<Signal<Notation>>();
    let notation = *notation.read();

    rsx! {
        div { class: "reference-legend",
            div { class: "legend-group",
                span { class: "reference-label", "chords" }
                div { class: "legend-entries",
                    {
                        chord_legend(notation)
                            .into_iter()
                            .map(|entry| rsx! {
                                span { key: "{entry.example}", class: "legend-entry",
                                    span { class: "legend-example mono", "{entry.example}" }
                                    span { class: "legend-meaning", "{entry.meaning}" }
                                }
                            })
                    }
                }
            }
            div { class: "legend-group",
                span { class: "reference-label", "degrees" }
                div { class: "legend-entries",
                    {
                        degree_legend()
                            .into_iter()
                            .map(|entry| rsx! {
                                span { key: "{entry.example}", class: "legend-entry",
                                    span { class: "legend-example mono", "{entry.example}" }
                                    span { class: "legend-meaning", "{entry.meaning}" }
                                }
                            })
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chord_legend_covers_every_quality() {
        assert_eq!(
            chord_legend(Notation::default()).len(),
            Quality::iter().count()
        );
    }

    #[test]
    fn test_chord_legend_explains_the_cryptic_symbols() {
        let entries = chord_legend(Notation::default());
        let find = |example: &str| {
            entries
                .iter()
                .find(|e| e.example == example)
                .unwrap_or_else(|| panic!("no legend entry for {example}"))
        };

        assert_eq!(find("C").meaning, "major");
        assert_eq!(find("Cm").meaning, "minor");
        assert_eq!(find("C°").meaning, "diminished");
        assert_eq!(find("C+").meaning, "augmented");
        assert_eq!(find("C7").meaning, "dominant");
        assert_eq!(find("C△7").meaning, "major seventh");
        assert_eq!(find("Cm7").meaning, "minor seventh");
        assert_eq!(find("Cø7").meaning, "half diminished");
        assert_eq!(find("C°7").meaning, "diminished seventh");
        assert_eq!(find("Cm△7").meaning, "minor major seventh");
        assert_eq!(find("C+△7").meaning, "augmented major seventh");
    }

    #[test]
    fn test_every_legend_entry_is_readable() {
        for entry in chord_legend(Notation::default())
            .into_iter()
            .chain(degree_legend())
        {
            assert!(
                !entry.example.is_empty(),
                "an empty example would render as a blank cell"
            );
            assert!(!entry.meaning.is_empty());
        }
    }
}
