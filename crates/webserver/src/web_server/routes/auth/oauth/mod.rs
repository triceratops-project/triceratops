use crate::state::AppState;
use axum::{routing::get, Router};
use serde::Serialize;
use url::Url;

mod discord;
mod microsoft;
mod services;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(services::handler))
        .nest("/discord", discord::router())
        .nest("/microsoft", microsoft::router())
}

#[derive(Debug, Serialize)]
pub struct OAuthRedirectResponse {
    url: Url,
}

impl OAuthRedirectResponse {
    pub fn new(url: Url) -> Self {
        Self { url }
    }
}
