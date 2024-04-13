mod controllers;
mod database;

use std::sync::{Arc, RwLock};

use axum::{routing::get, Router};
use controllers::get_items;
use database::InMemoryDatabase;

type Database = Arc<RwLock<InMemoryDatabase>>;

#[tokio::main]
async fn main() {
    let db = Database::default();
    let app = Router::new().route("/items", get(get_items)).with_state(db);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
