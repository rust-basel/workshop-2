use model::{CreateListResponse, PostShopItem, ShoppingListItem};

pub async fn get_items(list_id: &str) -> Result<Vec<ShoppingListItem>, reqwest::Error> {
    let url = format!("http://localhost:3001/list/{}/items", list_id);
    let list = reqwest::get(&url)
        .await?
        .json::<Vec<ShoppingListItem>>()
        .await;

    list
}

pub async fn post_item(
    list_id: &str,
    item: PostShopItem,
) -> Result<ShoppingListItem, reqwest::Error> {
    let response = reqwest::Client::new()
        .post(format!("http://localhost:3001/list/{}/items", list_id))
        .json(&item)
        .send()
        .await?
        .json::<ShoppingListItem>()
        .await?;

    Ok(response)
}

pub async fn delete_item(list_id: &str, item_id: &str) -> Result<(), reqwest::Error> {
    reqwest::Client::new()
        .delete(format!(
            "http://localhost:3001/list/{}/items/{}",
            list_id, item_id
        ))
        .send()
        .await?;

    Ok(())
}

pub async fn create_list() -> Result<CreateListResponse, reqwest::Error> {
    let response = reqwest::Client::new()
        .get("http://localhost:3001/list")
        .send()
        .await?
        .json::<CreateListResponse>()
        .await?;

    Ok(response)
}
