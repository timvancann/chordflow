#![allow(non_snake_case)]

use chordflow_music_theory::{
    chord::Chord,
    roman::roman_numeral,
    scale::{scale_degrees_of, ScaleFamily},
};
use dioxus::prelude::*;
use strum::IntoEnumIterator;

use crate::{ui::app::chord_to_midi, AudioCommand, AUDIO_CMD};

/// The chord whose popup is open, if any. Provided by context at the app level
/// because the trigger sits deep inside the scale table.
#[derive(Default)]
pub struct SelectedChord(pub Option<Chord>);

impl SelectedChord {
    pub fn select(&mut self, chord: Chord) {
        self.0 = Some(chord);
    }

    pub fn clear(&mut self) {
        self.0 = None;
    }
}

/// One place this chord turns up: which degree, of which scale.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DegreeEntry {
    /// A cased roman numeral, e.g. `V` or `ii`.
    pub numeral: String,
    /// The scale, e.g. "F ionian".
    pub scale: String,
}

/// The scales a chord is a degree of, grouped the way the poster groups them.
///
/// Families with no entries are dropped rather than shown empty.
pub fn degree_groups(chord: &Chord) -> Vec<(ScaleFamily, Vec<DegreeEntry>)> {
    let found = scale_degrees_of(chord);

    ScaleFamily::iter()
        .filter_map(|family| {
            let entries: Vec<DegreeEntry> = found
                .iter()
                .filter(|(scale, _)| scale.scale_type.family() == family)
                .filter_map(|(scale, degree)| {
                    Some(DegreeEntry {
                        numeral: roman_numeral(*degree, chord.quality)?,
                        scale: scale.to_string(),
                    })
                })
                .collect();

            if entries.is_empty() {
                None
            } else {
                Some((family, entries))
            }
        })
        .collect()
}

#[component]
pub fn ChordPopup() -> Element {
    let mut selected_chord = use_context::<Signal<SelectedChord>>();
    // Which families are expanded. Major starts open: it is the answer most
    // players want, and the others would open the popup dozens of rows tall.
    let mut open_families = use_signal(|| vec![ScaleFamily::Major]);

    let Some(chord) = selected_chord.read().0 else {
        return rsx! {
            div {}
        };
    };

    let tones = chord
        .notes()
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<String>>()
        .join("  ");
    let groups = degree_groups(&chord);
    let total: usize = groups.iter().map(|(_, e)| e.len()).sum();

    rsx! {
        div {
            class: "settings-overlay",
            onclick: move |_| selected_chord.write().clear(),

            div {
                class: "settings-panel chord-panel",
                onclick: move |e| e.stop_propagation(),

                div { class: "settings-header",
                    div { class: "chord-heading",
                        h2 { class: "chord-symbol mono", "{chord}" }
                        span { class: "chord-quality", "{chord.quality.name().to_lowercase()}" }
                    }
                    button {
                        class: "settings-close",
                        onclick: move |_| selected_chord.write().clear(),
                        "✕"
                    }
                }

                div { class: "settings-content",
                    div { class: "chord-tones-row",
                        div {
                            span { class: "reference-label", "notes" }
                            div { class: "chord-tones mono", "{tones}" }
                        }
                        button {
                            class: "chord-play",
                            onclick: move |_| {
                                let _ = AUDIO_CMD
                                    .0
                                    .try_send(AudioCommand::PlayChordNow(chord_to_midi(chord)));
                            },
                            "\u{25b6} play"
                        }
                    }

                    div { class: "chord-degrees",
                        span { class: "reference-label", "appears as" }
                        p { class: "detail-note",
                            "{chord} is a diatonic seventh in {total} of the catalog's scales. The modes of one parent scale share the same notes, so each parent contributes seven entries."
                        }
                        {
                            groups
                                .into_iter()
                                .map(|(family, entries)| {
                                    let is_open = open_families.read().contains(&family);
                                    let open_class = if is_open { "open" } else { "" };
                                    let count = entries.len();
                                    rsx! {
                                        div { key: "{family}", class: "degree-group",
                                            button {
                                                class: "degree-group-header {open_class}",
                                                onclick: move |_| {
                                                    let mut families = open_families.write();
                                                    if let Some(index) = families.iter().position(|f| *f == family) {
                                                        families.remove(index);
                                                    } else {
                                                        families.push(family);
                                                    }
                                                },
                                                span { class: "col-chevron", "\u{203a}" }
                                                span { "{family}" }
                                                span { class: "degree-count", "{count}" }
                                            }
                                            if is_open {
                                                div { class: "degree-entries",
                                                    {
                                                        entries
                                                            .into_iter()
                                                            .map(|entry| rsx! {
                                                                div { key: "{entry.scale}", class: "degree-entry",
                                                                    span { class: "degree-numeral mono", "{entry.numeral}" }
                                                                    span { class: "degree-scale", "of {entry.scale}" }
                                                                }
                                                            })
                                                    }
                                                }
                                            }
                                        }
                                    }
                                })
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use chordflow_music_theory::{
        note::{Note, NoteLetter},
        quality::Quality,
    };

    use super::*;

    fn c(quality: Quality) -> Chord {
        Chord::new(Note::new(NoteLetter::C, 0), quality)
    }

    #[test]
    fn test_dominant_seventh_is_the_fifth_of_its_parent_major_scale() {
        let groups = degree_groups(&c(Quality::Dominant));
        let (family, entries) = &groups[0];

        assert_eq!(*family, ScaleFamily::Major);
        assert_eq!(
            entries[0],
            DegreeEntry {
                numeral: "V".to_string(),
                scale: "F ionian".to_string(),
            }
        );
    }

    #[test]
    fn test_minor_sevenths_get_lowercase_numerals() {
        let groups = degree_groups(&c(Quality::MinorSeventh));
        let numerals: Vec<&str> = groups[0].1.iter().map(|e| e.numeral.as_str()).collect();

        assert!(
            numerals
                .iter()
                .all(|n| n.chars().all(|ch| ch.is_lowercase())),
            "a minor seventh is written lowercase, got {numerals:?}"
        );
    }

    #[test]
    fn test_groups_follow_poster_order_and_drop_empty_families() {
        let groups = degree_groups(&c(Quality::Dominant));
        let families: Vec<ScaleFamily> = groups.iter().map(|(f, _)| *f).collect();

        let expected_order: Vec<ScaleFamily> = ScaleFamily::iter()
            .filter(|f| families.contains(f))
            .collect();
        assert_eq!(families, expected_order);

        assert!(
            !families.contains(&ScaleFamily::Other),
            "the Other family is non-heptatonic, so nothing is a degree of it"
        );
        assert!(groups.iter().all(|(_, entries)| !entries.is_empty()));
    }

    #[test]
    fn test_diminished_seventh_is_far_more_constraining_than_a_dominant() {
        let count =
            |chord: Chord| -> usize { degree_groups(&chord).iter().map(|(_, e)| e.len()).sum() };

        assert!(
            count(c(Quality::DiminishedSeventh)) < count(c(Quality::Dominant)),
            "a diminished seventh pins the key down much harder"
        );
    }
}
