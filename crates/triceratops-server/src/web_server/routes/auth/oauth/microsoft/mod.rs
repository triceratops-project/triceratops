use axum::{
    routing::{get, post},
    Router,
};

use crate::state::AppState;

mod callback;
mod redirect;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(redirect::handler))
        .route("/callback", post(callback::handler))
}
