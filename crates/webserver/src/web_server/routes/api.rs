use super::{auth, servers, users};
use crate::{state::AppState, web_server::middleware::RateLimit};
use axum::{
    http::StatusCode,
    middleware,
    response::{IntoResponse, Response},
    Json, Router,
};
use serde_json::json;

pub fn route(app_state: &AppState) -> Router<AppState> {
    Router::new()
        .nest(
            "/auth",
            auth::router(app_state)
                .layer(middleware::from_fn_with_state(app_state.clone(), RateLimit)),
        )
        .nest("/servers", servers::router(app_state))
        .nest("/users", users::router(app_state))
        .fallback(handle_404)
}

async fn handle_404() -> Response {
    (StatusCode::NOT_FOUND, Json(json!({"message": "Not found"}))).into_response()
}
