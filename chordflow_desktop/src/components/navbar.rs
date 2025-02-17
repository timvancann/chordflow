use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Navbar() -> Element {
    rsx! {

            div {
                id: "navbar",
    class: "text-red-500",
                Link {
                    to: Route::Home {},
                    "Home"
                }
                Link {
                    to: Route::Blog { id: 1 },
                    "Blog"
                }
            }

            Outlet::<Route> {}
        }
}
