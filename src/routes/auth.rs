use axum::{response::Json, routing::post, Router};
use serde_json::{json, Value};

async fn login() -> Json<Value> {
    Json(json!({"message":"login"}))
}

async fn logout() -> Json<Value> {
    Json(json!({"message":"logout"}))
}

pub fn initiate_auth_route() -> Router {
    Router::new()
        .route("/login/", post(login))
        .route("/logout/", post(logout))
}
