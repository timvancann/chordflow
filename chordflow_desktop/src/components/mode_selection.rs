use chordflow_shared::ModeOption;
use dioxus::prelude::*;
use strum::IntoEnumIterator;

use crate::components::{apply_selected_changes, buttons::ToggleButton};

#[component]
pub fn ModeSelectionDisplay() -> Element {
    let selected_mode: Signal<ModeOption> = use_context();
    rsx! {
        div { class: "space-y-4 w-60",
            p { class: "text-tokyoNight-blue font-bold text-xl", "Practice Mode" }
            div { class: "flex-col justify-center space-y-2",
                for mode in ModeOption::iter() {

                    ToggleButton {
                        text: mode.to_string(),
                        is_selected: mode == selected_mode(),
                        onclick: move |_| {
                            let mut selected_mode: Signal<ModeOption> = use_context();
                            selected_mode.set(mode);
                            apply_selected_changes();
                        },
                    }
                }
            }
        }
    }
}
