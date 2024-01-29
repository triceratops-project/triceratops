use serde::{Deserialize, Serialize};

pub use oauth::OAuthProvidersConfig;
pub use saml::SamlConfig;

pub mod oauth;
mod saml;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AuthConfig {
    #[serde(rename = "jwt-secret")]
    jwt_secret: String,
    oauth: Option<OAuthProvidersConfig>,
    saml: Option<SamlConfig>,
}

impl AuthConfig {
    pub fn oauth(&self) -> &Option<OAuthProvidersConfig> {
        &self.oauth
    }

    pub fn saml(&self) -> &Option<SamlConfig> {
        &self.saml
    }
}
