use axum::{
    middleware,
    routing::{get, post},
    Router,
};

use crate::web_server::{middleware::Auth, state::AppState};

mod me;
mod user;
mod users;

pub fn router(state: AppState) -> Router<AppState> {
    let router = Router::new()
        .route("/", get(users::handler))
        .route(
            "/@me",
            get(me::handler).layer(middleware::from_fn_with_state(state, Auth)),
        )
        .route("/:user_id", get(user::handler));

    return router;
}
