use axum::{routing::get, Router};

mod config;
mod controllers;
mod models;
mod routes;

use config::db;
use routes::users_router;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    db::init_db().await;

    let app = Router::new()
        .route("/", get(root))
        .merge(users_router::routes());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}
