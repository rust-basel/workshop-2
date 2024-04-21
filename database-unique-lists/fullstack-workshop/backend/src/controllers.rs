use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use model::{PostShopItem, ShoppingListItem};
use uuid::Uuid;

use crate::{database::ShoppingItem, Database};

const LIST_UUID: &str = "9e137e61-08ac-469d-be9d-6b3324dd20ad";

pub async fn get_items(State(state): State<Database>) -> impl IntoResponse {
    let items: Vec<ShoppingListItem> = state.read().unwrap().as_vec(LIST_UUID);

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
    let item_uuid = Uuid::new_v4().to_string();

    let Ok(mut db) = state.write() else {
        return (StatusCode::SERVICE_UNAVAILABLE).into_response();
    };

    db.insert_item(LIST_UUID, &item_uuid, item);

    (
        StatusCode::OK,
        Json(ShoppingListItem {
            title: post_request.title,
            posted_by: post_request.posted_by,
            uuid: item_uuid,
        }),
    )
        .into_response()
}

pub async fn delete_item(
    State(state): State<Database>,
    Path(uuid): Path<Uuid>,
) -> impl IntoResponse {
    let Ok(mut db) = state.write() else {
        return StatusCode::SERVICE_UNAVAILABLE;
    };

    db.delete_item(LIST_UUID, &uuid.to_string());

    StatusCode::OK
}
