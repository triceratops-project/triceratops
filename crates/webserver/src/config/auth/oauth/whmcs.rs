use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename = "whmcs")]
pub struct WhmcsOAuthProviderConfig {
    client_id: String,
    client_secret: String
}
