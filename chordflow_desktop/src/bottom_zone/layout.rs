use dioxus::prelude::*;

use crate::bottom_zone::{controls::PlayControls, quality::CircleOfFourthsQuality};

pub fn BottomZone() -> Element{
    rsx! {

        div { class: "bottom-zone",
            div { class: "zone-content",
                // Left: Mode selector
                div { class: "control-group-left",
                    span { class: "label-small", "Mode" }
                    select { class: "select-styled",
                        option { "Circle of Fourths" }
                        option { "Diatonic Progression" }
                        option { "Random Chords" }
                    }
                }
                PlayControls {}

                // Right: Quality selector
                div { class: "control-group-right",
                    CircleOfFourthsQuality{}
                }
            }
        }
    }
}
