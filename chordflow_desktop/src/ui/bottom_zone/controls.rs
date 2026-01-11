use dioxus::prelude::*;

use crate::ui::app::MetronomeState;

pub fn PlayControls() -> Element {
    let mut metronome_state: Signal<MetronomeState> = use_context();

    rsx! {
        div { class: "control-group-center",
            label {
                class: "flex items-center space-x-2 cursor-pointer",
                input {
                    r#type: "checkbox",
                    class: "cursor-pointer",
                    checked: metronome_state.read().count_in_enabled,
                    onchange: move |evt| {
                        metronome_state.write().count_in_enabled = evt.checked();
                    }
                }
                span { "Count-in" }
            }
        }
    }
}
