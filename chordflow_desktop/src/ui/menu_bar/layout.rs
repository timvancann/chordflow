use dioxus::prelude::*;

use crate::{
    components::settings_panel::SettingsPanel,
    state::view::View,
    ui::{app::AppState, menu_bar::mode_selector::ModeSelector},
};

#[component]
pub fn MenuBar() -> Element {
    let mut show_settings = use_signal(|| false);
    let mut app_state = use_context::<Signal<AppState>>();
    let view = app_state.read().view;

    rsx! {
        div { class: "menu-bar",
            div { class: "menu-left",
                // The practice-mode selector has nothing to act on while the
                // reference screen is up, so it gives way to a title.
                match view {
                    View::Practice => rsx! {
                        ModeSelector {}
                    },
                    View::Reference => rsx! {
                        span { class: "menu-title", "Reference" }
                    },
                }
            }
            div { class: "menu-right",
                button {
                    class: "settings-button",
                    onclick: move |_| {
                        let next = match app_state.read().view {
                            View::Practice => View::Reference,
                            View::Reference => View::Practice,
                        };
                        app_state.write().view = next;
                    },
                    match view {
                        View::Practice => "Reference",
                        View::Reference => "Back to practice",
                    }
                }
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
