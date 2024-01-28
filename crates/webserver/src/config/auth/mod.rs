use oauth::OAuthProvidersConfig;
use saml::SamlConfig;
use serde::{Deserialize, Serialize};

mod oauth;
mod saml;

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthConfig {
    oauth: Option<OAuthProvidersConfig>,
    saml: Option<SamlConfig>,
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            oauth: Some(OAuthProvidersConfig::default()),
            saml: Some(SamlConfig::default()),
        }
    }
}
