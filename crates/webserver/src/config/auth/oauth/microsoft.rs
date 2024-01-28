use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename = "microsoft")]
pub struct MicrosoftOAuthProviderConfig {
    client_id: String,
    client_secret: String,
    tenant_id: String,
}

impl Default for MicrosoftOAuthProviderConfig {
    fn default() -> Self {
        Self {
            client_id: "Password1".to_string(),
            client_secret: "Password1".to_string(),
            tenant_id: "b32229b0-a29a-41ca-bf91-af2fe5182c5e".to_string(),
        }
    }
}
