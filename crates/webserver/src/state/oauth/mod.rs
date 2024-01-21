pub use discord::Discord;
use oauth2::{
    basic::{BasicErrorResponseType, BasicTokenType},
    Client, EmptyExtraTokenFields, RevocationErrorResponseType, StandardErrorResponse,
    StandardRevocableToken, StandardTokenIntrospectionResponse, StandardTokenResponse,
};

mod discord;

pub type OAuthClient = Client<
    StandardErrorResponse<BasicErrorResponseType>,
    StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>,
    BasicTokenType,
    StandardTokenIntrospectionResponse<EmptyExtraTokenFields, BasicTokenType>,
    StandardRevocableToken,
    StandardErrorResponse<RevocationErrorResponseType>,
>;

pub struct OAuthProviders {
    discord: OAuthClient,
}

impl OAuthProviders {
    pub fn discord(&self) -> &OAuthClient {
        &self.discord
    }
}

impl Default for OAuthProviders {
    fn default() -> Self {
        Self {
            discord: Discord::new(),
        }
    }
}
