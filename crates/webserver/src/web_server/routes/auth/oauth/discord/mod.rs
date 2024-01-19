use crate::web_server::state::AppState;
use axum::{
    routing::{get, post},
    Router,
};

mod callback;
mod redirect;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(redirect::handler))
        .route("/callback", post(callback::handler))
}
