use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct WhmcsOAuthProviderConfig {
    client_id: String,
    client_secret: String
}
