#![allow(non_snake_case)]

use chordflow_music_theory::fretboard::positions;
use dioxus::prelude::*;

use crate::ui::reference::layout::ReferenceState;

/// How much of the neck to draw. The poster shows about this much, and it puts
/// the twelfth-fret double inlay comfortably inside the diagram rather than at
/// the edge.
const FRETS: usize = 14;
/// Standard guitar inlays. Twelve is doubled and handled separately.
const INLAYS: [usize; 4] = [3, 5, 7, 9];
const DOUBLE_INLAY: usize = 12;

// Geometry, in SVG user units. The viewBox scales to the container, so these
// are proportions rather than pixels.
const FRET_WIDTH: f64 = 46.0;
const STRING_GAP: f64 = 26.0;
const PAD_LEFT: f64 = 42.0;
const PAD_TOP: f64 = 26.0;
const DOT_RADIUS: f64 = 10.0;
const STRINGS: usize = 6;

/// The selected scale drawn across a standard-tuned neck, degree-labelled.
///
/// Draws only: `chordflow_music_theory::fretboard` decides where every dot
/// goes and which ones are thirds or fifths.
#[component]
pub fn Fretboard() -> Element {
    let reference_state = use_context::<Signal<ReferenceState>>();
    let state = reference_state.read();
    let scale = state.scale();
    let root = state.root;
    let name = state.selected.display_name();
    drop(state);

    let found = positions(&scale, FRETS);

    let board_right = PAD_LEFT + FRET_WIDTH * FRETS as f64;
    let board_bottom = PAD_TOP + STRING_GAP * (STRINGS - 1) as f64;
    let width = board_right + 16.0;
    let height = board_bottom + 34.0;

    rsx! {
        div { class: "fretboard",
            div { class: "fretboard-caption",
                span { class: "reference-label", "fretboard" }
                span { class: "fretboard-title mono", "{root} {name}" }
            }
            svg {
                class: "fretboard-svg",
                view_box: "0 0 {width} {height}",
                preserve_aspect_ratio: "xMinYMid meet",

                // Frets. The nut is thicker and lighter, the way it looks.
                {
                    (0..=FRETS)
                        .map(|fret| {
                            let x = PAD_LEFT + FRET_WIDTH * fret as f64;
                            let is_nut = fret == 0;
                            rsx! {
                                line {
                                    key: "fret-{fret}",
                                    x1: "{x}", y1: "{PAD_TOP}", x2: "{x}", y2: "{board_bottom}",
                                    stroke: if is_nut { "#e5e5e5" } else { "#3a3a3a" },
                                    stroke_width: if is_nut { "3" } else { "1" },
                                }
                            }
                        })
                }

                // Position markers, both the inlay dot and the fret number.
                {
                    INLAYS
                        .iter()
                        .map(|fret| {
                            let x = PAD_LEFT + FRET_WIDTH * (*fret as f64 - 0.5);
                            rsx! {
                                g { key: "inlay-{fret}",
                                    circle { cx: "{x}", cy: "{board_bottom + 14.0}", r: "2.5", fill: "#3a3a3a" }
                                    text {
                                        x: "{x}", y: "{PAD_TOP - 9.0}",
                                        fill: "#525252", font_size: "9",
                                        text_anchor: "middle", font_family: "var(--font-mono)",
                                        "{fret}"
                                    }
                                }
                            }
                        })
                }
                {
                    {
                        let x = PAD_LEFT + FRET_WIDTH * (DOUBLE_INLAY as f64 - 0.5);
                        rsx! {
                            g {
                                circle { cx: "{x - 4.5}", cy: "{board_bottom + 14.0}", r: "2.5", fill: "#3a3a3a" }
                                circle { cx: "{x + 4.5}", cy: "{board_bottom + 14.0}", r: "2.5", fill: "#3a3a3a" }
                                text {
                                    x: "{x}", y: "{PAD_TOP - 9.0}",
                                    fill: "#525252", font_size: "9",
                                    text_anchor: "middle", font_family: "var(--font-mono)",
                                    "{DOUBLE_INLAY}"
                                }
                            }
                        }
                    }
                }

                // Strings, thickening toward the low E.
                {
                    (0..STRINGS)
                        .map(|string| {
                            let y = PAD_TOP + STRING_GAP * string as f64;
                            let thickness = 0.6 + string as f64 * 0.22;
                            rsx! {
                                line {
                                    key: "string-{string}",
                                    x1: "{PAD_LEFT}", y1: "{y}", x2: "{board_right}", y2: "{y}",
                                    stroke: "#4a4a4a", stroke_width: "{thickness}",
                                }
                            }
                        })
                }

                // Degree dots. Open strings sit left of the nut.
                {
                    found
                        .iter()
                        .map(|position| {
                            let y = PAD_TOP + STRING_GAP * position.string as f64;
                            let x = if position.fret == 0 {
                                PAD_LEFT - 19.0
                            } else {
                                PAD_LEFT + FRET_WIDTH * (position.fret as f64 - 0.5)
                            };
                            let is_root = position.degree == "R";
                            let (fill, stroke, text_fill) = if is_root {
                                ("#00d9ff", "#00d9ff", "#0a0a0a")
                            } else if position.is_third_or_fifth {
                                ("rgba(255,140,50,0.22)", "#ff8c32", "#ff8c32")
                            } else {
                                ("rgba(255,255,255,0.05)", "#4a4a4a", "#8a8a8a")
                            };
                            rsx! {
                                g { key: "dot-{position.string}-{position.fret}",
                                    circle {
                                        cx: "{x}", cy: "{y}", r: "{DOT_RADIUS}",
                                        fill: "{fill}", stroke: "{stroke}", stroke_width: "1.2",
                                    }
                                    text {
                                        x: "{x}", y: "{y + 3.4}",
                                        fill: "{text_fill}", font_size: "9.5", font_weight: "500",
                                        text_anchor: "middle", font_family: "var(--font-mono)",
                                        "{position.degree}"
                                    }
                                }
                            }
                        })
                }
            }
        }
    }
}
