#![allow(non_snake_case)]

use chordflow_music_theory::{
    note::{Note, NoteLetter},
    scale::{ScaleFamily, ScaleType},
};
use dioxus::prelude::*;

use crate::ui::reference::{
    family_tabs::FamilyTabs, key_selector::KeySelector, legend::ReferenceLegend,
    scale_table::ScaleTable,
};

/// What the reference screen is currently showing. Provided by context at the
/// app level rather than owned by `ReferenceScreen`, so switching to the
/// practice screen and back does not reset the key you were reading in.
///
/// In memory only, like every other setting in this app.
pub struct ReferenceState {
    pub root: Note,
    pub family: ScaleFamily,
    /// The one scale whose chords are open, if any.
    pub expanded: Option<ScaleType>,
}

impl Default for ReferenceState {
    fn default() -> Self {
        Self {
            // G, because that is the key the Scales poster is printed in.
            root: Note::new(NoteLetter::G, 0),
            family: ScaleFamily::Major,
            expanded: None,
        }
    }
}

impl ReferenceState {
    pub fn select_root(&mut self, root: Note) {
        self.root = root;
    }

    /// Changing family closes any open row, since that row belongs to the
    /// family you just left.
    pub fn select_family(&mut self, family: ScaleFamily) {
        self.family = family;
        self.expanded = None;
    }

    pub fn toggle_expanded(&mut self, scale_type: ScaleType) {
        self.expanded = if self.expanded == Some(scale_type) {
            None
        } else {
            Some(scale_type)
        };
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
    fn test_default_opens_on_g_major_with_nothing_expanded() {
        let state = ReferenceState::default();
        assert_eq!(state.root, Note::new(NoteLetter::G, 0));
        assert_eq!(state.family, ScaleFamily::Major);
        assert_eq!(state.expanded, None);
    }

    #[test]
    fn test_toggle_expanded_opens_then_closes() {
        let mut state = ReferenceState::default();

        state.toggle_expanded(ScaleType::Dorian);
        assert_eq!(state.expanded, Some(ScaleType::Dorian));

        state.toggle_expanded(ScaleType::Lydian);
        assert_eq!(state.expanded, Some(ScaleType::Lydian));

        state.toggle_expanded(ScaleType::Lydian);
        assert_eq!(state.expanded, None);
    }

    #[test]
    fn test_changing_family_closes_the_open_row() {
        let mut state = ReferenceState::default();
        state.toggle_expanded(ScaleType::Dorian);

        state.select_family(ScaleFamily::Other);

        assert_eq!(state.family, ScaleFamily::Other);
        assert_eq!(state.expanded, None, "dorian is not in the Other family");
    }

    #[test]
    fn test_changing_key_leaves_the_open_row_alone() {
        let mut state = ReferenceState::default();
        state.toggle_expanded(ScaleType::Dorian);

        state.select_root(Note::new(NoteLetter::E, -1));

        assert_eq!(state.root, Note::new(NoteLetter::E, -1));
        assert_eq!(state.expanded, Some(ScaleType::Dorian));
    }
}
