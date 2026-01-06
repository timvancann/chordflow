use dioxus::prelude::*;

use crate::{components::settings_panel::SettingsPanel, ui::menu_bar::mode_selector::ModeSelector};

#[component]
pub fn MenuBar() -> Element {
    let mut show_settings = use_signal(|| false);

    rsx! {
        div { class: "menu-bar",
            div { class: "menu-left",
                ModeSelector {}
            }
            div { class: "menu-right",
                button {
                    class: "settings-button",
                    onclick: move |_| show_settings.set(true),
                    "⚙️ Settings"
                }
            }
        }

        SettingsPanel { show: show_settings }
    }
}
