use std::net::SocketAddr;

use crate::state::AppState;
use axum::{
    extract::{ConnectInfo, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Extension, Json,
};
use oauth2::{CsrfToken, PkceCodeChallenge, Scope};
use redis::{AsyncCommands, SetExpiry, SetOptions};
use serde_json::json;

pub async fn handler(
    State(state): State<AppState>,
    Extension(ConnectInfo(connection_info)): Extension<ConnectInfo<SocketAddr>>,
) -> Response {
    let oauth_provider = match state.oauth().discord() {
        Some(provider) => provider,
        None => return (StatusCode::FORBIDDEN, Json(json!({"message": "Discord is not an enabled provider"}))).into_response()
    };

    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    let (auth_url, csrf_token) = oauth_provider
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("email".to_string()))
        .add_scope(Scope::new("identify".to_string()))
        .set_pkce_challenge(pkce_challenge)
        .url();

    let mut redis_client = match state.cache().get().await {
        Ok(client) => client,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Internal Server Error"})).into_response(),
            )
                .into_response()
        }
    };

    let redis_expiry = SetOptions::default().with_expiration(SetExpiry::EX(60));

    if let Err(_) = redis_client
        .set_options::<String, &String, ()>(
            format!("{}:discord:pkce_code", connection_info.ip()),
            pkce_verifier.secret(),
            redis_expiry,
        )
        .await
    {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"message": "Internal Server Error"})).into_response(),
        )
            .into_response();
    }

    let redis_expiry = SetOptions::default().with_expiration(SetExpiry::EX(60));

    if let Err(_) = redis_client
        .set_options::<String, &String, ()>(
            format!("{}:discord:csrf_code", connection_info.ip()),
            csrf_token.secret(),
            redis_expiry,
        )
        .await
    {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"message": "Internal Server Error"})).into_response(),
        )
            .into_response();
    }

    Json(json!({
        "url": auth_url.as_str()
    }))
    .into_response()
}
