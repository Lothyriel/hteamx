mod api;
mod routes;

use axum::{routing, Router};
use tower_http::services::{ServeDir, ServeFile};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or("debug,hyper=off".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::new()
        .route("/", routing::get(routes::index))
        .nest("/api", api::api_routes())
        .nest_service("/assets", ServeDir::new("assets"))
        .nest_service("/favicon.ico", ServeFile::new("assets/favicon.ico"))
        .layer(tower_http::trace::TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .expect("Expected to bind to 0.0.0.0:8080");

    tracing::info!("listening on {:?}", listener.local_addr().unwrap());

    axum::serve(listener, app)
        .await
        .expect("Expected to start axum");
}
