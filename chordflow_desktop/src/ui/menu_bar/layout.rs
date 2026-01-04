use dioxus::prelude::*;

use crate::ui::menu_bar::mode_selector::ModeSelector;

pub fn MenuBar() -> Element {
    rsx! {
        div { class: "menu-bar",
            div { class: "menu-left",
                ModeSelector {}
            }
            div { class: "menu-right",
                button {
                    class: "settings-button",
                    "⚙️ Settings"
                }
            }
        }
    }
}
