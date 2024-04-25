use dioxus::prelude::*;

use crate::components::{ItemInput, ListChanged, ShoppingList, Profile, Layout};

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
        Profile {}
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
    let rust_basel = "Rust Basel";
    rsx! {
        h1{
            "Welcome to {rust_basel}"
        }
        button{
            class: "btn",
            "My stylish button"
        }
        ShoppingList{change_signal}
        ItemInput{change_signal}
    }
}
