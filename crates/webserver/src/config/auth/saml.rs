use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename = "saml")]
pub struct SamlConfig {
    sso_url: Url,
    issuer: String,
    signing_certificate: String,
}

impl Default for SamlConfig {
    fn default() -> Self {
        Self {
            sso_url: Url::parse("https://example.com").unwrap(),
            issuer: "ACME Corp".to_string(),
            signing_certificate: "super secure cert".to_string(),
        }
    }
}
