use dioxus::prelude::*;

use crate::bottom_zone::{
    controls::PlayControls, mode_selector::ModeSelector, quality::CircleOfFourthsQuality,
};

pub fn BottomZone() -> Element {
    rsx! {

        div { class: "bottom-zone",
            div { class: "zone-content",
                // Left: Mode selector
                ModeSelector {}
                PlayControls {}

                // Right: Quality selector
                div { class: "control-group-right",
                    CircleOfFourthsQuality{}
                }
            }
        }
    }
}
