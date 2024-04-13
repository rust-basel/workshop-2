use axum::{extract::State, response::IntoResponse, Json};
use model::ShoppingListItem;

use crate::Database;

pub async fn get_items(State(state): State<Database>) -> impl IntoResponse {
    let items: Vec<ShoppingListItem> = state
        .read()
        .unwrap()
        .as_vec()
        .iter()
        .cloned()
        .map(|(uuid, item)| ShoppingListItem {
            title: item.title,
            posted_by: item.creator,
            uuid,
        })
        .collect();

    Json(items)
}
