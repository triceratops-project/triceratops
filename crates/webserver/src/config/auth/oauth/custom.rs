use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename = "custom")]
pub struct CustomOAuthProviderConfig {
    client_id: String,
    client_secret: String,
    discovery_url: Url,
    scopes: Vec<String>
}
