use crate::web_server::state::AppState;
use axum::{middleware, routing::get, Router};

pub mod server;
pub mod servers;

pub fn router(state: AppState) -> Router<AppState> {
    let router = Router::new().route("/", get(servers::handler)).route(
        "/:server_id",
        get(server::handler).layer(middleware::from_fn_with_state(
            state,
            crate::web_server::middleware::Auth,
        )),
    );

    return router;
}
