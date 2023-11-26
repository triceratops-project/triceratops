use axum::{
    extract::State,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use zeroize::ZeroizeOnDrop;

use crate::web_server::state::AppState;

#[derive(Deserialize, Serialize, ZeroizeOnDrop)]
pub struct RequestBody {
    username: String,
    password: String,
}

pub async fn handler(State(state): State<AppState>, Json(body): Json<RequestBody>) -> Response {
    Json(body).into_response()

    // "penis".into_response()
}