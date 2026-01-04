use chordflow_music_theory::quality::Quality;
use dioxus::prelude::*;
use strum::IntoEnumIterator;

use crate::{
    state::{config::ConfigState, mode::Mode, practice::PracticeState},
    ui::app::AppState,
};

pub fn CircleOfFourthsQuality() -> Element {
    let mut practice_state = use_context::<Signal<PracticeState>>();
    let mut config_state = use_context::<Signal<AppState>>();

    rsx! {

        span { class: "label-small", "Quality" }
        select {
            class: "select-styled",
            onchange: move |e| {
                let value = e.value();
                let q = Quality::from_name(&value);
                config_state.write().fourths_quality = q;
                practice_state.write().set_mode(Mode::Fourths(q));
            },
            for quality in Quality::iter() {
                option { selected: quality == config_state.read().fourths_quality, "{quality.name()}" }
            }
        }
    }
}
