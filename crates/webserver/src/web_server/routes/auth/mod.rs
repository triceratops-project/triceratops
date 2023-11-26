use axum::{Router, routing::post};

use crate::web_server::state::AppState;

mod login;
mod logout;

pub fn router(_state: AppState) -> Router<AppState> {
    let router = Router::new()
        .route("/login", post(login::handler))
        .route("/logout", post(logout::handler));

    return router;
}
