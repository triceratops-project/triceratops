use crate::{state::AppState, web_server::error::ErrorResponse};
use axum::{
    extract::{ConnectInfo, State},
    Extension, Json,
};
use fred::interfaces::KeysInterface;
use oauth2::{reqwest::async_http_client, AuthorizationCode, PkceCodeVerifier, TokenResponse};
use serde::Deserialize;
use serde_json::{json, Value};
use std::net::SocketAddr;

#[derive(Deserialize, Debug)]
pub struct RequestQuery {
    code: String,
    state: String,
}

pub async fn handler(
    State(state): State<AppState>,
    Extension(ConnectInfo(connection_info)): Extension<ConnectInfo<SocketAddr>>,
    Json(body): Json<RequestQuery>,
) -> Result<String, ErrorResponse<Json<Value>>> {
    let oauth_provider = state
        .oauth()
        .microsoft()
        .as_ref()
        .ok_or(ErrorResponse::Forbidden(Json(
            json!({"message": "Microsoft is not an enabled provider"}),
        )))?;

    let redis_client = state.cache();

    let csrf_token: Option<String> = redis_client
        .get(format!("{}:microsoft:csrf_code", connection_info.ip()))
        .await
        .map_err(|_| {
            ErrorResponse::InternalServerError(Json(json!({"message": "Internal Server Error"})))
        })?;

    let csrf_token = csrf_token.ok_or(ErrorResponse::BadRequest(Json(
        json!({"message": "Authentication Timeout"}),
    )))?;

    if body.state != csrf_token {
        return Err(ErrorResponse::BadRequest(Json(
            json!({"message": "Invalid CSRF Token"}),
        )));
    }

    let pkce_verifier: Option<String> = redis_client
        .get(format!("{}:microsoft:pkce_code", connection_info.ip()))
        .await
        .map_err(|_| {
            ErrorResponse::InternalServerError(Json(json!({"message": "Internal Server Error"})))
        })?;

    let pkce_verifier = pkce_verifier.ok_or(ErrorResponse::BadRequest(Json(
        json!({"message": "Authentication Timeout"}),
    )))?;

    // Fix MS's retardation
    // https://github.com/ramosbugs/oauth2-rs/issues/191
    let token_response = oauth_provider
        .exchange_code(AuthorizationCode::new(body.code))
        .set_pkce_verifier(PkceCodeVerifier::new(pkce_verifier))
        .request_async(async_http_client)
        .await
        .map_err(|_| {
            ErrorResponse::InternalServerError(Json(json!({"message": "Internal Server Error (Microsoft doesn't follow spec, will fix later)"})))
        })?;

    Ok(token_response.access_token().secret().clone())
}
