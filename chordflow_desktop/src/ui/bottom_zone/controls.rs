use dioxus::prelude::*;

use crate::ui::app::AppState;

pub fn PlayControls() -> Element {
    let _app_state: Signal<AppState> = use_context();

    rsx! {
        div { class: "control-group-center" }
    }
}
