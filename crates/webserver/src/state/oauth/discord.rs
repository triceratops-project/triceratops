use std::fmt::Display;

use error_stack::{Context, Report, Result, ResultExt};
use oauth2::{
    basic::BasicClient, AuthUrl, ClientId, ClientSecret, RedirectUrl, RevocationUrl, TokenUrl,
};
use url::Url;

#[derive(Debug)]
pub struct DiscordOAuthError;

impl Display for DiscordOAuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Error loading Discord OAuth Module")
    }
}

impl Context for DiscordOAuthError {}

pub struct Discord;

impl Discord {
    pub fn new() -> Result<Option<super::OAuthClient>, DiscordOAuthError> {
        let auth_url = AuthUrl::new("https://discord.com/api/oauth2/authorize".to_string())
            .map_err(Report::from)
            .attach_printable("Failed to parse AuthUrl")
            .change_context(DiscordOAuthError)?;

        let token_url = TokenUrl::new("https://discord.com/api/oauth2/token".to_string())
            .map_err(Report::from)
            .attach_printable("Failed to parse TokenUrl")
            .change_context(DiscordOAuthError)?;

        let redirect_url = Url::parse("http://localhost:8080/login/oauth/discord/callback")
            .map_err(Report::from)
            .attach_printable("Failed to parse Discord RedirectUrl")
            .change_context(DiscordOAuthError)?;

        let revocation_url = Url::parse("https://discord.com/api/oauth2/token/revoke")
            .map_err(Report::from)
            .attach_printable("Failed to parse RevocationUrl")
            .change_context(DiscordOAuthError)?;

        let client = BasicClient::new(
            ClientId::new("1022199052827381780".to_string()),
            Some(ClientSecret::new(
                "QIRiwF46OiUEfAkd14oJzKSRx8fdTipB".to_string(),
            )),
            auth_url,
            Some(token_url),
        )
        .set_redirect_uri(RedirectUrl::from_url(redirect_url))
        .set_revocation_uri(RevocationUrl::from_url(revocation_url));

        Ok(Some(client))
    }
}
