use chordflow_shared::metronome::MetronomeCommand;
use dioxus::prelude::*;

use dioxus_free_icons::{
    icons::hi_solid_icons::{HiMinusCircle, HiPlusCircle},
    Icon,
};

use crate::{components::buttons::Button, MetronomeSignal, MetronomeState};

#[component]
pub fn MetronomSettingsDisplay() -> Element {
    let metronome_state: Signal<MetronomeState> = use_context();
    fn change_speed(command: MetronomeCommand) {
        let metronome: MetronomeSignal = use_context();
        let _ = metronome.read().0.send(command);
    }
    rsx! {
        div { class: "flex-col",
            div { class: "flex items-center justify-center align-middle space-x-4",
                Button {
                    icon: rsx! {
                        Icon { icon: HiMinusCircle }
                    },
                    onclick: |_| {
                        let mut metronome_state: Signal<MetronomeState> = use_context();
                        metronome_state.write().bpm -= 2;
                        change_speed(MetronomeCommand::DecreaseBpm(2));
                    },
                }
                div { class: "space-x-1 align-middle inline-block",

                    span { class: "font-bold text-lg", {metronome_state.read().bpm.to_string()} }
                    span { "bpm" }
                }
                div {
                    class: "button space-x-1 flex align-middle items-center",
                    onclick: |_| {
                        let mut metronome_state: Signal<MetronomeState> = use_context();
                        metronome_state.write().bpm += 2;
                        change_speed(MetronomeCommand::IncreaseBpm(2));
                    },
                    Icon { icon: HiPlusCircle }
                }
            }
        }
    }
}
