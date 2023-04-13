use axum::response;
use axum::routing::post;
use axum::Router;
use serde_json::{json, Value};

async fn login() -> response::Json<Value> {
    response::Json(json!({"message":"login"}))
}

async fn logout() -> response::Json<Value> {
    response::Json(json!({"message":"logout"}))
}

pub fn initiate_auth_route() -> Router {
    Router::new()
        .route("/login/", post(login))
        .route("/logout/", post(logout))
}
