use axum::{routing::post, Router};

use crate::web_server::state::AppState;

mod login;
mod logout;
mod register;

pub fn router(_state: AppState) -> Router<AppState> {
    let router = Router::new()
        .route("/login", post(login::handler))
        .route("/logout", post(logout::handler))
        .route("/register", post(register::handler));

    return router;
}
