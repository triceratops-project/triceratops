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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OAuthProvidersConfig {
    custom: Option<CustomOAuthProviderConfig>,
    discord: Option<DiscordOAuthProviderConfig>,
    google: Option<GoogleOAuthProviderConfig>,
    microsoft: Option<MicrosoftOAuthProviderConfig>,
    okta: Option<OktaOAuthProviderConfig>,
    whmcs: Option<WhmcsOAuthProviderConfig>,
}
