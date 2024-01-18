use crate::web_server::state::AppState;
use axum::Router;

mod discord;

pub fn router() -> Router<AppState> {
    Router::new().nest("/discord", discord::router())
}
