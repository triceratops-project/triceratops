use custom::CustomOAuthProviderConfig;
use discord::DiscordOAuthProviderConfig;
use google::GoogleOAuthProviderConfig;
use microsoft::MicrosoftOAuthProviderConfig;
use okta::OktaOAuthProviderConfig;
use serde::{Deserialize, Serialize};
use whmcs::WhmcsOAuthProviderConfig;

mod custom;
mod discord;
mod google;
mod microsoft;
mod okta;
mod whmcs;

#[derive(Debug, Deserialize, Serialize)]
pub struct OAuthProvidersConfig {
    custom: Option<CustomOAuthProviderConfig>,
    discord: Option<DiscordOAuthProviderConfig>,
    google: Option<GoogleOAuthProviderConfig>,
    microsoft: Option<MicrosoftOAuthProviderConfig>,
    okta: Option<OktaOAuthProviderConfig>,
    whmcs: Option<WhmcsOAuthProviderConfig>,
}

impl Default for OAuthProvidersConfig {
    fn default() -> Self {
        Self {
            custom: Some(CustomOAuthProviderConfig::default()),
            discord: Some(DiscordOAuthProviderConfig::default()),
            google: Default::default(),
            microsoft: Some(MicrosoftOAuthProviderConfig::default()),
            okta: Default::default(),
            whmcs: Default::default(),
        }
    }
}
