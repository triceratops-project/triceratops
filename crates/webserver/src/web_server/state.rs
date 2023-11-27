use mobc::Pool;
use mobc_redis::{redis, RedisConnectionManager};
use oauth2::{
    basic::{BasicClient, BasicErrorResponseType, BasicTokenType},
    AuthUrl, Client, ClientId, ClientSecret, EmptyExtraTokenFields, RedirectUrl,
    RevocationErrorResponseType, RevocationUrl, StandardErrorResponse, StandardRevocableToken,
    StandardTokenIntrospectionResponse, StandardTokenResponse, TokenUrl,
};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::{sync::Arc, time::Duration};
use url::Url;

pub struct InternalAppState {
    pool: DatabaseConnection,
    redis: Pool<RedisConnectionManager>,
    oauth: InternalOAuthProviders,
}

pub struct InternalOAuthProviders {
    // pub whmcs:
    discord: OAuthClient,
}

pub type OAuthClient = Client<
    StandardErrorResponse<BasicErrorResponseType>,
    StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>,
    BasicTokenType,
    StandardTokenIntrospectionResponse<EmptyExtraTokenFields, BasicTokenType>,
    StandardRevocableToken,
    StandardErrorResponse<RevocationErrorResponseType>,
>;
pub type AppState = Arc<InternalAppState>;

impl InternalAppState {
    pub async fn new() -> Self {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let mut opt = ConnectOptions::new(database_url);
        opt.max_connections(100)
            .min_connections(3)
            .connect_timeout(Duration::from_secs(3))
            .acquire_timeout(Duration::from_secs(3))
            .idle_timeout(Duration::from_secs(8))
            .max_lifetime(Duration::from_secs(120));

        let pool = Database::connect(opt)
            .await
            .expect("Failed to connect to database");

        let redis_url = std::env::var("REDIS_URL").expect("REDIS_URL must be set");

        let redis_connection = redis::Client::open(redis_url).expect("Failed to connect to redis");

        let redis_manager = RedisConnectionManager::new(redis_connection);

        let redis = Pool::builder()
            .get_timeout(Some(Duration::from_secs(3)))
            .max_lifetime(Some(Duration::from_secs(120)))
            .max_open(100)
            .max_idle(3)
            .build(redis_manager);

        let discord_oauth = BasicClient::new(
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
            Url::parse("http://localhost:8080/auth/oauth/discord/callback").unwrap(),
        ))
        .set_revocation_uri(RevocationUrl::from_url(
            Url::parse("https://discord.com/api/oauth2/token/revoke").unwrap(),
        ));

        let oauth = InternalOAuthProviders {
            discord: discord_oauth,
        };

        Self { pool, redis, oauth }
    }

    pub fn get_pool(&self) -> &DatabaseConnection {
        &self.pool
    }

    pub fn get_redis(&self) -> &Pool<RedisConnectionManager> {
        &self.redis
    }

    pub fn get_oauth(&self) -> &InternalOAuthProviders {
        &self.oauth
    }
}

impl InternalOAuthProviders {
    pub fn discord(&self) -> &OAuthClient {
        &self.discord
    }
}
