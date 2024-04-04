use dioxus::prelude::*;
use nested::Component;

fn main() {
    launch(|| rsx! {
        Component {}
    })
}
