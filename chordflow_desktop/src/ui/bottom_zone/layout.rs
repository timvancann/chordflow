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
            // Three slots: an empty spacer, the play controls, and the mode
            // panel. The spacer is what keeps the controls visually centred
            // while leaving them in the flex flow, so a wide panel pushes them
            // aside instead of rendering underneath.
            div { class: "zone-content",
                div { class: "zone-slot" }

                PlayControls {}

                div { class: "zone-slot zone-slot-right",
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
                        // Random's chip rows are far too wide for a slot; they
                        // take the full-width row below instead.
                        ModeOption::Random => {
                            rsx! {}
                        }
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
