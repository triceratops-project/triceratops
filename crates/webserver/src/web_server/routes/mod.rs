use crate::state::InternalAppState;
use axum::Router;
use error_stack::{Context, Result, ResultExt};
use std::{fmt::Display, sync::Arc};

mod api;
mod auth;
mod servers;
mod spa;
mod users;

#[derive(Debug)]
pub struct RouterError;

impl Display for RouterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Router error")
    }
}

impl Context for RouterError {}

pub async fn route() -> Result<Router, RouterError> {
    let app_state_raw = InternalAppState::new()
        .await
        .attach_printable("Failed to assemble app state")
        .change_context(RouterError)?;
    let app_state = Arc::new(app_state_raw);

    let api_router = Router::new().nest("/api", api::route(&app_state));

    let spa_router = spa::router();

    Ok(Router::new()
        .merge(spa_router)
        .merge(api_router)
        .with_state(app_state))
}
