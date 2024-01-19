use deadpool_redis::{Config as RedisConfig, Pool as RedisPool, Runtime};

pub struct Cache;

impl Cache {
    pub async fn new() -> RedisPool {
        let redis_url = std::env::var("REDIS_URL").expect("REDIS_URL must be set");

        let redis_config = RedisConfig::from_url(redis_url);

        redis_config.create_pool(Some(Runtime::Tokio1)).unwrap()

        // We still need this config somehow
        
        // let redis = Pool::builder()
        //     .get_timeout(Some(Duration::from_secs(3)))
        //     .max_lifetime(Some(Duration::from_secs(120)))
        //     .max_open(100)
        //     .max_idle(3)
        //     .build(redis_manager);

    }
}