use axum::routing;
use axum::Router;
use confirm::ConfirmTemplate;
use tower_cookies::Cookies;

use tower_cookies::CookieManagerLayer;
use tower_http::services::{ServeDir, ServeFile};

use crate::api;

mod confirm;

pub fn app_router() -> Router {
    Router::new()
        .route("/", routing::get(index))
        .nest("/api", api::api_routes())
        .nest_service("/assets", ServeDir::new("assets"))
        .nest_service("/favicon.ico", ServeFile::new("assets/favicon.ico"))
        .layer(CookieManagerLayer::new())
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .fallback(handler)
}

pub const COOKIE_NAME: &str = "PARTICIPANT_NAME";

#[derive(askama::Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    confirm: ConfirmTemplate,
}

async fn index(cookies: Cookies) -> Result<IndexTemplate, String> {
    let conf_template = if let Some(c) = cookies.get(COOKIE_NAME) {
        get_confirmation_info(c.value()).await?
    } else {
        None
    };

    Ok(IndexTemplate {
        confirm: ConfirmTemplate {
            name: conf_template.as_ref().map(|c| c.name.to_owned()),
            escorts: conf_template.map(|c| c.escorts).unwrap_or_default(),
        },
    })
}

async fn get_confirmation_info(name: &str) -> Result<Option<ConfirmationInfo>, String> {
    todo!()
}

struct ConfirmationInfo {
    name: String,
    escorts: Vec<String>,
}

#[derive(thiserror::Error, Debug)]
pub enum IndexError {
    #[error("unknown data store error")]
    Unknown,
}
