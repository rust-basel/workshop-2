use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use model::{PostShopItem, ShoppingListItem};
use uuid::Uuid;

use crate::{database::ShoppingItem, Database};

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

pub async fn add_item(
    State(state): State<Database>,
    Json(post_request): Json<PostShopItem>,
) -> impl IntoResponse {
    let item = ShoppingItem {
        title: post_request.title.clone(),
        creator: post_request.posted_by.clone(),
    };
    let uuid = Uuid::new_v4().to_string();

    let Ok(mut db) = state.write() else {
        return (StatusCode::SERVICE_UNAVAILABLE).into_response();
    };

    db.insert_item(&uuid, item);

    (
        StatusCode::OK,
        Json(ShoppingListItem {
            title: post_request.title,
            posted_by: post_request.posted_by,
            uuid,
        }),
    )
        .into_response()
}
