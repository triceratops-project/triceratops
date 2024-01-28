use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct GoogleOAuthProviderConfig {
    client_id: String,
    client_secret: String,
}