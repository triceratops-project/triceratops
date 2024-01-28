use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct CustomOAuthProviderConfig {
    client_id: String,
    client_secret: String,
    discovery_url: Url,
    scopes: Vec<String>,
}

impl Default for CustomOAuthProviderConfig {
    fn default() -> Self {
        Self {
            client_id: "Password1".to_string(),
            client_secret: "Password1".to_string(),
            discovery_url: Url::parse("https://example.com/.well-known/openid-configuration")
                .unwrap(),
            scopes: vec![
                "openid".to_string(),
                "profile".to_string(),
                "email".to_string(),
            ],
        }
    }
}
