#![allow(non_snake_case)]

use dioxus::prelude::*;

use crate::{
    state::modes::ModeOption,
    ui::{
        app::AppState,
        bottom_zone::{
            controls::PlayControls, diatonic::DiatonicSelector, fourths::CircleOfFourthsQuality,
            progression::ProgressionSelector,
        },
    },
};

pub fn BottomZone() -> Element {
    let app_state: Signal<AppState> = use_context();
    rsx! {

        div { class: "bottom-zone",
            div { class: "zone-content",
                // Left: Play controls
                PlayControls {}

                // Right: Mode-specific controls
                match app_state.read().selected_mode {
                    ModeOption::Fourths => {
                        rsx! {div { class: "control-group-right", CircleOfFourthsQuality{} }}
                    }
                    ModeOption::Diatonic => {
                        rsx! { DiatonicSelector {} }
                    }
                    ModeOption::Custom => {
                        rsx! { ProgressionSelector {} }
                    }

                    _ => {
                        rsx!{div { class: "control-group-right", }}
                    }
                }
            }
        }
    }
}
