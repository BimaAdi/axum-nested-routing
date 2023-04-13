pub mod routes;

use axum::{response::Json, routing::get, Router};
use serde_json::json;

pub fn initiate_route() -> Router {
    // build our application with a single route
    let hello_route = Router::new().route(
        "/",
        get(|| async { Json(json!({ "message": "hello world" })) }),
    );

    Router::new()
        .nest("/auth", routes::auth::initiate_auth_route())
        .nest("/user", routes::user::initiate_user_route())
        .nest("/", hello_route)
}
