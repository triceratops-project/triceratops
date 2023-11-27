use super::{middleware::RateLimit, state::InternalAppState};
use axum::{middleware, Router};
use std::sync::Arc;

mod auth;
mod servers;
mod users;

pub async fn route() -> Router {
    let app_state = Arc::new(InternalAppState::new().await);

    let router = Router::new()
        .nest(
            "/auth",
            auth::router(app_state.clone())
                .layer(middleware::from_fn_with_state(app_state.clone(), RateLimit)),
        )
        .nest("/servers", servers::router(app_state.clone()))
        .nest("/users", users::router(app_state.clone()))
        .with_state(app_state);

    return router;
}
