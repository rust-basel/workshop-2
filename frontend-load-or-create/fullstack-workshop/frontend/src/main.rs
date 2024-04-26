use dioxus::prelude::*;

use crate::components::{Home, Layout, LoadOrCreateList, Profile};

mod components;
mod controllers;

const _STYLE: &str = manganis::mg!(file("public/tailwind.css"));

fn main() {
    launch(App);
}

#[derive(Routable, Clone)]
pub enum Route {
    #[layout(Layout)]
    #[route("/")]
    LoadOrCreateList {},
    #[route("/list/:list_uuid")]
    Home { list_uuid: String },
    #[route("/profile")]
    Profile {},
}

#[component]
fn App() -> Element {
    rsx! {
        Router::<Route>{}
    }
}
