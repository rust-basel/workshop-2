use axum::{extract::Path, response::IntoResponse, routing::get, Router};

async fn hello_world() -> impl IntoResponse {
    "Hello World"
}

async fn hello_name(Path(name): Path<String>) -> impl IntoResponse {
    format!("Hello {name}")
}


#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(hello_world))
        .route("/:name", get(hello_name));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
