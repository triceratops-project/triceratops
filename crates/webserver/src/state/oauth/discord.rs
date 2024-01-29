use crate::config::oauth::DiscordOAuthProviderConfig;
use error_stack::{Context, Report, Result, ResultExt};
use oauth2::{
    basic::BasicClient, AuthUrl, ClientId, ClientSecret, RedirectUrl, RevocationUrl, TokenUrl,
};
use std::fmt;
use url::Url;

#[derive(Debug)]
pub struct DiscordOAuthError;

impl fmt::Display for DiscordOAuthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Error loading Discord OAuth Module")
    }
}

impl Context for DiscordOAuthError {}

pub struct Discord;

impl Discord {
    pub fn new(
        config: &DiscordOAuthProviderConfig,
        base_url: &Url,
    ) -> Result<super::OAuthClient, DiscordOAuthError> {
        let mut redirect_url = base_url.clone();
        redirect_url.set_path("/login/oauth/discord/callback");

        let auth_url = AuthUrl::new("https://discord.com/api/oauth2/authorize".to_string())
            .map_err(Report::from)
            .attach_printable("Failed to parse AuthUrl")
            .change_context(DiscordOAuthError)?;

        let token_url = TokenUrl::new("https://discord.com/api/oauth2/token".to_string())
            .map_err(Report::from)
            .attach_printable("Failed to parse TokenUrl")
            .change_context(DiscordOAuthError)?;

        let revocation_url = Url::parse("https://discord.com/api/oauth2/token/revoke")
            .map_err(Report::from)
            .attach_printable("Failed to parse RevocationUrl")
            .change_context(DiscordOAuthError)?;

        let client = BasicClient::new(
            ClientId::new(config.client_id().to_owned()),
            Some(ClientSecret::new(config.client_secret().to_owned())),
            auth_url,
            Some(token_url),
        )
        .set_redirect_uri(RedirectUrl::from_url(redirect_url))
        .set_revocation_uri(RevocationUrl::from_url(revocation_url));

        Ok(client)
    }
}
