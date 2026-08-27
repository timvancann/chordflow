#![allow(non_snake_case)]

use dioxus::prelude::*;

use crate::ui::top_zone::{
    bar_counter::BarCounter, beat_fraction::BeatFraction, beat_viz::BeatViz,
    bpm_control::BeatControl, count_in::CountIn, play_control::PlayControl,
    subdivision_selector::SubdivisionSelector,
};

pub fn TopZone() -> Element {
    rsx! {
        div { class: "top-zone",
            div { class: "zone-content",
                BarCounter {}
                BeatViz {}
                BeatFraction {}
                SubdivisionSelector {}
                BeatControl {}
                CountIn {}
                PlayControl {}
            }
        }
    }
}
