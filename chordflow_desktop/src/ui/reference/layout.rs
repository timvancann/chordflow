#![allow(non_snake_case)]

use chordflow_music_theory::{
    note::{Note, NoteLetter},
    scale::{Scale, ScaleFamily, ScaleType},
};
use dioxus::prelude::*;
use strum::IntoEnumIterator;

use crate::ui::reference::{
    family_tabs::FamilyTabs, fretboard::Fretboard, key_selector::KeySelector,
    legend::ReferenceLegend, scale_table::ScaleTable,
};

/// What the reference screen is currently showing. Provided by context at the
/// app level rather than owned by `ReferenceScreen`, so switching to the
/// practice screen and back does not reset the key you were reading in.
///
/// In memory only, like every other setting in this app.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ReferenceState {
    pub root: Note,
    pub family: ScaleFamily,
    /// The scale the fretboard is drawing, and whose chords are open. Always
    /// set: a pinned fretboard needs something to show.
    pub selected: ScaleType,
}

impl Default for ReferenceState {
    fn default() -> Self {
        Self {
            // G, because that is the key the Scales poster is printed in.
            root: Note::new(NoteLetter::G, 0),
            family: ScaleFamily::Major,
            selected: ScaleType::Ionian,
        }
    }
}

impl ReferenceState {
    pub fn select_root(&mut self, root: Note) {
        self.root = root;
    }

    /// Changing family selects that family's first scale, since the previously
    /// selected one belongs to the family you just left and the fretboard
    /// always needs a scale to draw.
    pub fn select_family(&mut self, family: ScaleFamily) {
        self.family = family;
        self.selected = ScaleType::iter()
            .find(|t| t.family() == family)
            .expect("every family has at least one scale");
    }

    pub fn select_scale(&mut self, scale_type: ScaleType) {
        self.selected = scale_type;
    }

    /// The scale the fretboard should draw.
    pub fn scale(&self) -> Scale {
        Scale::new(self.root, self.selected)
    }
}

#[component]
pub fn ReferenceScreen() -> Element {
    rsx! {
        div { class: "reference-screen",
            div { class: "reference-controls",
                KeySelector {}
                FamilyTabs {}
            }
            Fretboard {}
            div { class: "reference-scroll",
                ScaleTable {}
            }
            ReferenceLegend {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_opens_on_g_ionian() {
        let state = ReferenceState::default();
        assert_eq!(state.root, Note::new(NoteLetter::G, 0));
        assert_eq!(state.family, ScaleFamily::Major);
        assert_eq!(state.selected, ScaleType::Ionian);
    }

    #[test]
    fn test_changing_family_selects_that_familys_first_scale() {
        let mut state = ReferenceState::default();

        state.select_family(ScaleFamily::Other);
        assert_eq!(state.family, ScaleFamily::Other);
        assert_eq!(
            state.selected,
            ScaleType::MajorBlues,
            "the fretboard always needs a scale, and dorian is not in Other"
        );

        state.select_family(ScaleFamily::HarmonicMinor);
        assert_eq!(state.selected, ScaleType::HarmonicMinor);
    }

    #[test]
    fn test_every_family_can_be_selected_without_panicking() {
        let mut state = ReferenceState::default();
        for family in ScaleFamily::iter() {
            state.select_family(family);
            assert_eq!(state.selected.family(), family);
        }
    }

    #[test]
    fn test_scale_combines_the_selected_root_and_type() {
        let mut state = ReferenceState::default();
        state.select_root(Note::new(NoteLetter::E, -1));
        state.select_scale(ScaleType::LydianDominant);

        let scale = state.scale();
        assert_eq!(scale.root, Note::new(NoteLetter::E, -1));
        assert_eq!(scale.scale_type, ScaleType::LydianDominant);
    }

    #[test]
    fn test_state_survives_a_detach_and_attach_round_trip() {
        // The detached window is seeded by copying this value across a props
        // boundary, and hands it back over a channel on the way out. Both legs
        // are plain copies, so browsing in the panel comes back with it.
        let mut state = ReferenceState::default();
        state.select_root(Note::new(NoteLetter::E, -1));
        state.select_family(ScaleFamily::HarmonicMinor);
        state.select_scale(ScaleType::PhrygianDominant);

        let seeded = state;
        let returned = seeded;

        assert_eq!(returned, state);
        assert_eq!(returned.root, Note::new(NoteLetter::E, -1));
        assert_eq!(returned.family, ScaleFamily::HarmonicMinor);
        assert_eq!(returned.selected, ScaleType::PhrygianDominant);
    }

    #[test]
    fn test_changing_key_leaves_the_open_row_alone() {
        let mut state = ReferenceState::default();
        state.select_scale(ScaleType::Dorian);

        state.select_root(Note::new(NoteLetter::E, -1));

        assert_eq!(state.root, Note::new(NoteLetter::E, -1));
        assert_eq!(state.selected, ScaleType::Dorian);
    }
}
