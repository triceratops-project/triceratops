use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename = "okta")]
pub struct OktaOAuthProviderConfig {
    client_id: String,
    client_secret: String
}
