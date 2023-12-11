use super::{middleware::RateLimit, state::InternalAppState};
use axum::{middleware, Router};
use std::sync::Arc;

mod auth;
mod servers;
mod spa;
mod users;

pub async fn route() -> Router {
    let app_state = Arc::new(InternalAppState::new().await);

    let router = Router::new()
        .nest(
            "/api/auth",
            auth::router(app_state.clone())
                .layer(middleware::from_fn_with_state(app_state.clone(), RateLimit)),
        )
        .nest("/api/servers", servers::router(app_state.clone()))
        .nest("/api/users", users::router(app_state.clone()))
        .nest("/", spa::router())
        .with_state(app_state);

    return router;
}
