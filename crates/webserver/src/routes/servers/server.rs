use axum::{extract::Path, Json};
use serde_json::{json, Value};

pub async fn handler(Path(server_id): Path<String>) -> Json<Value> {
    Json(json!({
        "id:": server_id,
    }))
}
