use axum::{routing::delete, routing::get, routing::patch, routing::post, Router};

use crate::controllers::users_controller::{self};

pub fn routes() -> Router {
    Router::new()
        .route("/users", post(users_controller::create_user))
        .route("/users", get(users_controller::get_users))
        .route("/users/:id", get(users_controller::get_user))
        .route("/users/:id", patch(users_controller::update_user))
        .route("/users/:id", delete(users_controller::delete_user))
}
