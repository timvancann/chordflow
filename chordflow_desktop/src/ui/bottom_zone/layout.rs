#![allow(non_snake_case)]

use dioxus::prelude::*;

use crate::{
    state::modes::ModeOption,
    ui::{
        app::AppState,
        bottom_zone::{
            diatonic::DiatonicSelector, fourths::CircleOfFourthsQuality,
            progression::ProgressionSelector, random::RandomSelector,
        },
    },
};

pub fn BottomZone() -> Element {
    let app_state: Signal<AppState> = use_context();

    rsx! {
        div { class: "bottom-zone",
            // Only mode controls live here now that the count-in has moved up
            // to the metronome settings, so a panel gets the whole width and
            // cannot collide with anything.
            div { class: "zone-content",
                match app_state.read().selected_mode {
                    ModeOption::Fourths => {
                        rsx! {
                            CircleOfFourthsQuality {}
                        }
                    }
                    ModeOption::Diatonic => {
                        rsx! {
                            DiatonicSelector {}
                        }
                    }
                    ModeOption::Custom => {
                        rsx! {
                            ProgressionSelector {}
                        }
                    }
                    ModeOption::Random => {
                        rsx! {
                            RandomSelector {}
                        }
                    }
                }
            }
        }
    }
}
