use axum::{middleware, routing::post, Router};

use crate::web_server::{middleware::Auth, state::AppState};

mod login;
mod logout;
mod register;

pub fn router(state: AppState) -> Router<AppState> {
    let router = Router::new()
        .route("/login", post(login::handler))
        .route(
            "/logout",
            post(logout::handler).layer(middleware::from_fn_with_state(state, Auth)),
        )
        .route("/register", post(register::handler));

    return router;
}
