use crate::state::AppState;
use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ResponseBody {
    pub services: Vec<OAuthService>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OAuthService {
    name: String,
    iden: String,
}

impl OAuthService {
    pub fn new<T: Into<String>>(name: T, iden: T) -> Self {
        let name = name.into();
        let iden = iden.into();

        Self { name, iden }
    }
}

pub async fn handler(State(state): State<AppState>) -> Json<ResponseBody> {
    let Some(oauth_config) = state.config().auth.oauth() else {
        return Json(ResponseBody { services: vec![] });
    };

    let mut services: Vec<OAuthService> = vec![];

    // match oauth_config.custom() {
    //     Some(_) => {
    //         services.push(OAuthService::new("OAuth", "custom"));
    //     }
    //     None => {}
    // };

    match oauth_config.discord() {
        Some(_) => {
            services.push(OAuthService::new("Discord", "discord"));
        }
        None => {}
    };

    // match oauth_config.google() {
    //     Some(_) => {
    //         services.push(OAuthService::new("Google", "google"));
    //     }
    //     None => {}
    // };

    match oauth_config.microsoft() {
        Some(_) => {
            services.push(OAuthService::new("Microsoft", "microsoft"));
        }
        None => {}
    };

    // match oauth_config.okta() {
    //     Some(_) => {
    //         services.push(OAuthService::new("Okta", "okta"));
    //     }
    //     None => {}
    // };

    // match oauth_config.whmcs() {
    //     Some(_) => {
    //         services.push(OAuthService::new("Whmcs", "whmcs"));
    //     }
    //     None => {}
    // };

    Json(ResponseBody { services })
}
