use std::net::{IpAddr, Ipv4Addr};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename = "redis")]
pub struct RedisConfig {
    ip: IpAddr,
    port: u16,
    password: Option<String>,
}

impl Default for RedisConfig {
    fn default() -> Self {
        let ip = IpAddr::from(Ipv4Addr::new(127, 0, 0, 1));
        
        Self {
            ip,
            port: 6379,
            password: None
        }
    }
}