use oauth2::{RevocationUrl, ClientId, basic::BasicClient, ClientSecret, AuthUrl, TokenUrl, RedirectUrl};
use url::Url;

pub struct Discord;

impl Discord {
    pub fn new() -> super::OAuthClient {
        BasicClient::new(
            ClientId::new("1022199052827381780".to_string()),
            Some(ClientSecret::new(
                "QIRiwF46OiUEfAkd14oJzKSRx8fdTipB".to_string(),
            )),
            AuthUrl::new("https://discord.com/api/oauth2/authorize".to_string())
                .expect("Failed to parse discord auth url"),
            Some(
                TokenUrl::new("https://discord.com/api/oauth2/token".to_string())
                    .expect("Failed to parse discord token url"),
            ),
        )
        .set_redirect_uri(RedirectUrl::from_url(
            Url::parse("http://localhost:8080/api/auth/oauth/discord/callback").unwrap(),
        ))
        .set_revocation_uri(RevocationUrl::from_url(
            Url::parse("https://discord.com/api/oauth2/token/revoke").unwrap(),
        ))
    }
}