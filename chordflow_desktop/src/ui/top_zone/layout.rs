#![allow(non_snake_case)]

use dioxus::prelude::*;

use crate::ui::top_zone::{
    bar_counter::BarCounter, beat_fraction::BeatFraction, beat_viz::BeatViz,
    bpm_control::BeatControl, play_control::PlayControl, subdivision_selector::SubdivisionSelector,
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
                // CountIn is deliberately not rendered. The count-in still
                // works — MetronomeState keeps the flag and the audio thread
                // still honours it — but the control looked wrong in both the
                // bottom zone and here, so it is hidden until it has a home
                // worth having. Re-add `CountIn {}` to bring it back.
                PlayControl {}
            }
        }
    }
}
