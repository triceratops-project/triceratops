use super::state::InternalAppState;
use axum::Router;
use std::sync::Arc;

mod auth;
mod servers;
mod users;

pub async fn route() -> Router {
    let app_state = Arc::new(InternalAppState::new().await);

    let router = Router::new()
        .nest("/auth", auth::router(app_state.clone()))
        .nest("/servers", servers::router(app_state.clone()))
        .nest("/users", users::router(app_state.clone()))
        .with_state(app_state);

    return router;
}
