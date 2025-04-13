use axum::{extract::Path, http::StatusCode, Json};
use bson::{doc, oid::ObjectId};
use futures::stream::StreamExt;

use crate::config::db;
use crate::models::user::{CreateUser, UpdateUser, User};

pub async fn create_user(Json(payload): Json<CreateUser>) -> (StatusCode, Json<User>) {
    let db = db::get_db(); // ⬅️ obtenemos la DB directamente

    let collection = db.collection("users");

    let user = User {
        id_mongo: None,
        username: payload.username.clone(),
    };

    let doc = doc! {
        "username": &user.username
    };

    let _ = collection.insert_one(doc, None).await.unwrap();

    (StatusCode::CREATED, Json(user))
}

pub async fn get_users() -> (StatusCode, Json<Vec<User>>) {
    let db = db::get_db();
    let collection = db.collection::<User>("users");

    let mut cursor = match collection.find(doc! {}, None).await {
        Ok(cursor) => cursor,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(vec![])),
    };

    let mut users_vec = Vec::new();

    while let Some(user) = cursor.next().await {
        match user {
            Ok(user) => users_vec.push(user),
            Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(vec![])),
        }
    }

    (StatusCode::OK, Json(users_vec))
}

pub async fn get_user(Path(id): Path<String>) -> (StatusCode, Json<Option<User>>) {
    let db = db::get_db();
    let collection = db.collection::<User>("users");

    let obj_id = match ObjectId::parse_str(&id) {
        Ok(oid) => oid,
        Err(_) => return (StatusCode::BAD_REQUEST, Json(None)),
    };

    let filter = doc! { "_id": obj_id };

    match collection.find_one(filter, None).await {
        Ok(Some(user)) => (StatusCode::OK, Json(Some(user))),
        Ok(None) => (StatusCode::NOT_FOUND, Json(None)),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(None)),
    }
}

pub async fn update_user(
    Path(id): Path<String>,
    Json(payload): Json<UpdateUser>,
) -> (StatusCode, Json<Option<String>>) {
    let db = db::get_db();
    let collection = db.collection::<User>("users");

    let obj_id = match ObjectId::parse_str(&id) {
        Ok(oid) => oid,
        Err(_) => return (StatusCode::BAD_REQUEST, Json(None)),
    };

    let filter = doc! { "_id": obj_id };

    let update = doc! {
        "$set": {
            "username": payload.username
        }
    };

    match collection.update_one(filter, update, None).await {
        Ok(updated) => match updated.modified_count {
            1 => (
                StatusCode::OK,
                Json(Some(format!("User with id {} updated", id))),
            ),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(Some(format!("Internal server error"))),
            ),
        },
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(Some(format!("Internal server error"))),
        ),
    }
}

pub async fn delete_user(Path(id): Path<String>) -> (StatusCode, Json<Option<String>>) {
    let db = db::get_db();
    let collection = db.collection::<User>("users");

    let obj_id = match ObjectId::parse_str(&id) {
        Ok(oid) => oid,
        Err(_) => return (StatusCode::BAD_REQUEST, Json(None)),
    };

    let filter = doc! { "_id": obj_id };

    match collection.delete_one(filter, None).await {
        Ok(deleted) => match deleted.deleted_count {
            1 => (
                StatusCode::OK,
                Json(Some(format!("User with id {} deleted", id))),
            ),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(Some(format!("Internal server error"))),
            ),
        },
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(Some(format!("Internal server error"))),
        ),
    }
}
