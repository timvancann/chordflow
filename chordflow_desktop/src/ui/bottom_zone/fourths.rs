use chordflow_music_theory::quality::Quality;
use dioxus::prelude::*;
use strum::IntoEnumIterator;

use crate::{state::fourths::FourthsConfig, ui::app::AppState};

pub fn CircleOfFourthsQuality() -> Element {
    let mut app_state = use_context::<Signal<AppState>>();

    rsx! {

        span { class: "label-small", "Quality" }
        select {
            class: "select-styled",
            onchange: move |e| {
                let value = e.value();
                let q = Quality::from_name(&value);
                app_state.write().fourths_config = FourthsConfig::new(q);
            },
            for quality in Quality::iter() {
                option { selected: quality == app_state.read().fourths_config.quality, "{quality.name()}" }
            }
        }
    }
}
