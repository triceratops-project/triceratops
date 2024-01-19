use argon2::{Argon2, PasswordHash, PasswordVerifier};
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use chrono::{DateTime, Utc};
use regex::Regex;
use sea_orm::{ColumnTrait, Condition, EntityTrait, QueryFilter, Set, TryIntoModel};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sha2::{Digest, Sha512};
use std::net::Ipv4Addr;
use triceratops_server_entity::sessions as Sessions;
use triceratops_server_entity::users as Users;

use crate::web_server::state::AppState;

#[derive(Deserialize)]
pub struct RequestBody {
    username: String,
    password: String,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ResponseBody {
    user: Users::Model,
    session: ResponseSessionToken,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ResponseSessionToken {
    id: String,
    token: String,
    expires_at: Option<DateTime<Utc>>,
    created_at: DateTime<Utc>,
}

pub async fn handler(State(state): State<AppState>, Json(body): Json<RequestBody>) -> Response {
    let username_regex = Regex::new("^[A-Za-z0-9_.]+$").unwrap();
    let email_regex = Regex::new(r"^[\w\-\.]+@([\w-]+\.)+[\w-]{2,}$").unwrap();

    let username_pass_regex = username_regex.is_match(body.username.trim());
    let email_pass_regex = email_regex.is_match(body.username.trim());

    if !(username_pass_regex || email_pass_regex) {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"message": "Username must only contain letters A-z, 0-9, _, & ."})),
        )
            .into_response();
    }

    let users = Users::Entity::find()
        .filter(
            Condition::any()
                .add(Users::Column::Email.eq(body.username.trim().to_lowercase()))
                .add(Users::Column::Username.eq(body.username.trim().to_lowercase())),
        )
        .one(state.get_pool())
        .await;

    let user = match users {
        Ok(users) => users,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Internal Server Error"})),
            )
                .into_response();
        }
    };

    let user = match user {
        Some(user) => user,
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({"message": "Incorrect username or password"})),
            )
                .into_response()
        }
    };

    let password = match user.password.clone() {
        Some(password) => password,
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({"message": "Password authentication is not enabled for this account, sign in with SSO instead."})),
            )
                .into_response()
        }
    };

    let password_hash = match PasswordHash::new(password.as_str()) {
        Ok(password_hash) => password_hash,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Internal Server Error"})),
            )
                .into_response()
        }
    };

    match Argon2::default().verify_password(body.password.as_bytes(), &password_hash) {
        Ok(_) => {}
        Err(_) => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({"message": "Incorrect username or password"})),
            )
                .into_response()
        }
    }

    let user_as_model = match user.try_into_model() {
        Ok(model) => model,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Internal Server Error"})),
            )
                .into_response()
        }
    };

    let mut active_user = Users::ActiveModel::from(user_as_model.clone());

    active_user.last_login_at = Set(Some(Utc::now()));

    let user = Users::Entity::update(active_user)
        .filter(Users::Column::Id.eq(user_as_model.id.clone()))
        .exec(state.get_pool())
        .await;

    match user {
        Ok(_) => {}
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Internal Server Error"})),
            )
                .into_response();
        }
    }

    let utc_time = Utc::now();

    let mut hasher = Sha512::new();

    let session_token = cuid2::create_id();

    hasher.update(session_token.as_bytes());

    let session_token_hash = hasher.finalize();

    let hex_encoding = base16ct::lower::encode_string(&session_token_hash);

    let session = Sessions::ActiveModel {
        id: Set(cuid2::create_id()),
        token: Set(hex_encoding),
        user_id: Set(user_as_model.id.clone()),
        ip_address: Set(Ipv4Addr::new(0, 0, 0, 0).to_string()),
        expires_at: Set(None),
        created_at: Set(utc_time),
    };

    let db_session = Sessions::Entity::insert(session.clone())
        .exec(state.get_pool())
        .await;

    match db_session {
        Ok(_) => {}
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Internal Server Error"})),
            )
                .into_response();
        }
    };

    let db_session_as_model = match session.try_into_model() {
        Ok(model) => model,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Internal Server Error"})),
            )
                .into_response();
        }
    };

    let response_session = ResponseSessionToken {
        id: db_session_as_model.id,
        token: session_token,
        expires_at: db_session_as_model.expires_at,
        created_at: db_session_as_model.created_at,
    };

    (
        StatusCode::OK,
        Json(ResponseBody {
            user: user_as_model,
            session: response_session,
        }),
    )
        .into_response()
}
