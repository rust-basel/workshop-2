use dioxus::prelude::*;

fn main() {
    launch(App);
}

#[allow(non_snake_case)]
pub fn App() -> Element {
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

use model::{PostShopItem, ShoppingListItem};

async fn get_items() -> Result<Vec<ShoppingListItem>, reqwest::Error> {
    let url = "http://localhost:3001/items";
    let list = reqwest::get(url)
        .await?
        .json::<Vec<ShoppingListItem>>()
        .await;

    list
}

async fn post_item(item: PostShopItem) -> Result<ShoppingListItem, reqwest::Error> {
    let response = reqwest::Client::new()
        .post("http://localhost:3001/items")
        .json(&item)
        .send()
        .await?
        .json::<ShoppingListItem>()
        .await?;

    Ok(response)
}

#[component]
fn ShoppingListItemComponent(display_name: String, posted_by: String) -> Element {
    rsx! {
        div {
            class: "flex items-center space-x-2",
            p {
                class: "grow text-2xl",
                "{display_name}"
            }
            span {
                "posted by {posted_by}"
            }
        }
    }
}

#[component]
fn ItemInput(change_signal: Signal<ListChanged>) -> Element {
    let mut item = use_signal(|| "".to_string());
    let mut author = use_signal(|| "".to_string());

    let onsubmit = move |_| {
        spawn({
            async move {
                let item_name = item.read().to_string();
                let author = author.read().to_string();
                let _ = post_item(PostShopItem {
                    title: item_name,
                    posted_by: author,
                })
                .await;
            }
        });
    };

    rsx! {
        div {
            class: "w-300 m-4 mt-16 rounded",
            form { class: "grid grid-cols-3 gap-2",
                onsubmit: onsubmit,
                div {
                    input {
                        value: "{item}",
                        class: "input input-bordered input-primary w-full",
                        placeholder: "next item..",
                        r#type: "text",
                        id: "item_name",
                        name: "item_name",
                        oninput: move |e| item.set(e.data.value().clone())
                    }
                }
                div {
                    input {
                        value: "{author}",
                        class: "input input-bordered input-primary w-full",
                        placeholder: "wanted by..",
                        r#type: "text",
                        id: "author",
                        name: "author",
                        oninput: move |e| author.set(e.data.value().clone())
                    }
                }
                button {
                    class: "btn btn-primary w-full",
                    r#type: "submit",
                    "Commit"
                }
            }
        }
    }
}

struct ListChanged;

#[component]
fn ShoppingList(change_signal: Signal<ListChanged>) -> Element {
    let items_request = use_resource(move || async move { get_items().await });

    match &*items_request.read_unchecked() {
        Some(Ok(list)) => rsx! {
            div { class: "grid place-items-center min-h-500",
                ul {
                    class: "menu bg-base-200 w-200 rounded-box gap-1",
                    for i in list {
                        li {
                            key: "{i.uuid}",
                            ShoppingListItemComponent{
                                display_name: i.title.clone(),
                                posted_by: i.posted_by.clone()
                            },
                        }
                    }
                }
            }
        },
        Some(Err(err)) => {
            rsx! {
                p {
                    "Error: {err}"
                }
            }
        }
        None => {
            rsx! {
                p {
                    "Loading items..."
                }
            }
        }
    }
}
