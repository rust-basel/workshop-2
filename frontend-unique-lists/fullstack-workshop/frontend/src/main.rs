use dioxus::prelude::*;

use crate::components::{ItemInput, Layout, ListChanged, Profile, ShoppingList};

mod components;
mod controllers;

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

#[allow(non_snake_case)]
fn App() -> Element {
    rsx! {
        Router::<Route>{}
    }
}

#[allow(non_snake_case)]
pub fn Home() -> Element {
    let change_signal = use_signal(|| ListChanged);
    rsx! {
        ShoppingList{change_signal}
        ItemInput{change_signal}
    }
}
