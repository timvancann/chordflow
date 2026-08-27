use chordflow_music_theory::quality::Notation;
use dioxus::prelude::*;

use crate::{
    components::settings_panel::SettingsPanel,
    state::view::View,
    ui::{
        app::AppState,
        menu_bar::mode_selector::ModeSelector,
        reference::{layout::ReferenceState, window::detach_reference_window},
    },
};

#[component]
pub fn MenuBar() -> Element {
    let mut show_settings = use_signal(|| false);
    let mut app_state = use_context::<Signal<AppState>>();
    let reference_state = use_context::<Signal<ReferenceState>>();
    let notation = use_context::<Signal<Notation>>();

    let view = app_state.read().view;
    let detached = app_state.read().reference_detached;

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
                // Detaching is only offered from the page itself, the way an
                // editor lets you pull out the tab you are looking at.
                if view == View::Reference {
                    button {
                        class: "settings-button",
                        onclick: move |_| {
                            detach_reference_window(*reference_state.peek(), *notation.peek());
                            let mut app = app_state.write();
                            app.reference_detached = true;
                            app.view = View::Practice;
                        },
                        "Detach"
                    }
                }
                button {
                    class: "settings-button",
                    disabled: detached,
                    onclick: move |_| {
                        if app_state.read().reference_detached {
                            return;
                        }
                        let next = match app_state.read().view {
                            View::Practice => View::Reference,
                            View::Reference => View::Practice,
                        };
                        app_state.write().view = next;
                    },
                    // While detached the page lives in the other window, so
                    // there is nothing here to switch to.
                    if detached {
                        "Reference (detached)"
                    } else {
                        match view {
                            View::Practice => "Reference",
                            View::Reference => "Back to practice",
                        }
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
