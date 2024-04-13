use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ShoppingListItem {
    pub title: String,
    pub posted_by: String,
    pub uuid: String,
}
