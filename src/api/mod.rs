use axum::{routing, Router};
use tower_cookies::{Cookie, Cookies};

use crate::routes::COOKIE_NAME;

pub fn api_routes() -> Router {
    let confirmation = Router::new().route("/", routing::post(set_confirmation));

    Router::new().nest("/confirmation", confirmation)
}

async fn set_confirmation(cookies: Cookies, name: String) -> String {
    cookies.add(Cookie::new(COOKIE_NAME, name));

    "João Xavier".to_string()
}
