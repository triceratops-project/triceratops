pub use error::OpenIdConfigError;
use error_stack::{Report, Result, ResultExt};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use url::Url;

mod error;

#[derive(Debug, Deserialize, Serialize)]
pub struct OidcConfig {
    pub authorization_endpoint: Url,
    pub token_endpoint: Url,
    pub userinfo_endpoint: Url,
    pub scopes_supported: Vec<String>,
}

pub async fn get_data(oidc_cfg: Url) -> Result<OidcConfig, OpenIdConfigError> {
    let domain = oidc_cfg
        .domain()
        .ok_or(Report::new(OpenIdConfigError))
        .attach_printable("Url is invalid")?;

    let oidc_config_req = reqwest::get(oidc_cfg.clone())
        .await
        .map_err(Report::from)
        .attach_printable_lazy(|| format!("Failed to fetch openid-configuration from: {}", domain))
        .change_context(OpenIdConfigError)?;

    if oidc_config_req.status() != StatusCode::OK {
        return Err(Report::new(OpenIdConfigError)).attach_printable_lazy(|| {
            format!(
                "Request failed with status code: {}",
                oidc_config_req.status()
            )
        });
    };

    let oidc_config = oidc_config_req
        .json::<OidcConfig>()
        .await
        .map_err(Report::from)
        .attach_printable("Failed to parse openid-configuration")
        .change_context(OpenIdConfigError)?;

    Ok(oidc_config)
}
