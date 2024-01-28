use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename = "okta")]
#[serde(rename_all = "kebab-case")]
pub struct OktaOAuthProviderConfig {
    client_id: String,
    client_secret: String
}
