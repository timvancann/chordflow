use dioxus::prelude::*;

use crate::top_zone::{
    bar_counter::BarCounter, beat_fraction::BeatFraction, beat_viz::BeatViz,
    bpm_control::BeatControl,
};

pub fn TopZone() -> Element {
    rsx! {
        div { class: "top-zone",
            div { class: "zone-content",
                BarCounter {}
                BeatViz {}
                BeatFraction {}
                BeatControl {}
            }
        }
    }
}
