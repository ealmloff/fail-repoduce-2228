use dioxus::prelude::*;

#[component]
pub fn Component() -> Element {
    rsx! {
        div {
            "Hello, world!"
        }
    }
}