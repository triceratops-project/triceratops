use crate::web_server::state::AppState;
use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

use chrono::Utc;
use rand_core::OsRng;
use sea_orm::{ColumnTrait, Condition, EntityTrait, QueryFilter, Set, TryIntoModel};
use serde::{Deserialize, Serialize};
use serde_json::json;
use triceratops_server_entity::users as Users;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RequestBody {
    username: String,
    email: String,
    password: String,
    first_name: Option<String>,
    last_name: Option<String>,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ResponseBody {
    user: Users::Model,
    // session:
}

pub async fn handler(State(state): State<AppState>, Json(body): Json<RequestBody>) -> Response {
    let user_exists = Users::Entity::find()
        .filter(
            Condition::any()
                .add(Users::Column::Email.eq(&body.email))
                .add(Users::Column::Username.eq(&body.username)),
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
        username: Set(body.username),
        email: Set(body.email),
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

    match user {
        Ok(_) => {
            return (
                StatusCode::CREATED,
                Json(ResponseBody {
                    user: new_user_as_model,
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
