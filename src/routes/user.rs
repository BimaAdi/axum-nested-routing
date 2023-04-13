use axum::extract;
use axum::response;
use axum::routing::{delete, get, post, put};
use axum::Router;
use serde_json::{json, Value};
use std::collections::HashMap;

async fn get_all_user(
    extract::Query(params): extract::Query<HashMap<String, String>>,
) -> response::Json<Value> {
    let default_page = "1".to_string();
    let default_page_size = "10".to_string();
    let page = params.get("page").unwrap_or(&default_page);
    let page_size = params.get("page_size").unwrap_or(&default_page_size);
    response::Json(json!({
        "query": {
            "page": page,
            "page_size": page_size
        },
        "message":"get all user"
    }))
}

async fn get_detail_user(extract::Path(user_id): extract::Path<String>) -> response::Json<Value> {
    response::Json(json!({"message":"get detail user", "params": user_id}))
}

async fn post_create_user(extract::Json(payload): extract::Json<Value>) -> response::Json<Value> {
    response::Json(json!({"message":"post create user", "json": payload}))
}

async fn put_update_user(
    extract::Path(user_id): extract::Path<String>,
    extract::Json(payload): extract::Json<Value>,
) -> response::Json<Value> {
    response::Json(json!({"message":"put update user", "params": user_id, "json": payload}))
}

async fn delete_delete_user(
    extract::Path(user_id): extract::Path<String>,
) -> response::Json<Value> {
    response::Json(json!({"message":"delete delete user", "params": user_id}))
}

pub fn initiate_user_route() -> Router {
    Router::new()
        .route("/", get(get_all_user))
        .route("/", post(post_create_user))
        .route("/:id", get(get_detail_user))
        .route("/:id", put(put_update_user))
        .route("/:id", delete(delete_delete_user))
}
