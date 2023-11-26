use std::net::Ipv4Addr;

use crate::web_server::state::AppState;
use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

use chrono::{DateTime, Utc};
use rand_core::OsRng;
use regex::Regex;
use sea_orm::{ColumnTrait, Condition, EntityTrait, QueryFilter, Set, TryIntoModel};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sha2::{Digest, Sha512};
use triceratops_server_entity::sessions as Sessions;
use triceratops_server_entity::users as Users;
use validator::Validate;

#[derive(Deserialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct RequestBody {
    #[validate(length(min = 3, max = 128))]
    username: String,
    #[validate(email)]
    email: String,
    #[validate(length(min = 8, max = 128))]
    password: String,
    first_name: Option<String>,
    last_name: Option<String>,
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
    let username_pass_regex = username_regex.is_match(body.username.trim());

    if !username_pass_regex {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"message": "Username must only contain letters A-z, 0-9, _, & ."})),
        )
            .into_response();
    }

    match body.validate() {
        Ok(_) => {}
        Err(e) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({"message": e.to_string()})),
            )
                .into_response();
        }
    }

    let user_exists = Users::Entity::find()
        .filter(
            Condition::any()
                .add(Users::Column::Email.eq(body.email.trim()))
                .add(Users::Column::Username.eq(body.username.trim())),
        )
        .all(state.get_pool())
        .await;

    match user_exists {
        Ok(user) => {
            if user.len() > 0 {
                return (
                    StatusCode::CONFLICT,
                    Json(json!({"message": "Username or email are already in use"})),
                )
                    .into_response();
            }
        }
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Internal Server Error"})),
            )
                .into_response();
        }
    }

    let password_salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let password_hash = match argon2.hash_password(body.password.as_bytes(), &password_salt) {
        Ok(hash) => hash,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Internal Server Error"})),
            )
                .into_response();
        }
    };

    let utc_time = Utc::now();

    let new_user = Users::ActiveModel {
        id: Set(cuid2::create_id()),
        external_id: Set(None),
        username: Set(body.username.trim().to_owned()),
        email: Set(body.email.trim().to_owned()),
        password: Set(Some(password_hash.to_string())),
        first_name: Set(body.first_name),
        last_name: Set(body.last_name),
        last_login_at: Set(None),
        created_at: Set(utc_time),
    };

    let user = Users::Entity::insert(new_user.clone())
        .exec(state.get_pool())
        .await;

    let new_user_as_model = match new_user.try_into_model() {
        Ok(model) => model,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Internal Server Error"})),
            )
                .into_response();
        }
    };

    let mut hasher = Sha512::new();

    let session_token = cuid2::create_id();

    hasher.update(session_token.as_bytes());

    let session_token_hash = hasher.finalize();

    let hex_encoding = hex::encode(session_token_hash);

    let session = Sessions::ActiveModel {
        id: Set(cuid2::create_id()),
        token: Set(hex_encoding),
        user_id: Set(new_user_as_model.id.clone()),
        ip_address: Set(Ipv4Addr::new(127, 0, 0, 1).to_string()),
        expires_at: Set(None),
        created_at: Set(utc_time),
    };

    let db_session = Sessions::Entity::insert(session.clone())
        .exec(state.get_pool())
        .await;

    match db_session {
        Ok(_) => {}
        Err(e) => {
            println!("{:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Internal Server Error"})),
            )
                .into_response();
        }
    };

    let db_session_as_model = match session.try_into_model() {
        Ok(model) => model,
        Err(e) => {
            println!("{:?}", e);
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

    match user {
        Ok(_) => {
            return (
                StatusCode::CREATED,
                Json(ResponseBody {
                    user: new_user_as_model,
                    session: response_session,
                }),
            )
                .into_response()
        }
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Internal Server Error"})),
            )
                .into_response();
        }
    }
}
