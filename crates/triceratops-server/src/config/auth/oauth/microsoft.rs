use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct MicrosoftOAuthProviderConfig {
    client_id: String,
    client_secret: String,
    tenant_id: String,
}

impl MicrosoftOAuthProviderConfig {
    pub fn client_id(&self) -> &String {
        &self.client_id
    }

    pub fn client_secret(&self) -> &String {
        &self.client_secret
    }

    pub fn tenant_id(&self) -> &String {
        &self.tenant_id
    }
}