use dioxus::prelude::*;

pub fn CircleOfFourthsQuality() -> Element {
    rsx! {

        span { class: "label-small", "Quality" }
        select { class: "select-styled",
            option { "Major" }
            option { "Minor" }
            option { "Dominant 7th" }
            option { "Diminished" }
        }
    }
}
