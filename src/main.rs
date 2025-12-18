use axum::{
    routing::{delete, get, post, put},
    Router
};

#[tokio::main]
async fn main() {
    let user_routes = Router::new()
        .route("/", get(|| async move { "user" }))
        .route("/login", get(|| async move { "login" }));
    let team_routes = Router::new()
        .route("/", post(|| async move {"teams" }));

    let api_routes = Router::new()
        .nest("/users", user_routes)
        .nest("/teams", team_routes);

    let app = Router::new()
        .nest("/api", api_routes);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
