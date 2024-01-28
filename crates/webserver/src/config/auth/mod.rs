use oauth::OAuthProvidersConfig;
use saml::SamlConfig;
use serde::{Deserialize, Serialize};

mod oauth;
mod saml;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AuthConfig {
    oauth: Option<OAuthProvidersConfig>,
    saml: Option<SamlConfig>,
}

