use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct SamlConfig {
    sso_url: Url,
    issuer: String,
    signing_certificate: String,
}
