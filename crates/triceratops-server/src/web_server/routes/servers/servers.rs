use crate::state::AppState;
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use sea_orm::EntityTrait;
use serde_json::json;
use triceratops_entity::servers as Servers;

pub async fn handler(State(state): State<AppState>) -> Response {
    let servers = Servers::Entity::find().all(state.pool()).await;

    let servers = match servers {
        Ok(servers) => servers,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Internal Server Error"})),
            )
                .into_response();
        }
    };

    Json(servers).into_response()
}
