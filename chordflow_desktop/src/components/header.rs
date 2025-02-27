use dioxus::prelude::*;

#[component]
pub fn Header() -> Element {
    rsx! {

        div { class: "w-screen bg-tokyoNight-bg_highlight/70",
            p { class: "font-mono font-bold text-3xl p-3 text-tokyoNight-blue", "ChordFlow" }
        }
    }
}
