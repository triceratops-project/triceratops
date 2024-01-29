use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct OktaOAuthProviderConfig {
    client_id: String,
    client_secret: String,
    #[serde(rename = "sso-url")]
    sso_url: Url,
}
