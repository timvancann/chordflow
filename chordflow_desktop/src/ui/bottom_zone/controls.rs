use dioxus::prelude::*;
use dioxus_free_icons::{icons::ld_icons::LdUndo, Icon};

use crate::{state::practice::PracticeState, ui::app::MetronomeState};

pub fn PlayControls() -> Element {
    rsx! {
        div { class: "control-group-center",
            button { class: "btn-icon btn-large-icon", onclick: move |_| restart(),
                Icon { width: 20, height: 20, icon: LdUndo }
            }
        }

    }
}

fn restart() {
    let mut practice_state: Signal<PracticeState> = use_context();
    let mut metronome_state: Signal<MetronomeState> = use_context();
    // tx_audio is no longer needed via context as use global
    practice_state.write().reset();
    metronome_state.write().current_bar = 1;
    metronome_state.write().current_tick = 0;
}
