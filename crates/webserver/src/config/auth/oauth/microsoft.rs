use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct MicrosoftOAuthProviderConfig {
    client_id: String,
    client_secret: String,
    tenant_id: String,
}

