use axum::{
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

pub async fn handler() -> Response {
    Json(json!({
        "services": [
            {
                "displayName": "Discord",
                "iden": "discord",
                "enabled": true,
                "icon": "faDiscord"
            }
        ]
    }))
    .into_response()
}
