use dioxus::prelude::*;

use crate::ui::app::AppState;

pub fn CurrentChord() -> Element {
    let app_state: Signal<AppState> = use_context();
    let (chord, _) = app_state.read().get_chords();

    rsx! {
        div { class: "current-chord",
            "{chord}"
        }
    }
}
