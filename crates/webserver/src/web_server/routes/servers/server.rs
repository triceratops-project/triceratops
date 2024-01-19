use crate::web_server::state::AppState;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use sea_orm::EntityTrait;
use serde_json::json;
use triceratops_server_entity::servers as Servers;

pub async fn handler(Path(server_id): Path<String>, State(state): State<AppState>) -> Response {
    let server = Servers::Entity::find_by_id(&server_id)
        .one(state.get_pool())
        .await;

    let server = match server {
        Ok(server) => server,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Internal Server Error"})),
            )
                .into_response();
        }
    };

    match server {
        Some(server) => Json(server).into_response(),
        None => {
            (StatusCode::NOT_FOUND, Json(json!({"message": "Not Found"}))).into_response()
        }
    }
}
