use axum::{
    middleware,
    routing::{delete, get, patch, post},
    Router,
};

use crate::controllers::users_controller::{self};
use crate::middlewares::validate_trusly_content::validate_trusly_content;

pub fn routes() -> Router {
    Router::new()
        .route("/users", post(users_controller::create_user))
        .route_layer(middleware::from_fn(validate_trusly_content))
        .route("/users", get(users_controller::get_users))
        .route("/users/:id", get(users_controller::get_user))
        .route("/users/:id", patch(users_controller::update_user))
        .route("/users/:id", delete(users_controller::delete_user))
}
