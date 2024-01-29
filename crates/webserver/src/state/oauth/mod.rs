use crate::config::OAuthProvidersConfig;
use error_stack::{Context, Result, ResultExt};
use oauth2::{
    basic::{BasicErrorResponseType, BasicTokenType},
    Client, EmptyExtraTokenFields, RevocationErrorResponseType, StandardErrorResponse,
    StandardRevocableToken, StandardTokenIntrospectionResponse, StandardTokenResponse,
};
use std::fmt;
use url::Url;

pub use discord::Discord;
pub use microsoft::Microsoft;

mod discord;
mod microsoft;

pub type OAuthClient = Client<
    StandardErrorResponse<BasicErrorResponseType>,
    StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>,
    BasicTokenType,
    StandardTokenIntrospectionResponse<EmptyExtraTokenFields, BasicTokenType>,
    StandardRevocableToken,
    StandardErrorResponse<RevocationErrorResponseType>,
>;

#[derive(Debug)]
pub struct OAuthError;

impl fmt::Display for OAuthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Error loading OAuth module")
    }
}

impl Context for OAuthError {}

#[derive(Debug, Clone)]
pub struct OAuthProviders {
    // custom: Option<OAuthClient>,
    discord: Option<OAuthClient>,
    // google: Option<OAuthClient>,
    microsoft: Option<OAuthClient>,
    // okta: Option<OAuthClient>,
    // whmcs: Option<OAuthClient>,
}

impl OAuthProviders {
    pub async fn new(config: &Option<OAuthProvidersConfig>, base_url: &Url) -> Result<Self, OAuthError> {
        let Some(config) = config else {
            return Ok(Self {
                discord: None,
                microsoft: None,
            });
        };

        let discord = match config.discord() {
            Some(discord_config) => {
                Some(Discord::new(discord_config, base_url).change_context(OAuthError)?)
            }
            None => None,
        };

        let microsoft = match config.microsoft() {
            Some(microsoft_config) => Some(
                Microsoft::new(microsoft_config, base_url)
                    .await
                    .change_context(OAuthError)?,
            ),
            None => None,
        };

        Ok(Self { discord, microsoft })
    }

    pub fn discord(&self) -> &Option<OAuthClient> {
        &self.discord
    }

    pub fn microsoft(&self) -> &Option<OAuthClient> {
        &self.microsoft
    }
}
