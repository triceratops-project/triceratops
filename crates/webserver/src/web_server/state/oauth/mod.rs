use oauth2::{
    basic::{BasicErrorResponseType, BasicTokenType}, Client, EmptyExtraTokenFields,
    RevocationErrorResponseType, StandardErrorResponse, StandardRevocableToken,
    StandardTokenIntrospectionResponse, StandardTokenResponse,
};

mod discord;
pub use discord::Discord;

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
    pub fn new() -> Self {
        OAuthProviders {
            discord: Discord::new(),
        }
    }

    pub fn discord(&self) -> &OAuthClient {
        &self.discord
    }
}
