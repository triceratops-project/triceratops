use super::{auth, servers, users};
use crate::state::AppState;
use axum::Router;

pub fn route(app_state: &AppState) -> Router<AppState> {
    Router::new()
        .nest("/auth", auth::router(app_state))
        .nest("/servers", servers::router(app_state))
        .nest("/users", users::router(app_state))
}
