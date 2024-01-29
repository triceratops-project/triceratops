pub use custom::CustomOAuthProviderConfig;
pub use discord::DiscordOAuthProviderConfig;
pub use google::GoogleOAuthProviderConfig;
pub use microsoft::MicrosoftOAuthProviderConfig;
pub use okta::OktaOAuthProviderConfig;
use serde::{Deserialize, Serialize};
pub use whmcs::WhmcsOAuthProviderConfig;

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

impl OAuthProvidersConfig {
    pub fn custom(&self) -> &Option<CustomOAuthProviderConfig> {
        &self.custom
    }

    pub fn discord(&self) -> &Option<DiscordOAuthProviderConfig> {
        &self.discord
    }

    pub fn google(&self) -> &Option<GoogleOAuthProviderConfig> {
        &self.google
    }

    pub fn microsoft(&self) -> &Option<MicrosoftOAuthProviderConfig> {
        &self.microsoft
    }

    pub fn okta(&self) -> &Option<OktaOAuthProviderConfig> {
        &self.okta
    }

    pub fn whmcs(&self) -> &Option<WhmcsOAuthProviderConfig> {
        &self.whmcs
    }
}
