use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct DiscordOAuthProviderConfig {
    client_id: String,
    client_secret: String,
}

impl DiscordOAuthProviderConfig {
    pub fn client_id(&self) -> &String {
        &self.client_id
    }

    pub fn client_secret(&self) -> &String {
        &self.client_secret
    }
}
