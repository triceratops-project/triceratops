use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename = "google")]
pub struct GoogleOAuthProviderConfig {
    client_id: String,
    client_secret: String,
}