use axum::Server;

#[tokio::main]
async fn main() {
    // run it with hyper on localhost:3000
    let app = axum_group_route::initiate_route();
    Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
