use crate::config::oauth::MicrosoftOAuthProviderConfig;
use crate::utils::oauth;
use error_stack::{Context, Report, Result, ResultExt};
use oauth2::{basic::BasicClient, AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};
use std::fmt;
use url::Url;

const MICROSOFT_BASE_URL: &'static str = "http://login.microsoftonline.com";

#[derive(Debug)]
pub struct MicrosoftOAuthError;

impl fmt::Display for MicrosoftOAuthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Error loading Microsoft OAuth Module")
    }
}

impl Context for MicrosoftOAuthError {}

pub struct Microsoft;

impl Microsoft {
    pub async fn new(
        config: &MicrosoftOAuthProviderConfig,
        base_url: &Url,
    ) -> Result<super::OAuthClient, MicrosoftOAuthError> {
        let oidc_url_string = format!(
            "{}/{}/.well-known/openid-configuration",
            MICROSOFT_BASE_URL,
            config.tenant_id()
        );

        let oidc_url = Url::parse(&oidc_url_string.as_str())
            .map_err(Report::from)
            .attach_printable("Failed to parse oidc-configuration url")
            .change_context(MicrosoftOAuthError)?;

        let oauth_metadata = oauth::get_data(oidc_url)
            .await
            .change_context(MicrosoftOAuthError)?;

        let mut redirect_url = base_url.clone();
        redirect_url.set_path("/login/oauth/microsoft/callback");

        let auth_url = AuthUrl::new(oauth_metadata.authorization_endpoint.to_string())
            .map_err(Report::from)
            .attach_printable("Failed to parse AuthUrl")
            .change_context(MicrosoftOAuthError)?;

        let token_url = TokenUrl::new(oauth_metadata.token_endpoint.to_string())
            .map_err(Report::from)
            .attach_printable("Failed to parse TokenUrl")
            .change_context(MicrosoftOAuthError)?;

        let client = BasicClient::new(
            ClientId::new(config.client_id().to_owned()),
            Some(ClientSecret::new(config.client_secret().to_owned())),
            auth_url,
            Some(token_url),
        )
        .set_redirect_uri(RedirectUrl::from_url(redirect_url));

        Ok(client)
    }
}
