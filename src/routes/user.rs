use axum::routing::{delete, get, post, put};
use axum::{response::Json, Router};
use serde_json::{json, Value};

async fn get_all_user() -> Json<Value> {
    Json(json!({"message":"get all user"}))
}

async fn get_detail_user() -> Json<Value> {
    Json(json!({"message":"get detail user"}))
}

async fn post_create_user() -> Json<Value> {
    Json(json!({"message":"post create user"}))
}

async fn put_update_user() -> Json<Value> {
    Json(json!({"message":"put update user"}))
}

async fn delete_delete_user() -> Json<Value> {
    Json(json!({"message":"delete delete user"}))
}

pub fn initiate_user_route() -> Router {
    Router::new()
        .route("/", get(get_all_user))
        .route("/", post(post_create_user))
        .route("/:id", get(get_detail_user))
        .route("/:id", put(put_update_user))
        .route("/:id", delete(delete_delete_user))
}
