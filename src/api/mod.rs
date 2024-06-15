use axum::Router;

pub fn api_routes() -> Router {
    Router::new().route("/hello-world", axum::routing::post(hello_world))
}

async fn hello_world() -> String {
    "João Xavier".to_string()
}
