pub use discord::Discord;
use error_stack::{Context, ResultExt, Result};
use oauth2::{
    basic::{BasicErrorResponseType, BasicTokenType},
    Client, EmptyExtraTokenFields, RevocationErrorResponseType, StandardErrorResponse,
    StandardRevocableToken, StandardTokenIntrospectionResponse, StandardTokenResponse,
};
use std::fmt::Display;

mod discord;

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

impl Display for OAuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Error loading OAuth module")
    }
}

impl Context for OAuthError {}

#[derive(Debug, Clone)]
pub struct OAuthProviders {
    discord: Option<OAuthClient>,
}

impl OAuthProviders {
    pub fn new() -> Result<Self, OAuthError> {
        let discord = Discord::new().change_context(OAuthError)?;

        Ok(Self { discord: discord })
    }

    pub fn discord(&self) -> Option<&OAuthClient> {
        self.discord.as_ref()
    }
}
