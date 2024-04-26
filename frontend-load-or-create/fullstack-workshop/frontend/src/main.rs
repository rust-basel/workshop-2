use dioxus::prelude::*;

use crate::components::{Home, Layout, Profile};

mod components;
mod controllers;

fn main() {
    launch(App);
}

#[derive(Routable, Clone)]
pub enum Route {
    #[layout(Layout)]
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
