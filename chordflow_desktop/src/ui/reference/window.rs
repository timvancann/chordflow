#![allow(non_snake_case)]

use dioxus::desktop::{
    tao::platform::macos::WindowBuilderExtMacOS, use_window, Config, LogicalSize, WindowBuilder,
};
use dioxus::prelude::*;

use crate::{
    ui::{
        app::{MAIN_CSS, TAILWIND_CSS},
        reference::{
            chord_popup::{ChordPopup, SelectedChord},
            layout::{ReferenceScreen, ReferenceState},
        },
    },
    ReferenceEvent, REFERENCE_EVT,
};

/// Root of the detached reference window.
///
/// This renders in its own `VirtualDom`, so nothing from the main window's
/// scope tree reaches it: it provides its own state, and re-declares the
/// stylesheets, because a second window is a second webview with an empty
/// document.
///
/// It is deliberately not the whole app. There is no menu bar, no view switch
/// and no transport — the reference page is a companion to the practice
/// window, and two metronome UIs driving one global audio engine would be a
/// mess.
#[component]
pub fn ReferenceWindowRoot(seed: ReferenceState) -> Element {
    let reference_state = use_context_provider(|| Signal::new(seed));
    use_context_provider(|| Signal::new(SelectedChord::default()));

    let window = use_window();

    // Fires when this window's VirtualDom is dropped, which happens whether the
    // page was attached with the button or the window was closed outright —
    // dioxus-desktop drops the whole `WebviewInstance`, and the dom with it.
    //
    // Both routes landing here is what makes closing and attaching genuinely
    // the same action rather than two behaviours kept in agreement.
    use_drop(move || {
        let _ = REFERENCE_EVT
            .0
            .try_send(ReferenceEvent::Attached(*reference_state.peek()));
    });

    rsx! {
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        div { class: "app-container",
            div { class: "panel-bar",
                span { class: "menu-title", "Reference" }
                button {
                    class: "settings-button",
                    onclick: move |_| window.close(),
                    "Attach to main window"
                }
            }
            ReferenceScreen {}
            ChordPopup {}
        }
    }
}

/// Open the reference page in its own window, seeded with what the main window
/// was showing.
pub fn detach_reference_window(seed: ReferenceState) {
    let mut builder = WindowBuilder::new()
        .with_title("ChordFlow Reference")
        .with_inner_size(LogicalSize {
            width: 900.0,
            height: 720.0,
        });

    #[cfg(target_os = "macos")]
    {
        builder = builder.with_title_hidden(false);
    }

    let dom = VirtualDom::new_with_props(ReferenceWindowRoot, ReferenceWindowRootProps { seed });
    dioxus::desktop::window().new_window(dom, Config::new().with_window(builder));
}
