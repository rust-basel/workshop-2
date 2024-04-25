use dioxus::prelude::*;

use crate::components::{Home, Layout, Profile};

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
    Home {},
    #[route("/profile")]
    Profile {},
}

#[component]
fn App() -> Element {
    rsx! {
        Router::<Route>{}
    }
}
