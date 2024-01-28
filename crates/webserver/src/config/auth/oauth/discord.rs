use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct DiscordOAuthProviderConfig {
    client_id: String,
    client_secret: String,
}

impl Default for DiscordOAuthProviderConfig {
    fn default() -> Self {
        Self {
            client_id: "Password1".to_string(),
            client_secret: "Password1".to_string(),
        }
    }
}
