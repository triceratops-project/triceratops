use crate::state::{AppState, InternalAppState};
use axum::Router;
use std::sync::Arc;

mod servers;

pub async fn route() -> Router {
    let app_state = Arc::new(InternalAppState::new().await);

    let router = Router::new()
        .nest("/servers", servers::router())
        .with_state(app_state);

    return router;
}
