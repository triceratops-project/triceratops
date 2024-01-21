use std::net::SocketAddr;

use crate::state::AppState;
use axum::{
    extract::{ConnectInfo, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Extension, Json,
};
use oauth2::{reqwest::async_http_client, AuthorizationCode, PkceCodeVerifier, TokenResponse};
use redis::AsyncCommands;
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize, Debug)]
pub struct RequestQuery {
    code: String,
    state: String,
}

pub async fn handler(
    State(state): State<AppState>,
    Extension(ConnectInfo(connection_info)): Extension<ConnectInfo<SocketAddr>>,
    Json(body): Json<RequestQuery>,
) -> Response {
    let oauth_provider = state.get_oauth().discord();

    let mut redis_client = match state.get_cache().get().await {
        Ok(client) => client,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Internal Server Error"})).into_response(),
            )
                .into_response()
        }
    };

    let csrf_token: String = match redis_client
        .get(format!("{}:discord:csrf_code", connection_info.ip()))
        .await
    {
        Ok(Some(code)) => code,
        _ => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({"message": "Invalid or expired state"})).into_response(),
            )
                .into_response()
        }
    };

    if body.state != csrf_token {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"message": "Invalid CSRF Token"})).into_response(),
        )
            .into_response();
    }

    let pkce_verifier: String = match redis_client
        .get(format!("{}:discord:pkce_code", connection_info.ip()))
        .await
    {
        Ok(Some(code)) => code,
        _ => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({"message": "Authentication Timeout"})).into_response(),
            )
                .into_response()
        }
    };

    let token_response = match oauth_provider
        .exchange_code(AuthorizationCode::new(body.code))
        .set_pkce_verifier(PkceCodeVerifier::new(pkce_verifier))
        .request_async(async_http_client)
        .await
    {
        Ok(response) => response,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Internal Server Error"})).into_response(),
            )
                .into_response()
        }
    };

    token_response
        .access_token()
        .secret()
        .clone()
        .into_response()
}
