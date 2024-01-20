use super::state::InternalAppState;
use axum::Router;
use std::sync::Arc;

mod api;
mod auth;
mod servers;
mod spa;
mod users;

pub async fn route() -> Router {
    let app_state = Arc::new(InternalAppState::new().await);

    let api_router = Router::new().nest("/api", api::route(&app_state));

    let spa_router = spa::router();

    Router::new()
        .merge(spa_router)
        .merge(api_router)
        .with_state(app_state)
}
