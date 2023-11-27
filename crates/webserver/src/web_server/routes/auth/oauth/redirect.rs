use std::net::SocketAddr;

use crate::web_server::state::AppState;
use axum::{
    extract::{ConnectInfo, Path, State},
    response::{IntoResponse, Response},
    Extension, Json, http::StatusCode,
};
use oauth2::{CsrfToken, PkceCodeChallenge, Scope};
use redis::{AsyncCommands, SetExpiry, SetOptions};
use serde_json::json;

pub async fn handler(
    Path(provider): Path<String>,
    State(state): State<AppState>,
    Extension(ConnectInfo(connection_info)): Extension<ConnectInfo<SocketAddr>>,
) -> Response {
    let oauth_provider = match provider.as_str() {
        "discord" => state.get_oauth().discord(),
        "whmcs" => todo!(),
        _ => {
            return (
                StatusCode::NOT_FOUND,
                Json(json!({"message": "Invalid OAuth2 Provider"})).into_response(),
            )
                .into_response()
        }
    };

    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    let (auth_url, csrf_token) = oauth_provider
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("email".to_string()))
        .add_scope(Scope::new("identify".to_string()))
        .set_pkce_challenge(pkce_challenge)
        .url();

    let mut redis_client = match state.get_redis().get().await {
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
            format!("{}:{}:pkce_code", connection_info.ip(), provider),
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
            format!("{}:{}:csrf_code", connection_info.ip(), provider),
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
