#![allow(non_snake_case)]

use dioxus::prelude::*;

use crate::{
    state::modes::ModeOption,
    ui::{
        app::AppState,
        bottom_zone::{
            controls::PlayControls, diatonic::DiatonicSelector, fourths::CircleOfFourthsQuality,
            progression::ProgressionSelector, random::RandomSelector,
        },
    },
};

pub fn BottomZone() -> Element {
    let app_state: Signal<AppState> = use_context();
    rsx! {

        div { class: "bottom-zone",
            div { class: "zone-content",
                // Play controls. Note the count-in inside this is absolutely
                // positioned at the horizontal centre, so anything sharing this
                // row must stay clear of the middle.
                PlayControls {}

                // Right: mode-specific controls, for modes whose controls are
                // narrow enough to sit beside the centred count-in.
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
                    ModeOption::Random => {
                        rsx! {}
                    }
                }
            }

            // Random's two chip rows are far too wide to share a row with the
            // centred count-in, so they get the full width underneath.
            if app_state.read().selected_mode == ModeOption::Random {
                div { class: "zone-content zone-wide",
                    RandomSelector {}
                }
            }
        }
    }
}
