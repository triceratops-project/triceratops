use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename = "whmcs")]
#[serde(rename_all = "kebab-case")]
pub struct WhmcsOAuthProviderConfig {
    client_id: String,
    client_secret: String
}
