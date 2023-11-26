use axum::{
    routing::{get, post},
    Router,
};

use crate::web_server::state::AppState;

mod user;
mod users;

pub fn router(_state: AppState) -> Router<AppState> {
    let router = Router::new()
        .route("/", get(users::handler))
        .route("/:user_id", get(user::handler));

    return router;
}
