use dioxus::prelude::*;

const _STYLE: &str = manganis::mg!(file("public/tailwind.css"));

fn main() {
    launch(App);
}

#[allow(non_snake_case)]
pub fn App() -> Element {
    let rust_basel = "Rust Basel";
    rsx! {
        h1{
            "Welcome to {rust_basel}"
        }
        button{
            class: "btn",
            "My stylish button"
        }
    }
}
