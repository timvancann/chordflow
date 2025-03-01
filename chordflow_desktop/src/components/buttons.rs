use dioxus::prelude::*;

#[component]
pub fn Button(
    text: Option<String>,
    icon: Option<Element>,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
        button {
            class: "button space-x-1 flex align-middle items-center",
            onclick,
            if let Some(ico) = icon {
                {ico}
            }
            span {
                if let Some(title) = text {
                    {title}
                }
            }
        }
    }
}

#[component]
pub fn ToggleButton(
    text: String,
    is_selected: bool,
    onclick: EventHandler<MouseEvent>,
    is_disabled: Option<bool>,
) -> Element {
    let dis: bool = is_disabled.unwrap_or(false);
    rsx! {
        button {
            class: format!(
                "flex items-center space-x-2 {} {}",
                if is_selected { "selected-button " } else { "button" },
                if dis { "cursor-not-allowed" } else { "" },
            ),
            disabled: dis,
            onclick,
            span { {text} }
        }
    }
}
